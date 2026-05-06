//! Retry utilities.
//!
//! [`OpenAIRetryLayer`] is Tower layer and [`SimpleRetryPolicy`] is Tower retry policy.
//!
//! Both retires on 429, 5xx and connection errors.
//!
//! The differnce is that upon seeing 429, [OpenAIRetryLayer] consumes response body to check if it is rate
//! limit (retryable error) or insufficient quota (permanent error).
//!
//!
//! [SimpleRetryPolicy] uses [`should_retry`] to determine if the request should be retried.
//!
//! The retry boundary is [`crate::middleware::HttpRequestFactory`]. Retrying
//! clones the factory and rebuilds a fresh `reqwest::Request` for each attempt
//! instead of cloning a built request. That matters because `reqwest::Request` is not Clone.
//!
//! Custom tower retry policies can call [`should_retry`] to reuse the same
//! retry classification while changing delay behavior.

mod openai;

pub use openai::{OpenAIRetry, OpenAIRetryLayer};

use std::{future::Future, pin::Pin};

use reqwest::{header::HeaderMap, Response};

use crate::{error::OpenAIError, executor::HttpRequestFactory};

/// Header containing the maximum request count permitted before rate-limit exhaustion.
pub const X_RATELIMIT_LIMIT_REQUESTS: &str = "x-ratelimit-limit-requests";
/// Header containing the maximum token count permitted before rate-limit exhaustion.
pub const X_RATELIMIT_LIMIT_TOKENS: &str = "x-ratelimit-limit-tokens";
/// Header containing the remaining request count before rate-limit exhaustion.
pub const X_RATELIMIT_REMAINING_REQUESTS: &str = "x-ratelimit-remaining-requests";
/// Header containing the remaining token count before rate-limit exhaustion.
pub const X_RATELIMIT_REMAINING_TOKENS: &str = "x-ratelimit-remaining-tokens";
/// Header containing the duration until the request-count rate limit resets.
pub const X_RATELIMIT_RESET_REQUESTS: &str = "x-ratelimit-reset-requests";
/// Header containing the duration until the token-count rate limit resets.
pub const X_RATELIMIT_RESET_TOKENS: &str = "x-ratelimit-reset-tokens";

const RATE_LIMIT_HEADERS: [&str; 6] = [
    X_RATELIMIT_LIMIT_REQUESTS,
    X_RATELIMIT_LIMIT_TOKENS,
    X_RATELIMIT_REMAINING_REQUESTS,
    X_RATELIMIT_REMAINING_TOKENS,
    X_RATELIMIT_RESET_REQUESTS,
    X_RATELIMIT_RESET_TOKENS,
];

fn log_rate_limit_headers(headers: &HeaderMap) {
    for header in RATE_LIMIT_HEADERS {
        if let Some(value) = headers.get(header).and_then(|value| value.to_str().ok()) {
            tracing::warn!("rate-limit: {header} = {value}");
        }
    }
    // Also log the Retry-After header if present
    if let Some(value) = headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
    {
        tracing::warn!("retry-after={value}");
    }
}

/// Return whether [SimpleRetryPolicy] should retry this result.
///
/// It retries only:
///
/// - HTTP `429 Too Many Requests`, because the server explicitly rate limited
///   the request.
/// - HTTP `5xx` server errors, because the server did not successfully process
///   the request.
/// - Native reqwest connect errors
#[allow(unused_variables)]
pub fn should_retry(result: &Result<Response, OpenAIError>) -> bool {
    match result {
        Ok(response) => response.status().as_u16() == 429 || response.status().is_server_error(),
        #[cfg(not(target_family = "wasm"))]
        Err(OpenAIError::Reqwest(error)) => error.is_connect(),
        #[cfg(target_family = "wasm")]
        Err(OpenAIError::Reqwest(_)) => false,
        _ => false,
    }
}

/// Simple [`tower::retry::Policy`] for OpenAI compatible APIs.
///
/// `SimpleRetryPolicy` retries rate limits, server errors, and native connect
/// errors. It can be used directly with [`tower::ServiceBuilder::retry`]
/// around [`crate::middleware::ReqwestService`] or any compatible tower service
/// whose request type is [`crate::middleware::HttpRequestFactory`].
///
/// The default policy allows three retry attempts.
#[derive(Clone, Debug)]
pub struct SimpleRetryPolicy {
    max_retries: usize,
    attempts: usize,
    backoff_attempt: u32,
}

impl SimpleRetryPolicy {
    /// Create a policy that allows at most `max_retries` retry attempts.
    ///
    /// This value is the number of additional attempts after the initial
    /// request, not the total number of requests.
    pub fn new(max_retries: usize) -> Self {
        Self {
            max_retries,
            attempts: 0,
            backoff_attempt: 0,
        }
    }

    /// Number of retry attempts configured for this policy.
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }

    /// Number of retry attempts already consumed by this policy instance.
    pub fn attempts(&self) -> usize {
        self.attempts
    }
}

impl Default for SimpleRetryPolicy {
    fn default() -> Self {
        Self::new(3)
    }
}

impl tower::retry::Policy<HttpRequestFactory, Response, OpenAIError> for SimpleRetryPolicy {
    #[cfg(not(target_family = "wasm"))]
    type Future = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
    #[cfg(target_family = "wasm")]
    type Future = Pin<Box<dyn Future<Output = ()> + 'static>>;

    fn retry(
        &mut self,
        _req: &mut HttpRequestFactory,
        result: &mut Result<Response, OpenAIError>,
    ) -> Option<Self::Future> {
        if self.attempts >= self.max_retries || !should_retry(result) {
            return None;
        }

        if let Ok(response) = result.as_ref() {
            log_rate_limit_headers(response.headers());
        }

        let retry_after = result
            .as_ref()
            .ok()
            .and_then(|response| response.headers().get(reqwest::header::RETRY_AFTER))
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(std::time::Duration::from_secs);

        let delay = retry_after.unwrap_or_else(|| {
            let delay = std::time::Duration::from_millis(100)
                .saturating_mul(2_u32.saturating_pow(self.backoff_attempt));
            self.backoff_attempt = self.backoff_attempt.saturating_add(1);
            delay.min(std::time::Duration::from_secs(8))
        });

        self.attempts += 1;

        #[cfg(target_family = "wasm")]
        {
            let _ = delay;
            return Some(Box::pin(std::future::ready(())));
        }

        #[cfg(not(target_family = "wasm"))]
        Some(Box::pin(tokio::time::sleep(delay)))
    }

    fn clone_request(&mut self, req: &HttpRequestFactory) -> Option<HttpRequestFactory> {
        Some(req.clone())
    }
}
