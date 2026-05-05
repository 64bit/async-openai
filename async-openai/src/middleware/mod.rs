//! Tower-compatible middleware support for async-openai HTTP requests.
//!
//! Enable the `middleware` feature to customize the HTTP execution path with
//! [`tower`] services and layers:
//!
//! ```toml
//! async-openai = { version = "...", features = ["middleware", "responses"] }
//! ```
//!
//! The middleware boundary is intentionally below the API groups and above the
//! concrete HTTP transport:
//!
//! ```text
//! async-openai API groups
//!     responses(), chat(), files(), ...
//!             |
//!             v
//! HttpRequestFactory
//!             |
//!             v
//! user tower layers and services
//!             |
//!             v
//! ReqwestService or custom transport
//!             |
//!             v
//! reqwest::Response
//! ```
//!
//! The request value passed through tower is [`HttpRequestFactory`], not
//! `reqwest::Request`. This is deliberate: `reqwest::Request` is not generally
//! cloneable once it contains a streaming body, but retry middleware needs a
//! way to replay a request. The factory is cheap to clone and rebuilds a fresh
//! `reqwest::Request` for each attempt.
//!
//! ## Use the default reqwest transport
//!
//! [`ReqwestService`] is the default tower-compatible service backed by
//! `reqwest::Client`. Start with it when you want tower layers but still want
//! async-openai to use reqwest for the actual network I/O:
//!
//! ```no_run
//! # use async_openai::{Client, config::OpenAIConfig};
//! # use async_openai::middleware::{HttpRetryPolicy, ReqwestService};
//! # use std::time::Duration;
//! let service = tower::ServiceBuilder::new()
//!     .concurrency_limit(8)
//!     .timeout(Duration::from_secs(30))
//!     .retry(HttpRetryPolicy::default())
//!     .service(ReqwestService::new(reqwest::Client::new()));
//!
//! let client = Client::with_config(OpenAIConfig::default())
//!     .with_http_service(service);
//! ```
//!
//! ## Use a custom service
//!
//! You can replace [`ReqwestService`] entirely. This is useful for logging,
//! metrics, tests, mocks, alternate transports, or policy layers that want to
//! inspect the generated request before sending it.
//!
//! ```no_run
//! # use async_openai::{Client, config::OpenAIConfig, error::OpenAIError};
//! # use async_openai::middleware::HttpRequestFactory;
//! # use tower::service_fn;
//! let service = service_fn(|factory: HttpRequestFactory| async move {
//!     let request = factory.build().await?;
//!     println!("sending {} {}", request.method(), request.url());
//!
//!     reqwest::Client::new()
//!         .execute(request)
//!         .await
//!         .map_err(OpenAIError::Reqwest)
//! });
//!
//! let client = Client::with_config(OpenAIConfig::default())
//!     .with_http_service(service);
//! ```
//!
//! ## Retry policy
//!
//! [`HttpRetryPolicy`] is public so it can be composed anywhere in your tower
//! stack. The default async-openai client uses this policy internally to
//! preserve the library's default retry behavior. When you provide your own
//! service with `Client::with_http_service`, place [`HttpRetryPolicy`] wherever
//! it makes sense for your stack.
//!
//! The policy retries:
//!
//! - HTTP `429` responses.
//! - HTTP `5xx` responses.
//! - retryable reqwest transport errors.
//! - boxed middleware errors.
//!
//! On native targets the policy waits using `tokio::time::sleep` with a simple
//! exponential backoff. On `wasm32` there is no universal timer runtime, so the
//! policy retries immediately. If you need delayed wasm backoff, compose a
//! wasm-runtime-compatible tower layer in your own service stack.
//!
//! ## Native and wasm bounds
//!
//! Native reqwest futures are `Send`, so native middleware services installed
//! with `Client::with_http_service` must be `Send + Sync + 'static` and return
//! `Send + 'static` futures.
//!
//! In browsers, reqwest is backed by JavaScript promises and those futures are
//! not `Send`. On wasm, async-openai relaxes the middleware service and future
//! bounds accordingly. The conceptual middleware boundary stays the same; only
//! the platform thread-safety bounds differ.
//!
//! ## Bring-your-own-types interaction
//!
//! With the `byot` feature, generated `*_byot` methods keep their existing
//! behavior for normal JSON requests. For replayable multipart/form requests
//! under `middleware`, the generated methods add an internal hidden bound so
//! the input can be stored long enough to rebuild a fresh form for retries.
//! This preserves streaming multipart behavior instead of buffering entire
//! forms into memory.

pub use crate::executor::{
    HttpExecutor, HttpRequestFactory, HttpRetryPolicy, MiddlewareInput, ReqwestService,
};
