use std::time::Duration;

use reqwest::StatusCode;
use reqwest_eventsource::retry::RetryPolicy;

/// Wraps `backoff::ExponentialBackoff` to provide a custom backoff suitable for
/// reqwest_eventsource
pub struct StreamingBackoff(backoff::ExponentialBackoff);

impl StreamingBackoff {
    fn should_retry(&self, error: &reqwest_eventsource::Error) -> bool {
        // Errors at the connection level only
        if let reqwest_eventsource::Error::Transport(error) = error {
            // TODO: We can't inspect the response body as reading consumes it.
            // This is problematic because quota exceeded errors are also 429.
            return error
                .status()
                .as_ref()
                .is_some_and(StatusCode::is_server_error)
                || error.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS);
        }

        true
    }
}

impl From<backoff::ExponentialBackoff> for StreamingBackoff {
    fn from(backoff: backoff::ExponentialBackoff) -> Self {
        Self(backoff)
    }
}

impl RetryPolicy for StreamingBackoff {
    fn retry(
        &self,
        error: &reqwest_eventsource::Error,
        last_retry: Option<(usize, Duration)>,
    ) -> Option<Duration> {
        if !self.should_retry(error) {
            return None;
        };

        // Ignoring backoff randomization factor for simplicity
        // Basically reimplements the retry policy from eventsource
        if let Some((_retry_num, last_duration)) = last_retry {
            let duration = last_duration.mul_f64(self.0.multiplier);

            if let Some(max_duration) = self.0.max_elapsed_time {
                Some(duration.min(max_duration))
            } else {
                Some(duration)
            }
        } else {
            Some(self.0.initial_interval)
        }
    }

    fn set_reconnection_time(&mut self, duration: Duration) {
        self.0.initial_interval = duration;
        if let Some(max_elapsed_time) = self.0.max_elapsed_time {
            self.0.max_elapsed_time = Some(max_elapsed_time.max(duration))
        }
    }
}
