/// HTTP client abstraction trait for async-openai
/// This allows using any HTTP client implementation, including those with middleware
use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{Method, StatusCode, Url, header::HeaderMap};
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

/// Error type for HTTP operations
#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub status: Option<StatusCode>,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Some(status) => write!(f, "HTTP {}: {}", status, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl StdError for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        HttpError {
            message: err.to_string(),
            status: err.status(),
        }
    }
}

/// Response from HTTP client
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// Trait for HTTP clients
/// This abstraction allows using reqwest::Client, ClientWithMiddleware, or any custom implementation
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// Send an HTTP request
    async fn request(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<HttpResponse, HttpError>;
}

/// Type alias for boxed HTTP client
pub type BoxedHttpClient = Arc<dyn HttpClient>;

/// Implementation for standard reqwest::Client
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn request(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<HttpResponse, HttpError> {
        let mut request = self.request(method, url).headers(headers);
        
        if let Some(body) = body {
            request = request.body(body);
        }
        
        let response = request.send().await?;
        
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;
        
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}