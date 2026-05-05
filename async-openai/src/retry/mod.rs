//! Retry utilities.
//!
//! The retry module is available when API features are enabled. It contains a
//! small default tower retry policy, [`SimpleRetryPolicy`], and the public
//! [`should_retry`] classifier used by that policy.
//!
//! The retry boundary is [`crate::middleware::HttpRequestFactory`]. Retrying
//! clones the factory and rebuilds a fresh `reqwest::Request` for each attempt
//! instead of cloning a built request. That matters for multipart and streaming
//! bodies, because `reqwest::Request` is not generally cloneable.
//!
//! Custom tower retry policies can call [`should_retry`] to reuse the same
//! retry classification while changing delay behavior.

use std::{future::Future, pin::Pin};

use reqwest::Response;

use crate::{error::OpenAIError, executor::HttpRequestFactory};

/// Return whether async-openai's simple retry policy should retry this result.
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

/// Simple [`tower::retry::Policy`] for async-openai HTTP requests.
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

        let retry_after = result
            .as_ref()
            .ok()
            .and_then(|response| response.headers().get(reqwest::header::RETRY_AFTER))
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(std::time::Duration::from_secs);

        let delay = retry_after.unwrap_or_else(|| {
            let delay = std::time::Duration::from_millis(500)
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
