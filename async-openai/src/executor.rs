use std::{future::Future, pin::Pin, sync::Arc};

use reqwest::{Request, Response};

use crate::error::OpenAIError;

#[cfg(not(target_family = "wasm"))]
type RequestFuture = Pin<Box<dyn Future<Output = Result<Request, OpenAIError>> + Send + 'static>>;
#[cfg(target_family = "wasm")]
type RequestFuture = Pin<Box<dyn Future<Output = Result<Request, OpenAIError>> + 'static>>;

#[cfg(not(target_family = "wasm"))]
pub(crate) type HttpFuture =
    Pin<Box<dyn Future<Output = Result<Response, OpenAIError>> + Send + 'static>>;
#[cfg(target_family = "wasm")]
pub(crate) type HttpFuture = Pin<Box<dyn Future<Output = Result<Response, OpenAIError>> + 'static>>;

#[cfg(not(target_family = "wasm"))]
type RequestFn = dyn Fn() -> RequestFuture + Send + Sync + 'static;
#[cfg(target_family = "wasm")]
type RequestFn = dyn Fn() -> RequestFuture + 'static;

/// Trait for types allowed as input in `*_byot` macro methods that works with `middleware` feature.
#[cfg(all(feature = "middleware", not(target_family = "wasm")))]
pub trait MiddlewareInput: Send + Sync + 'static {}
#[cfg(all(feature = "middleware", not(target_family = "wasm")))]
impl<T> MiddlewareInput for T where T: Send + Sync + 'static {}

#[cfg(all(feature = "middleware", target_family = "wasm"))]
pub trait MiddlewareInput: 'static {}
#[cfg(all(feature = "middleware", target_family = "wasm"))]
impl<T> MiddlewareInput for T where T: 'static {}

/// Cheaply cloneable request factory used to rebuild a request on demand.
///
/// This is the key boundary for middleware support:
/// - the client captures the request inputs once
/// - tower layers may clone the factory freely
/// - retries rebuild the request instead of trying to clone an already-built
///   `reqwest::Request`
///
/// The `Arc` is intentional. `tower::retry` needs to be able to clone the
/// request handle without forcing the payload itself to be eagerly copied.
/// The factory handle is cheap to clone; the request is only rebuilt when
/// `build()` is actually called.
#[derive(Clone)]
pub struct HttpRequestFactory {
    make_request: Arc<RequestFn>,
}

impl std::fmt::Debug for HttpRequestFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequestFactory").finish_non_exhaustive()
    }
}

impl HttpRequestFactory {
    /// Create a replayable request factory from an async request builder.
    ///
    /// The closure is stored behind an `Arc` so this value stays cheap to
    /// clone when it is passed through tower layers.
    #[cfg(not(target_family = "wasm"))]
    pub fn new<F, Fut>(make_request: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Request, OpenAIError>> + Send + 'static,
    {
        Self {
            make_request: Arc::new(move || Box::pin(make_request())),
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn new<F, Fut>(make_request: F) -> Self
    where
        F: Fn() -> Fut + 'static,
        Fut: Future<Output = Result<Request, OpenAIError>> + 'static,
    {
        Self {
            make_request: Arc::new(move || Box::pin(make_request())),
        }
    }

    /// Rebuild the request for the current attempt.
    ///
    /// This is what makes retries possible for non-cloneable bodies. The
    /// request is not cloned after construction; instead, the original request
    /// inputs are replayed to produce a fresh `reqwest::Request` each time.
    pub async fn build(&self) -> Result<Request, OpenAIError> {
        (self.make_request)().await
    }
}

/// Minimal request execution interface used by `Client`.
///
/// The executor sees the replayable factory rather than a built request so it
/// can decide when to rebuild and send. That keeps the retry decision close to
/// execution and avoids forcing every call site to know whether a
/// request body is cloneable.
#[cfg(not(target_family = "wasm"))]
pub trait HttpExecutor: Send + Sync {
    fn execute(&self, request: HttpRequestFactory) -> HttpFuture;
}

#[cfg(target_family = "wasm")]
pub trait HttpExecutor {
    fn execute(&self, request: HttpRequestFactory) -> HttpFuture;
}

/// Default tower-compatible service backed directly by `reqwest::Client`.
///
/// Users can layer retry, timeout, rate limiting, tracing, or any other tower
/// middleware around this service and then install the composed service with
/// `Client::with_http_service(...)`.
#[derive(Clone, Debug, Default)]
pub struct ReqwestService {
    client: reqwest::Client,
}

impl ReqwestService {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl tower::Service<HttpRequestFactory> for ReqwestService {
    type Response = Response;
    type Error = OpenAIError;
    type Future = HttpFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: HttpRequestFactory) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move {
            // This is the plain reqwest transport path. It intentionally does
            // nothing beyond rebuilding the request and executing it.
            let request = request.build().await?;
            client.execute(request).await.map_err(OpenAIError::Reqwest)
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ReqwestExecutor {
    service: tower::retry::Retry<HttpRetryPolicy, ReqwestService>,
}

impl ReqwestExecutor {
    pub(crate) fn new(client: reqwest::Client) -> Self {
        Self {
            service: tower::ServiceBuilder::new()
                .retry(HttpRetryPolicy::default())
                .service(ReqwestService::new(client)),
        }
    }
}

impl HttpExecutor for ReqwestExecutor {
    fn execute(&self, request: HttpRequestFactory) -> HttpFuture {
        use tower::ServiceExt;

        let service = self.service.clone();
        Box::pin(async move { service.oneshot(request).await })
    }
}

#[cfg(feature = "middleware")]
pub(crate) struct TowerExecutor<S> {
    service: S,
}

#[cfg(feature = "middleware")]
impl<S> TowerExecutor<S> {
    pub(crate) fn new(service: S) -> Self {
        // The executor is just an adapter around a user-supplied tower stack.
        // All of the interesting policy decisions live in the stack itself;
        // this wrapper only keeps `Client` from becoming generic over `S`.
        Self { service }
    }
}

#[cfg(all(feature = "middleware", not(target_family = "wasm")))]
impl<S> HttpExecutor for TowerExecutor<S>
where
    S: tower::Service<HttpRequestFactory, Response = Response> + Clone + Send + Sync + 'static,
    S::Future: Send + 'static,
    S::Error: Into<OpenAIError> + Send + Sync + 'static,
{
    fn execute(&self, request: HttpRequestFactory) -> HttpFuture {
        use tower::ServiceExt;

        let service = self.service.clone();
        Box::pin(async move {
            // `oneshot` keeps the client-side executor simple: the tower stack
            // decides how to use the replayable request factory, and the client
            // does not need to manage readiness or buffering itself.
            service.oneshot(request).await.map_err(Into::into)
        })
    }
}

#[cfg(all(feature = "middleware", target_family = "wasm"))]
impl<S> HttpExecutor for TowerExecutor<S>
where
    S: tower::Service<HttpRequestFactory, Response = Response> + Clone + 'static,
    S::Future: 'static,
    S::Error: Into<OpenAIError> + 'static,
{
    fn execute(&self, request: HttpRequestFactory) -> HttpFuture {
        use tower::ServiceExt;

        let service = self.service.clone();
        Box::pin(async move { service.oneshot(request).await.map_err(Into::into) })
    }
}

#[derive(Clone, Debug)]
pub struct HttpRetryPolicy {
    retries_remaining: usize,
    attempt: u32,
}

impl HttpRetryPolicy {
    pub fn new(retries_remaining: usize) -> Self {
        // The retry policy is public so users can place it anywhere in their
        // tower builder. We still provide a sensible default here to preserve
        // the old "retry a few times" client behavior when the middleware
        // feature is enabled.
        Self {
            retries_remaining,
            attempt: 0,
        }
    }
}

impl Default for HttpRetryPolicy {
    fn default() -> Self {
        Self::new(3)
    }
}

impl tower::retry::Policy<HttpRequestFactory, Response, OpenAIError> for HttpRetryPolicy {
    #[cfg(not(target_family = "wasm"))]
    type Future = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
    #[cfg(target_family = "wasm")]
    type Future = Pin<Box<dyn Future<Output = ()> + 'static>>;

    fn retry(
        &mut self,
        _req: &mut HttpRequestFactory,
        result: &mut Result<Response, OpenAIError>,
    ) -> Option<Self::Future> {
        // Once the response body is handed back to the caller we do not try to
        // interpret it here. Retry policy stays shallow:
        // - 5xx and 429 are retried
        // - connect/timeout transport errors are retried
        // - everything else is treated as a terminal failure
        //
        // That keeps transport concerns in the middleware layer while leaving
        // API body parsing in `client.rs`.
        if self.retries_remaining == 0 {
            return None;
        }

        let should_retry = match result {
            Ok(response) => {
                let status = response.status();
                status.is_server_error() || status.as_u16() == 429
            }
            #[cfg(not(target_family = "wasm"))]
            Err(OpenAIError::Reqwest(error)) => error.is_connect() || error.is_timeout(),
            #[cfg(target_family = "wasm")]
            Err(OpenAIError::Reqwest(_)) => true,
            #[cfg(feature = "middleware")]
            Err(OpenAIError::Boxed(_)) => true,
            _ => false,
        };

        if !should_retry {
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
            // Exponential backoff is intentionally simple here: the purpose of
            // this policy is to provide a reusable default that users can also
            // compose or replace in tower. More advanced retry strategies can
            // be layered on top without changing the client.
            let delay = std::time::Duration::from_millis(500)
                .saturating_mul(2_u32.saturating_pow(self.attempt));
            self.attempt = self.attempt.saturating_add(1);
            delay.min(std::time::Duration::from_secs(8))
        });

        self.retries_remaining -= 1;

        #[cfg(target_family = "wasm")]
        {
            // wasm has no universal timer runtime. We still preserve retry
            // behavior by immediately replaying the request factory; callers
            // that need timed backoff can install a wasm-runtime-compatible
            // tower layer in their own service stack.
            let _ = delay;
            return Some(Box::pin(std::future::ready(())));
        }

        #[cfg(not(target_family = "wasm"))]
        Some(Box::pin(tokio::time::sleep(delay)))
    }

    fn clone_request(&mut self, req: &HttpRequestFactory) -> Option<HttpRequestFactory> {
        // Retry requires a fresh factory for each attempt. Cloning the factory
        // is intentionally cheap because it only clones the shared `Arc`, not
        // the underlying request payload.
        Some(req.clone())
    }
}

pub(crate) type SharedExecutor = Arc<dyn HttpExecutor>;
