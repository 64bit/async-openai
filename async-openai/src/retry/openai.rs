use std::{future::Future, pin::Pin, time::Duration};

use reqwest::{header::HeaderMap, Response};

use crate::{
    error::{OpenAIError, WrappedError},
    executor::HttpRequestFactory,
};

use super::log_rate_limit_headers;
const INSUFFICIENT_QUOTA: &str = "insufficient_quota";

#[cfg(not(target_family = "wasm"))]
type RetryFuture = Pin<Box<dyn Future<Output = Result<Response, OpenAIError>> + Send + 'static>>;
#[cfg(target_family = "wasm")]
type RetryFuture = Pin<Box<dyn Future<Output = Result<Response, OpenAIError>> + 'static>>;

/// Retries `429`, `5xx`, and native reqwest connect errors with exponential backoff.
///
/// This layer consumes response body to check if 429 is from rate limit (retryable) or insufficient quota (permanent)
/// that's why any layer above it could receive OpenAIError as a result of parsing response body.
///
/// This is why [crate::retry::SimpleRetryPolicy] is also available which doesn't consume response body.
#[derive(Clone)]
pub struct OpenAIRetryLayer {
    max_retries: usize,
}

impl std::fmt::Debug for OpenAIRetryLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIRetryLayer")
            .field("max_retries", &self.max_retries)
            .finish_non_exhaustive()
    }
}

impl OpenAIRetryLayer {
    /// Create a retry layer that allows at most `max_retries` retry attempts.
    ///
    /// This value is the number of additional attempts after the initial
    /// request, not the total number of requests.
    pub fn new(max_retries: usize) -> Self {
        Self { max_retries }
    }

    /// Number of retry attempts configured for this layer.
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }
}

impl Default for OpenAIRetryLayer {
    fn default() -> Self {
        Self::new(3)
    }
}

impl<S> tower::Layer<S> for OpenAIRetryLayer {
    type Service = OpenAIRetry<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OpenAIRetry {
            inner,
            max_retries: self.max_retries,
        }
    }
}

/// Tower service produced by [`OpenAIRetryLayer`].
#[derive(Clone)]
pub struct OpenAIRetry<S> {
    inner: S,
    max_retries: usize,
}

impl<S> std::fmt::Debug for OpenAIRetry<S>
where
    S: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIRetry")
            .field("inner", &self.inner)
            .field("max_retries", &self.max_retries)
            .finish_non_exhaustive()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<S> tower::Service<HttpRequestFactory> for OpenAIRetry<S>
where
    S: tower::Service<HttpRequestFactory, Response = Response, Error = OpenAIError>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = OpenAIError;
    type Future = RetryFuture;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: HttpRequestFactory) -> Self::Future {
        let clone = self.inner.clone();
        let mut service = std::mem::replace(&mut self.inner, clone);
        let first_attempt = service.call(request.clone());
        let max_retries = self.max_retries;

        Box::pin(async move { retry_request(service, first_attempt, request, max_retries).await })
    }
}

#[cfg(target_family = "wasm")]
impl<S> tower::Service<HttpRequestFactory> for OpenAIRetry<S>
where
    S: tower::Service<HttpRequestFactory, Response = Response, Error = OpenAIError>
        + Clone
        + 'static,
    S::Future: 'static,
{
    type Response = Response;
    type Error = OpenAIError;
    type Future = RetryFuture;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: HttpRequestFactory) -> Self::Future {
        let clone = self.inner.clone();
        let mut service = std::mem::replace(&mut self.inner, clone);
        let first_attempt = service.call(request.clone());
        let max_retries = self.max_retries;

        Box::pin(async move { retry_request(service, first_attempt, request, max_retries).await })
    }
}

async fn retry_request<S>(
    mut service: S,
    first_attempt: S::Future,
    request: HttpRequestFactory,
    max_retries: usize,
) -> Result<Response, OpenAIError>
where
    S: tower::Service<HttpRequestFactory, Response = Response, Error = OpenAIError>,
{
    use tower::ServiceExt;

    let mut attempts = 0;
    let mut backoff_attempt = 0;

    let mut result = first_attempt.await;

    loop {
        // In this match satatement return early if the error is not retryable.
        let (final_result, headers, retry_after) = match result {
            Ok(response) if response.status().as_u16() == 429 => {
                let headers = response.headers().clone();
                let retry_after = retry_after(&headers);
                let bytes = match response.bytes().await {
                    Ok(bytes) => bytes,
                    Err(error) => return Err(OpenAIError::Reqwest(error)),
                };

                let error = match serde_json::from_slice::<WrappedError>(&bytes) {
                    Ok(wrapped_error) => {
                        // 429 and insufficient_quota are treated as permanent error.
                        // https://developers.openai.com/api/docs/guides/error-codes
                        if wrapped_error.error.r#type.as_deref() == Some(INSUFFICIENT_QUOTA) {
                            return Err(OpenAIError::ApiError(wrapped_error.error));
                        }

                        OpenAIError::ApiError(wrapped_error.error)
                    }
                    Err(error) => {
                        return Err(OpenAIError::JSONDeserialize(
                            error,
                            String::from_utf8_lossy(&bytes).into_owned(),
                        ));
                    }
                };

                (Err(error), Some(headers), retry_after)
            }
            Ok(response) if response.status().is_server_error() => {
                let retry_after = retry_after(response.headers());
                (Ok(response), None, retry_after)
            }
            Ok(response) => return Ok(response),
            Err(error) if is_connection_error(&error) => (Err(error), None, None),
            Err(error) => return Err(error),
        };

        if attempts >= max_retries {
            return final_result;
        }

        if let Some(headers) = headers.as_ref() {
            log_rate_limit_headers(headers);
        }

        let delay = retry_after.unwrap_or_else(|| {
            let delay =
                Duration::from_millis(100).saturating_mul(2_u32.saturating_pow(backoff_attempt));
            backoff_attempt = backoff_attempt.saturating_add(1);
            delay.min(Duration::from_secs(8))
        });

        attempts += 1;

        // on wasm there is no standard sleep so we retry immediately
        #[cfg(not(target_family = "wasm"))]
        tokio::time::sleep(delay).await;
        #[cfg(target_family = "wasm")]
        let _ = delay;

        // The service moved into this future was already made ready before the
        // first call. For retries we must poll readiness again before each
        // additional call, matching tower::retry's service contract.
        result = service.ready().await?.call(request.clone()).await;
    }
}

fn is_connection_error(error: &OpenAIError) -> bool {
    match error {
        #[cfg(not(target_family = "wasm"))]
        OpenAIError::Reqwest(error) => error.is_connect(),
        #[cfg(target_family = "wasm")]
        OpenAIError::Reqwest(_) => false,
        _ => false,
    }
}

fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
}
