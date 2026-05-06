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
//! # use async_openai::middleware::{retry::OpenAIRetryLayer, ReqwestService};
//! # use std::time::Duration;
//! let service = tower::ServiceBuilder::new()
//!     .concurrency_limit(8)
//!     .timeout(Duration::from_secs(30))
//!     .layer(OpenAIRetryLayer::default())
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
//! ## Retry layer
//!
//!
//! [`retry::OpenAIRetryLayer`] is a Tower layer and [`retry::SimpleRetryPolicy`] is a Tower retry policy.
//!
//! Both retries with exponential backoff on `429`, `5xx` and connection errors and respects `Retry-After` header.
//!
//! The difference is that upon seeing 429, `OpenAIRetryLayer` consumes response body to check if it is a rate
//! limit (retryable error) or insufficient quota (permanent error). The default async-openai client uses this layer internally
//! for library's default retry behavior.
//!
//!
//! The retry boundary is [`HttpRequestFactory`]. Retrying
//! clones the factory and rebuilds a fresh `reqwest::Request` for each attempt
//! instead of cloning a built request. That matters because `reqwest::Request` is not Clone.
//!
//! [`retry::SimpleRetryPolicy`] uses [`retry::should_retry`] to determine if a request should be retried.
//!
//! Custom tower retry policies can call [`retry::should_retry`] to reuse the same
//! retry classification while changing delay behavior.
//!
//! On native targets the layer waits using `tokio::time::sleep` with a simple
//! exponential backoff. On `wasm` the layer retries immediately.
//!
//! ## Native and WASM bounds
//!
//! The conceptual middleware boundary stays the same; only
//! the platform thread-safety bounds differ.
//!
//! On native targets, middleware services installed
//! with `Client::with_http_service` must be `Send + Sync + 'static` and return
//! `Send + 'static` futures.
//!
//! On WASM targets, middleware services and futures must be `'static`.
//!
//! ## Bring Your Own Types (BYOT) interaction
//!
//! With the `byot` feature, generated `*_byot` methods keep minimal trait bounds.
//! When `middleware` feature is enabled, the generated methods add an internal `MiddlewareInput` bound
//! based on native or WASM targets so the input can be stored long enough to
//! rebuild a fresh request for retries.

/// Retry layers and policies for middleware.
pub mod retry {
    #[doc(inline)]
    pub use crate::retry::*;
}

pub use crate::executor::{HttpExecutor, HttpRequestFactory, MiddlewareInput, ReqwestService};
