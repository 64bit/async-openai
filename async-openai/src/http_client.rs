/// HTTP client abstraction trait for async-openai
/// This allows using any HTTP client implementation, including those with middleware
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use reqwest::{Method, StatusCode, Url, header::HeaderMap};
use std::error::Error as StdError;
use std::fmt;
use std::pin::Pin;
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

/// Multipart form data for file uploads
#[derive(Clone)]
pub struct MultipartForm {
    // Store the form as bytes after encoding
    pub boundary: String,
    pub body: Bytes,
}

impl MultipartForm {
    /// Convert a reqwest multipart form to our MultipartForm
    /// This is a temporary helper until we have a better abstraction
    pub async fn from_reqwest_form(form: reqwest::multipart::Form) -> Result<Self, HttpError> {
        use uuid::Uuid;
        
        // Generate a unique boundary
        let boundary = format!("----FormBoundary{}", Uuid::new_v4().simple());
        
        // Create a client to serialize the form
        // This is a hack but reqwest doesn't expose form serialization directly
        let client = reqwest::Client::new();
        let request = client
            .post("http://localhost/dummy") // Dummy URL, we won't send this
            .multipart(form)
            .build()
            .map_err(|e| HttpError {
                message: format!("Failed to build multipart request: {}", e),
                status: None,
            })?;
        
        // Extract the body bytes
        let body = request.body()
            .and_then(|b| b.as_bytes())
            .ok_or_else(|| HttpError {
                message: "Failed to get multipart body bytes".to_string(),
                status: None,
            })?;
        
        Ok(MultipartForm {
            boundary,
            body: Bytes::copy_from_slice(body),
        })
    }
}

/// Server-sent event for streaming
#[derive(Debug, Clone)]
pub struct SseEvent {
    pub data: String,
    pub event: Option<String>,
    pub id: Option<String>,
    pub retry: Option<u64>,
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
    
    /// Send a multipart form request
    async fn request_multipart(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        form: MultipartForm,
    ) -> Result<HttpResponse, HttpError>;
    
    /// Send a request and receive Server-Sent Events stream
    async fn request_stream(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<SseEvent, HttpError>> + Send>>, HttpError>;
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
    
    async fn request_multipart(
        &self,
        method: Method,
        url: Url,
        mut headers: HeaderMap,
        form: MultipartForm,
    ) -> Result<HttpResponse, HttpError> {
        use reqwest::header::{CONTENT_TYPE, HeaderValue};
        
        // Set the multipart boundary in content-type header
        let content_type = format!("multipart/form-data; boundary={}", form.boundary);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(&content_type).map_err(|e| HttpError {
            message: format!("Invalid content type: {}", e),
            status: None,
        })?);
        
        let request = self.request(method, url)
            .headers(headers)
            .body(form.body);
        
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
    
    async fn request_stream(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<SseEvent, HttpError>> + Send>>, HttpError> {
        use futures::StreamExt;
        use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
        
        let mut request = self.request(method, url).headers(headers);
        
        if let Some(body) = body {
            request = request.body(body);
        }
        
        let event_source = request.eventsource().map_err(|e| HttpError {
            message: format!("Failed to create event source: {}", e),
            status: None,
        })?;
        
        // Convert reqwest EventSource to our SseEvent stream
        let stream = event_source.map(move |event| {
            match event {
                Ok(Event::Message(msg)) => Ok(SseEvent {
                    data: msg.data,
                    event: Some(msg.event),
                    id: Some(msg.id),
                    retry: msg.retry.map(|d| d.as_millis() as u64),
                }),
                Ok(Event::Open) => Ok(SseEvent {
                    data: String::new(),
                    event: Some("open".to_string()),
                    id: None,
                    retry: None,
                }),
                Err(e) => Err(HttpError {
                    message: format!("Stream error: {}", e),
                    status: None,
                }),
            }
        });
        
        Ok(Box::pin(stream))
    }
}