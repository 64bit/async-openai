//! Client-side request rate limiting middleware.
//!
//! This layer combines two signals. A local governor quota limits requests
//! before they are sent, and server response headers can add shared
//! backpressure when OpenAI reports no remaining request quota. Both paths use
//! monotonic deadlines measured from layer creation so local waits and
//! server-directed waits use the same clock model.

use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use governor::clock::Clock;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use reqwest::Response;

use crate::{error::OpenAIError, executor::HttpRequestFactory};

type RateLimitFuture =
    Pin<Box<dyn Future<Output = Result<Response, OpenAIError>> + Send + 'static>>;

/// Tower layer for client-side request rate limiting.
#[derive(Clone, Debug)]
pub struct RateLimitLayer {
    /// Shared local token bucket used by all services cloned from this layer.
    limiter: Arc<DefaultDirectRateLimiter>,
    /// Shared server-directed deadline derived from rate-limit response headers.
    backpressure: Arc<ServerBackpressure>,
    /// Monotonic base used for all internal deadlines, matching governor's clock model.
    base: Instant,
}

impl RateLimitLayer {
    /// Create a layer with the provided governor quota.
    #[must_use]
    pub fn new(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            backpressure: Arc::new(ServerBackpressure::default()),
            base: Instant::now(),
        }
    }

    /// Create a layer that allows at most `rps` requests per second.
    #[must_use]
    pub fn per_second(rps: NonZeroU32) -> Self {
        Self::new(Quota::per_second(rps))
    }

    /// Create a layer that allows at most `rpm` requests per minute.
    #[must_use]
    pub fn per_minute(rpm: NonZeroU32) -> Self {
        Self::new(Quota::per_minute(rpm))
    }

    /// Create a layer that allows at most `rph` requests per hour.
    #[must_use]
    pub fn per_hour(rph: NonZeroU32) -> Self {
        Self::new(Quota::per_hour(rph))
    }
}

impl<S> tower::Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
            backpressure: self.backpressure.clone(),
            base: self.base,
            delay: None,
            delay_deadline_ms: 0,
        }
    }
}

/// Tower service produced by [`RateLimitLayer`].
pub struct RateLimitService<S> {
    inner: S,
    /// Shared local token bucket, so cloned services apply one combined quota.
    limiter: Arc<DefaultDirectRateLimiter>,
    /// Shared server backpressure, so one exhausted response pauses sibling clones.
    backpressure: Arc<ServerBackpressure>,
    /// Monotonic base inherited from the layer for layer-relative deadlines.
    base: Instant,
    /// Active sleep for either local quota exhaustion or server backpressure.
    delay: Option<Pin<Box<tokio::time::Sleep>>>,
    /// Layer-relative deadline, in milliseconds, for the active sleep.
    delay_deadline_ms: u64,
}

impl<S> Clone for RateLimitService<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            limiter: self.limiter.clone(),
            backpressure: self.backpressure.clone(),
            base: self.base,
            delay: None,
            delay_deadline_ms: 0,
        }
    }
}

impl<S> RateLimitService<S> {
    fn now_millis(&self) -> u64 {
        duration_millis(self.base.elapsed())
    }

    fn poll_delay_until(&mut self, cx: &mut Context<'_>, deadline_ms: u64) -> Poll<()> {
        if self.delay_deadline_ms != deadline_ms {
            let wait_ms = deadline_ms.saturating_sub(self.now_millis());
            self.delay = Some(Box::pin(tokio::time::sleep(Duration::from_millis(wait_ms))));
            self.delay_deadline_ms = deadline_ms;
        }

        let Some(delay) = self.delay.as_mut() else {
            return Poll::Ready(());
        };

        match delay.as_mut().poll(cx) {
            Poll::Ready(()) => {
                self.delay = None;
                self.delay_deadline_ms = 0;
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<S> tower::Service<HttpRequestFactory> for RateLimitService<S>
where
    S: tower::Service<HttpRequestFactory, Response = Response, Error = OpenAIError>
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = OpenAIError;
    type Future = RateLimitFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            if let Some(reset_at) = self.backpressure.reset_at_millis() {
                let now = self.now_millis();
                if now < reset_at {
                    if self.poll_delay_until(cx, reset_at).is_pending() {
                        return Poll::Pending;
                    }
                    continue;
                }

                if self.backpressure.clear_if_deadline(reset_at).is_ok() {
                    tracing::info!("{}", backpressure_cleared_message(reset_at));
                } else {
                    continue;
                }
            }

            match self.inner.poll_ready(cx) {
                Poll::Ready(Ok(())) => {}
                other => return other,
            }

            match self.limiter.check() {
                Ok(()) => return Poll::Ready(Ok(())),
                Err(not_until) => {
                    let wait = not_until.wait_time_from(self.limiter.clock().now());
                    let wait_ms = duration_millis(wait);
                    let deadline = self.now_millis().saturating_add(wait_ms);
                    if self.delay_deadline_ms != deadline {
                        tracing::warn!("{}", governor_exhausted_message(deadline, wait_ms));
                    }

                    if self.poll_delay_until(cx, deadline).is_pending() {
                        return Poll::Pending;
                    }
                }
            }
        }
    }

    fn call(&mut self, request: HttpRequestFactory) -> Self::Future {
        let backpressure = self.backpressure.clone();
        let base = self.base;
        let future = self.inner.call(request);

        Box::pin(async move {
            let response = future.await?;
            update_backpressure_from_headers(
                response.headers(),
                &backpressure,
                duration_millis(base.elapsed()),
            );
            Ok(response)
        })
    }
}

#[derive(Debug, Default)]
struct ServerBackpressure {
    /// Layer-relative deadline, in milliseconds, until requests may resume.
    reset_at: AtomicU64,
}

impl ServerBackpressure {
    fn reset_at_millis(&self) -> Option<u64> {
        match self.reset_at.load(Ordering::Relaxed) {
            0 => None,
            value => Some(value),
        }
    }

    fn update_deadline(&self, deadline_ms: u64) {
        self.reset_at.fetch_max(deadline_ms, Ordering::Relaxed);
    }

    fn clear_if_deadline(&self, observed_deadline_ms: u64) -> Result<(), u64> {
        self.reset_at.compare_exchange(
            observed_deadline_ms,
            0,
            Ordering::Relaxed,
            Ordering::Relaxed,
        )?;
        Ok(())
    }
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn parse_reset_duration(value: &str) -> Option<Duration> {
    let bytes = value.as_bytes();
    let mut index = 0;
    let mut total = Duration::ZERO;
    let mut saw_part = false;

    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }

        if index == bytes.len() {
            break;
        }

        let number_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }

        if number_start == index {
            return None;
        }

        let number = value[number_start..index].parse::<u64>().ok()?;

        let unit_start = index;
        while index < bytes.len() && bytes[index].is_ascii_alphabetic() {
            index += 1;
        }

        let unit = &value[unit_start..index];
        let part = match unit {
            "ms" => Duration::from_millis(number),
            "s" => Duration::from_secs(number),
            "m" => Duration::from_secs(number.checked_mul(60)?),
            "h" => Duration::from_secs(number.checked_mul(60)?.checked_mul(60)?),
            _ => return None,
        };

        total = total.checked_add(part)?;
        saw_part = true;
    }

    saw_part.then_some(total)
}

fn update_backpressure_from_headers(
    headers: &reqwest::header::HeaderMap,
    backpressure: &ServerBackpressure,
    now_ms: u64,
) {
    let Some(remaining_value) = headers.get(crate::retry::X_RATELIMIT_REMAINING_REQUESTS) else {
        return;
    };

    let Some(remaining) = remaining_value
        .to_str()
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
    else {
        return;
    };

    if remaining != 0 {
        return;
    }

    let Some(reset) = headers
        .get(crate::retry::X_RATELIMIT_RESET_REQUESTS)
        .and_then(|value| value.to_str().ok())
        .and_then(parse_reset_duration)
    else {
        return;
    };

    let reset_ms = duration_millis(reset);
    let deadline = now_ms.saturating_add(reset_ms);
    backpressure.update_deadline(deadline);
    tracing::warn!("{}", backpressure_active_message(deadline, reset_ms));
}

fn governor_exhausted_message(deadline_ms: u64, wait_ms: u64) -> String {
    format!(
        "rate limit governor exhausted until {deadline_ms} (monotonic ms since layer creation); delaying request for {wait_ms}ms"
    )
}

fn backpressure_active_message(deadline_ms: u64, reset_ms: u64) -> String {
    format!(
        "rate limit request backpressure active until {deadline_ms} (monotonic ms since layer creation); delaying new requests for {reset_ms}ms"
    )
}

fn backpressure_cleared_message(reset_at_ms: u64) -> String {
    format!(
        "rate limit request backpressure cleared at {reset_at_ms} (monotonic ms since layer creation)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Response as HttpResponse;
    use std::sync::atomic::AtomicUsize;
    use tower::{service_fn, Layer, Service, ServiceBuilder, ServiceExt};

    fn test_factory() -> HttpRequestFactory {
        HttpRequestFactory::new(|| async {
            Ok(reqwest::Request::new(
                reqwest::Method::GET,
                "http://example.test/models".parse().unwrap(),
            ))
        })
    }

    #[tokio::test]
    async fn service_applies_backpressure_before_forwarding() {
        let calls = Arc::new(AtomicUsize::new(0));
        let service = RateLimitLayer::per_minute(NonZeroU32::new(60).unwrap()).layer({
            let calls = calls.clone();
            service_fn(move |_factory: HttpRequestFactory| {
                let calls = calls.clone();
                async move {
                    calls.fetch_add(1, Ordering::Relaxed);
                    Ok::<_, OpenAIError>(
                        HttpResponse::builder()
                            .status(200)
                            .header(crate::retry::X_RATELIMIT_REMAINING_REQUESTS, "0")
                            .header(crate::retry::X_RATELIMIT_RESET_REQUESTS, "25ms")
                            .body(reqwest::Body::from("{}"))
                            .unwrap()
                            .into(),
                    )
                }
            })
        });

        let mut service = service;
        service
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        let started = std::time::Instant::now();
        service
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        assert!(started.elapsed() >= Duration::from_millis(20));
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn cloned_services_share_governor_bucket() {
        let calls = Arc::new(AtomicUsize::new(0));
        let service = RateLimitLayer::per_second(NonZeroU32::new(1).unwrap()).layer({
            let calls = calls.clone();
            service_fn(move |_factory: HttpRequestFactory| {
                let calls = calls.clone();
                async move {
                    calls.fetch_add(1, Ordering::Relaxed);
                    Ok::<_, OpenAIError>(
                        HttpResponse::builder()
                            .status(200)
                            .body(reqwest::Body::from("{}"))
                            .unwrap()
                            .into(),
                    )
                }
            })
        });

        let mut first = service.clone();
        let mut second = service;

        first
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        let started = std::time::Instant::now();
        second
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        assert!(started.elapsed() >= Duration::from_millis(900));
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn pending_inner_readiness_does_not_consume_governor_permit() {
        use futures::task::noop_waker_ref;

        #[derive(Clone)]
        struct PendingOnceService {
            pending_once: bool,
            calls: Arc<AtomicUsize>,
        }

        impl Service<HttpRequestFactory> for PendingOnceService {
            type Response = reqwest::Response;
            type Error = OpenAIError;
            type Future =
                Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

            fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                if self.pending_once {
                    self.pending_once = false;
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }

                Poll::Ready(Ok(()))
            }

            fn call(&mut self, _req: HttpRequestFactory) -> Self::Future {
                self.calls.fetch_add(1, Ordering::Relaxed);
                Box::pin(async {
                    Ok(HttpResponse::builder()
                        .status(200)
                        .body(reqwest::Body::from("{}"))
                        .unwrap()
                        .into())
                })
            }
        }

        let calls = Arc::new(AtomicUsize::new(0));
        let mut service =
            RateLimitLayer::per_second(NonZeroU32::new(1).unwrap()).layer(PendingOnceService {
                pending_once: true,
                calls: calls.clone(),
            });

        let mut cx = Context::from_waker(noop_waker_ref());
        assert!(matches!(service.poll_ready(&mut cx), Poll::Pending));

        let started = std::time::Instant::now();
        service
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        assert!(started.elapsed() < Duration::from_millis(200));
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn retry_attempts_pass_through_rate_limiter() {
        let calls = Arc::new(AtomicUsize::new(0));
        let service = ServiceBuilder::new()
            .layer(crate::retry::OpenAIRetryLayer::new(1))
            .layer(RateLimitLayer::per_second(NonZeroU32::new(1).unwrap()))
            .service({
                let calls = calls.clone();
                service_fn(move |_factory: HttpRequestFactory| {
                    let calls = calls.clone();
                    async move {
                        let attempt = calls.fetch_add(1, Ordering::Relaxed);
                        let response = if attempt == 0 {
                            HttpResponse::builder()
                                .status(429)
                                .header("content-type", "application/json")
                                .body(reqwest::Body::from(
                                    r#"{"error":{"message":"retry me","type":"rate_limit_error","param":null,"code":null}}"#,
                                ))
                                .unwrap()
                        } else {
                            HttpResponse::builder()
                                .status(200)
                                .body(reqwest::Body::from("{}"))
                                .unwrap()
                        };

                        Ok::<_, OpenAIError>(response.into())
                    }
                })
            });

        let mut service = service;
        let started = std::time::Instant::now();
        service
            .ready()
            .await
            .unwrap()
            .call(test_factory())
            .await
            .unwrap();

        assert!(started.elapsed() >= Duration::from_millis(900));
        assert_eq!(calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn log_messages_include_deadline_and_duration() {
        assert_eq!(
            governor_exhausted_message(123, 45),
            "rate limit governor exhausted until 123 (monotonic ms since layer creation); delaying request for 45ms"
        );
        assert_eq!(
            backpressure_active_message(456, 78),
            "rate limit request backpressure active until 456 (monotonic ms since layer creation); delaying new requests for 78ms"
        );
        assert_eq!(
            backpressure_cleared_message(456),
            "rate limit request backpressure cleared at 456 (monotonic ms since layer creation)"
        );
    }

    #[test]
    fn services_use_layer_relative_monotonic_clock() {
        let service = RateLimitLayer::per_second(NonZeroU32::new(1).unwrap()).layer(service_fn(
            |_factory: HttpRequestFactory| async {
                Ok::<reqwest::Response, OpenAIError>(
                    HttpResponse::builder()
                        .status(200)
                        .body(reqwest::Body::from("{}"))
                        .unwrap()
                        .into(),
                )
            },
        ));

        assert!(service.now_millis() < 1_000);
    }

    #[test]
    fn parse_reset_duration_accepts_supported_units() {
        assert_eq!(
            parse_reset_duration("250ms"),
            Some(Duration::from_millis(250))
        );
        assert_eq!(parse_reset_duration("6s"), Some(Duration::from_secs(6)));
        assert_eq!(parse_reset_duration("1m30s"), Some(Duration::from_secs(90)));
        assert_eq!(
            parse_reset_duration("2h 1m 3s"),
            Some(Duration::from_secs(7263))
        );
    }

    #[test]
    fn parse_reset_duration_rejects_invalid_values() {
        assert_eq!(parse_reset_duration(""), None);
        assert_eq!(parse_reset_duration("   "), None);
        assert_eq!(parse_reset_duration("soon"), None);
        assert_eq!(parse_reset_duration("1"), None);
        assert_eq!(parse_reset_duration("1x"), None);
        assert_eq!(parse_reset_duration("ms"), None);
        assert_eq!(parse_reset_duration("18446744073709551615s1s"), None);
        assert_eq!(parse_reset_duration("5124095576030432h"), None);
    }

    #[test]
    fn backpressure_keeps_later_deadline() {
        let backpressure = ServerBackpressure::default();

        backpressure.update_deadline(1_000);
        backpressure.update_deadline(500);
        assert_eq!(backpressure.reset_at_millis(), Some(1_000));

        backpressure.update_deadline(2_000);
        assert_eq!(backpressure.reset_at_millis(), Some(2_000));
    }

    #[test]
    fn backpressure_clear_only_removes_observed_deadline() {
        let backpressure = ServerBackpressure::default();

        backpressure.update_deadline(1_000);
        assert!(backpressure.clear_if_deadline(500).is_err());
        assert_eq!(backpressure.reset_at_millis(), Some(1_000));

        assert!(backpressure.clear_if_deadline(1_000).is_ok());
        assert_eq!(backpressure.reset_at_millis(), None);
    }

    #[test]
    fn update_backpressure_uses_reset_when_remaining_is_zero() {
        let backpressure = ServerBackpressure::default();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            crate::retry::X_RATELIMIT_REMAINING_REQUESTS,
            reqwest::header::HeaderValue::from_static("0"),
        );
        headers.insert(
            crate::retry::X_RATELIMIT_RESET_REQUESTS,
            reqwest::header::HeaderValue::from_static("250ms"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);

        assert_eq!(backpressure.reset_at_millis(), Some(1_250));
    }

    #[test]
    fn update_backpressure_ignores_nonzero_remaining() {
        let backpressure = ServerBackpressure::default();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            crate::retry::X_RATELIMIT_REMAINING_REQUESTS,
            reqwest::header::HeaderValue::from_static("1"),
        );
        headers.insert(
            crate::retry::X_RATELIMIT_RESET_REQUESTS,
            reqwest::header::HeaderValue::from_static("1s"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);

        assert_eq!(backpressure.reset_at_millis(), None);
    }

    #[test]
    fn update_backpressure_ignores_missing_or_invalid_remaining() {
        let backpressure = ServerBackpressure::default();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            crate::retry::X_RATELIMIT_RESET_REQUESTS,
            reqwest::header::HeaderValue::from_static("1s"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);
        assert_eq!(backpressure.reset_at_millis(), None);

        headers.insert(
            crate::retry::X_RATELIMIT_REMAINING_REQUESTS,
            reqwest::header::HeaderValue::from_static("not-a-number"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);
        assert_eq!(backpressure.reset_at_millis(), None);
    }

    #[test]
    fn update_backpressure_ignores_missing_or_invalid_reset() {
        let backpressure = ServerBackpressure::default();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            crate::retry::X_RATELIMIT_REMAINING_REQUESTS,
            reqwest::header::HeaderValue::from_static("0"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);
        assert_eq!(backpressure.reset_at_millis(), None);

        headers.insert(
            crate::retry::X_RATELIMIT_RESET_REQUESTS,
            reqwest::header::HeaderValue::from_static("later"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);
        assert_eq!(backpressure.reset_at_millis(), None);
    }

    #[test]
    fn update_backpressure_keeps_later_existing_deadline() {
        let backpressure = ServerBackpressure::default();
        backpressure.update_deadline(5_000);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            crate::retry::X_RATELIMIT_REMAINING_REQUESTS,
            reqwest::header::HeaderValue::from_static("0"),
        );
        headers.insert(
            crate::retry::X_RATELIMIT_RESET_REQUESTS,
            reqwest::header::HeaderValue::from_static("250ms"),
        );

        update_backpressure_from_headers(&headers, &backpressure, 1_000);

        assert_eq!(backpressure.reset_at_millis(), Some(5_000));
    }
}
