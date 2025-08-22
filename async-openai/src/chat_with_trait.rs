// Chat API implementation that works with ClientWithTrait and HttpClient trait

use crate::{
    config::Config,
    error::{ApiError, OpenAIError},
    http_client::HttpResponse,
    types::{
        CreateChatCompletionRequest, CreateChatCompletionResponse,
        CreateChatCompletionStreamResponse,
    },
    ClientWithTrait,
};
use std::pin::Pin;
use futures::Stream;
use bytes::Bytes;
use reqwest::{Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};

/// Chat API group with HttpClient trait support
pub struct ChatWithTrait<'c, C: Config> {
    client: &'c ClientWithTrait<C>,
}

impl<'c, C: Config> ChatWithTrait<'c, C> {
    pub fn new(client: &'c ClientWithTrait<C>) -> Self {
        Self { client }
    }

    /// Creates a model response for the given chat conversation.
    pub async fn create(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, OpenAIError> {
        // Prepare the request - config.url() returns the full URL
        let url_str = self.client.config.url("/chat/completions");
        let mut url = reqwest::Url::parse(&url_str)
            .map_err(|e| OpenAIError::InvalidArgument(format!("Invalid URL: {}", e)))?;
        
        // Add query parameters from config
        for (key, value) in self.client.config.query() {
            url.query_pairs_mut().append_pair(key, value);
        }
        
        // Serialize request body
        let body = serde_json::to_vec(&request)
            .map_err(|e| OpenAIError::InvalidArgument(format!("Failed to serialize request: {}", e)))?;
        
        // Prepare headers
        let mut headers = self.client.config.headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        // Execute request with backoff
        let response = self.execute_with_backoff(
            Method::POST,
            url,
            headers,
            Some(Bytes::from(body)),
        ).await?;
        
        // Parse response
        serde_json::from_slice(&response.body)
            .map_err(|e| OpenAIError::JSONDeserialize(e))
    }

    /// Creates a streaming response for the given chat conversation.
    pub async fn create_stream(
        &self,
        mut request: CreateChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>>,
        OpenAIError,
    > {
        // Set stream flag
        request.stream = Some(true);
        
        // For now, return an error as streaming requires more complex implementation
        // This would need to handle SSE (Server-Sent Events) parsing
        Err(OpenAIError::InvalidArgument(
            "Streaming not yet implemented for ChatWithTrait".to_string()
        ))
    }

    /// Execute request with exponential backoff for rate limiting
    async fn execute_with_backoff(
        &self,
        method: Method,
        url: reqwest::Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<HttpResponse, OpenAIError> {
        use backoff::{future::retry, ExponentialBackoff};
        
        let http_client = self.client.http_client.clone();
        let backoff = self.client.backoff.clone();
        
        retry(backoff, || async {
            let result = http_client
                .request(method.clone(), url.clone(), headers.clone(), body.clone())
                .await;
            
            match result {
                Ok(response) => {
                    if response.status.is_success() {
                        Ok(response)
                    } else if response.status.as_u16() == 429 {
                        // Rate limited, retry with backoff
                        Err(backoff::Error::transient(OpenAIError::ApiError(
                            ApiError {
                                message: "Rate limited".to_string(),
                                r#type: Some("rate_limit_exceeded".to_string()),
                                param: None,
                                code: None,
                            }
                        )))
                    } else {
                        // Other error, don't retry
                        let api_error = serde_json::from_slice(&response.body)
                            .unwrap_or_else(|_| ApiError {
                                message: format!("HTTP {}", response.status),
                                r#type: None,
                                param: None,
                                code: None,
                            });
                        Err(backoff::Error::permanent(OpenAIError::ApiError(api_error)))
                    }
                }
                Err(e) => {
                    // Network error, retry  
                    // Convert to a string-based error since we can't create reqwest::Error directly
                    Err(backoff::Error::transient(OpenAIError::InvalidArgument(
                        format!("HTTP client error: {}", e.message)
                    )))
                }
            }
        })
        .await
    }
}