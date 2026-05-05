use std::{future::Future, pin::Pin, sync::Arc};

use reqwest::{Request, Response};

use crate::{error::OpenAIError, retry::SimpleRetryPolicy};

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
    service: tower::retry::Retry<SimpleRetryPolicy, ReqwestService>,
}

impl ReqwestExecutor {
    pub(crate) fn new(client: reqwest::Client) -> Self {
        Self {
            service: tower::ServiceBuilder::new()
                .retry(SimpleRetryPolicy::default())
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

pub(crate) type SharedExecutor = Arc<dyn HttpExecutor>;
