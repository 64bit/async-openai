use std::pin::Pin;

use bytes::Bytes;
use futures::{stream::StreamExt, Stream};
use reqwest::{header::HeaderMap, multipart::Form, Response};
use reqwest_eventsource::{Error as EventSourceError, Event, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::{Config, OpenAIConfig},
    error::{map_deserialization_error, ApiError, OpenAIError, StreamError, WrappedError},
    traits::AsyncTryFrom,
    RequestOptions,
};

#[cfg(feature = "administration")]
use crate::admin::Admin;
#[cfg(feature = "chatkit")]
use crate::chatkit::Chatkit;
#[cfg(feature = "file")]
use crate::file::Files;
#[cfg(feature = "image")]
use crate::image::Images;
#[cfg(feature = "moderation")]
use crate::moderation::Moderations;
#[cfg(feature = "assistant")]
use crate::Assistants;
#[cfg(feature = "audio")]
use crate::Audio;
#[cfg(feature = "batch")]
use crate::Batches;
#[cfg(feature = "chat-completion")]
use crate::Chat;
#[cfg(feature = "completions")]
use crate::Completions;
#[cfg(feature = "container")]
use crate::Containers;
#[cfg(feature = "responses")]
use crate::Conversations;
#[cfg(feature = "embedding")]
use crate::Embeddings;
#[cfg(feature = "evals")]
use crate::Evals;
#[cfg(feature = "finetuning")]
use crate::FineTuning;
#[cfg(feature = "model")]
use crate::Models;
#[cfg(feature = "realtime")]
use crate::Realtime;
#[cfg(feature = "responses")]
use crate::Responses;
#[cfg(feature = "assistant")]
use crate::Threads;
#[cfg(feature = "upload")]
use crate::Uploads;
#[cfg(feature = "vectorstore")]
use crate::VectorStores;
#[cfg(feature = "video")]
use crate::Videos;

#[derive(Debug, Clone, Default)]
/// Client is a container for config, backoff and http_client
/// used to make API calls.
pub struct Client<C: Config> {
    http_client: reqwest::Client,
    config: C,
    backoff: backoff::ExponentialBackoff,
}

impl Client<OpenAIConfig> {
    /// Client with default [OpenAIConfig]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<C: Config> Client<C> {
    /// Create client with a custom HTTP client, OpenAI config, and backoff.
    pub fn build(
        http_client: reqwest::Client,
        config: C,
        backoff: backoff::ExponentialBackoff,
    ) -> Self {
        Self {
            http_client,
            config,
            backoff,
        }
    }

    /// Create client with [OpenAIConfig] or [crate::config::AzureConfig]
    pub fn with_config(config: C) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config,
            backoff: Default::default(),
        }
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    /// Exponential backoff for retrying [rate limited](https://platform.openai.com/docs/guides/rate-limits) requests.
    pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    // API groups

    /// To call [Models] group related APIs using this client.
    #[cfg(feature = "model")]
    pub fn models(&self) -> Models<'_, C> {
        Models::new(self)
    }

    /// To call [Completions] group related APIs using this client.
    #[cfg(feature = "completions")]
    pub fn completions(&self) -> Completions<'_, C> {
        Completions::new(self)
    }

    /// To call [Chat] group related APIs using this client.
    #[cfg(feature = "chat-completion")]
    pub fn chat(&self) -> Chat<'_, C> {
        Chat::new(self)
    }

    /// To call [Images] group related APIs using this client.
    #[cfg(feature = "image")]
    pub fn images(&self) -> Images<'_, C> {
        Images::new(self)
    }

    /// To call [Moderations] group related APIs using this client.
    #[cfg(feature = "moderation")]
    pub fn moderations(&self) -> Moderations<'_, C> {
        Moderations::new(self)
    }

    /// To call [Files] group related APIs using this client.
    #[cfg(feature = "file")]
    pub fn files(&self) -> Files<'_, C> {
        Files::new(self)
    }

    /// To call [Uploads] group related APIs using this client.
    #[cfg(feature = "upload")]
    pub fn uploads(&self) -> Uploads<'_, C> {
        Uploads::new(self)
    }

    /// To call [FineTuning] group related APIs using this client.
    #[cfg(feature = "finetuning")]
    pub fn fine_tuning(&self) -> FineTuning<'_, C> {
        FineTuning::new(self)
    }

    /// To call [Embeddings] group related APIs using this client.
    #[cfg(feature = "embedding")]
    pub fn embeddings(&self) -> Embeddings<'_, C> {
        Embeddings::new(self)
    }

    /// To call [Audio] group related APIs using this client.
    #[cfg(feature = "audio")]
    pub fn audio(&self) -> Audio<'_, C> {
        Audio::new(self)
    }

    /// To call [Videos] group related APIs using this client.
    #[cfg(feature = "video")]
    pub fn videos(&self) -> Videos<'_, C> {
        Videos::new(self)
    }

    /// To call [Assistants] group related APIs using this client.
    #[cfg(feature = "assistant")]
    pub fn assistants(&self) -> Assistants<'_, C> {
        Assistants::new(self)
    }

    /// To call [Threads] group related APIs using this client.
    #[cfg(feature = "assistant")]
    pub fn threads(&self) -> Threads<'_, C> {
        Threads::new(self)
    }

    /// To call [VectorStores] group related APIs using this client.
    #[cfg(feature = "vectorstore")]
    pub fn vector_stores(&self) -> VectorStores<'_, C> {
        VectorStores::new(self)
    }

    /// To call [Batches] group related APIs using this client.
    #[cfg(feature = "batch")]
    pub fn batches(&self) -> Batches<'_, C> {
        Batches::new(self)
    }

    /// To call [Admin] group related APIs using this client.
    /// This groups together admin API keys, invites, users, projects, audit logs, and certificates.
    #[cfg(feature = "administration")]
    pub fn admin(&self) -> Admin<'_, C> {
        Admin::new(self)
    }

    /// To call [Responses] group related APIs using this client.
    #[cfg(feature = "responses")]
    pub fn responses(&self) -> Responses<'_, C> {
        Responses::new(self)
    }

    /// To call [Conversations] group related APIs using this client.
    #[cfg(feature = "responses")]
    pub fn conversations(&self) -> Conversations<'_, C> {
        Conversations::new(self)
    }

    /// To call [Containers] group related APIs using this client.
    #[cfg(feature = "container")]
    pub fn containers(&self) -> Containers<'_, C> {
        Containers::new(self)
    }

    /// To call [Evals] group related APIs using this client.
    #[cfg(feature = "evals")]
    pub fn evals(&self) -> Evals<'_, C> {
        Evals::new(self)
    }

    #[cfg(feature = "chatkit")]
    pub fn chatkit(&self) -> Chatkit<'_, C> {
        Chatkit::new(self)
    }

    /// To call [Realtime] group related APIs using this client.
    #[cfg(feature = "realtime")]
    pub fn realtime(&self) -> Realtime<'_, C> {
        Realtime::new(self)
    }

    pub fn config(&self) -> &C {
        &self.config
    }

    /// Helper function to build a request builder with common configuration
    fn build_request_builder(
        &self,
        method: reqwest::Method,
        path: &str,
        request_options: &RequestOptions,
    ) -> reqwest::RequestBuilder {
        let mut request_builder = if let Some(path) = request_options.path() {
            self.http_client
                .request(method, self.config.url(path.as_str()))
        } else {
            self.http_client.request(method, self.config.url(path))
        };

        request_builder = request_builder
            .query(&self.config.query())
            .headers(self.config.headers());

        if let Some(headers) = request_options.headers() {
            request_builder = request_builder.headers(headers.clone());
        }

        if !request_options.query().is_empty() {
            request_builder = request_builder.query(request_options.query());
        }

        request_builder
    }

    /// Make a GET request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn get<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::GET, path, request_options)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn delete<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::DELETE, path, request_options)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a GET request to {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn get_raw(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError> {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::GET, path, request_options)
                .build()?)
        };

        self.execute_raw(request_maker).await
    }

    /// Make a POST request to {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn post_raw<I>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError>
    where
        I: Serialize,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .json(&request)
                .build()?)
        };

        self.execute_raw(request_maker).await
    }

    /// Make a POST request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn post<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// POST a form at {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn post_form_raw<F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError>
    where
        Form: AsyncTryFrom<F, Error = OpenAIError>,
        F: Clone,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .multipart(<Form as AsyncTryFrom<F>>::try_from(form.clone()).await?)
                .build()?)
        };

        self.execute_raw(request_maker).await
    }

    /// POST a form at {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn post_form<O, F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
        F: Clone,
    {
        let request_maker = || async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .multipart(<Form as AsyncTryFrom<F>>::try_from(form.clone()).await?)
                .build()?)
        };

        self.execute(request_maker).await
    }

    #[allow(unused)]
    pub(crate) async fn post_form_stream<O, F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>, OpenAIError>
    where
        F: Clone,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        // Build and execute request manually since multipart::Form is not Clone
        // and .eventsource() requires cloneability
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .multipart(<Form as AsyncTryFrom<F>>::try_from(form.clone()).await?);

        let response = request_builder.send().await.map_err(OpenAIError::Reqwest)?;

        // Check for error status
        if !response.status().is_success() {
            return Err(read_response(response).await.unwrap_err());
        }

        // Convert response body to EventSource stream
        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(std::io::Error::other));
        let event_stream = eventsource_stream::EventStream::new(stream);

        // Convert EventSource stream to our expected format
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            use futures::StreamExt;
            let mut event_stream = std::pin::pin!(event_stream);

            while let Some(event_result) = event_stream.next().await {
                match event_result {
                    Err(e) => {
                        if let Err(_e) = tx.send(Err(OpenAIError::StreamError(Box::new(
                            StreamError::EventStream(e.to_string()),
                        )))) {
                            break;
                        }
                    }
                    Ok(event) => {
                        // eventsource_stream::Event is a struct with data field
                        if event.data == "[DONE]" {
                            break;
                        }

                        let response = match serde_json::from_str::<O>(&event.data) {
                            Err(e) => Err(map_deserialization_error(e, event.data.as_bytes())),
                            Ok(output) => Ok(output),
                        };

                        if let Err(_e) = tx.send(response) {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Box::pin(
            tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
        ))
    }

    /// Execute a HTTP request and retry on rate limit
    ///
    /// request_maker serves one purpose: to be able to create request again
    /// to retry API call after getting rate limited. request_maker is async because
    /// reqwest::multipart::Form is created by async calls to read files for uploads.
    async fn execute_raw<M, Fut>(&self, request_maker: M) -> Result<(Bytes, HeaderMap), OpenAIError>
    where
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    {
        let client = self.http_client.clone();

        backoff::future::retry(self.backoff.clone(), || async {
            let request = request_maker().await.map_err(backoff::Error::Permanent)?;
            let response = client
                .execute(request)
                .await
                .map_err(OpenAIError::Reqwest)
                .map_err(backoff::Error::Permanent)?;

            let status = response.status();

            match read_response(response).await {
                Ok((bytes, headers)) => Ok((bytes, headers)),
                Err(e) => {
                    match e {
                        OpenAIError::ApiError(api_error) => {
                            if status.is_server_error() {
                                Err(backoff::Error::Transient {
                                    err: OpenAIError::ApiError(api_error),
                                    retry_after: None,
                                })
                            } else if status.as_u16() == 429
                                && api_error.r#type != Some("insufficient_quota".to_string())
                            {
                                // Rate limited retry...
                                tracing::warn!("Rate limited: {}", api_error.message);
                                Err(backoff::Error::Transient {
                                    err: OpenAIError::ApiError(api_error),
                                    retry_after: None,
                                })
                            } else {
                                Err(backoff::Error::Permanent(OpenAIError::ApiError(api_error)))
                            }
                        }
                        _ => Err(backoff::Error::Permanent(e)),
                    }
                }
            }
        })
        .await
    }

    /// Execute a HTTP request and retry on rate limit
    ///
    /// request_maker serves one purpose: to be able to create request again
    /// to retry API call after getting rate limited. request_maker is async because
    /// reqwest::multipart::Form is created by async calls to read files for uploads.
    async fn execute<O, M, Fut>(&self, request_maker: M) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    {
        let (bytes, _headers) = self.execute_raw(request_maker).await?;

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }

    /// Make HTTP POST request to receive SSE
    #[allow(unused)]
    pub(crate) async fn post_stream<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
    where
        I: Serialize,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .json(&request);

        let event_source = request_builder.eventsource().unwrap();

        stream(event_source).await
    }

    #[allow(unused)]
    pub(crate) async fn post_stream_mapped_raw_events<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
        event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static,
    ) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
    where
        I: Serialize,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .json(&request);

        let event_source = request_builder.eventsource().unwrap();

        stream_mapped_raw_events(event_source, event_mapper).await
    }

    /// Make HTTP GET request to receive SSE
    #[allow(unused)]
    pub(crate) async fn get_stream<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
    where
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let request_builder =
            self.build_request_builder(reqwest::Method::GET, path, request_options);

        let event_source = request_builder.eventsource().unwrap();

        stream(event_source).await
    }
}

async fn read_response(response: Response) -> Result<(Bytes, HeaderMap), OpenAIError> {
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response.bytes().await.map_err(OpenAIError::Reqwest)?;

    if status.is_server_error() {
        // OpenAI does not guarantee server errors are returned as JSON so we cannot deserialize them.
        let message: String = String::from_utf8_lossy(&bytes).into_owned();
        tracing::warn!("Server error: {status} - {message}");
        return Err(OpenAIError::ApiError(ApiError {
            message,
            r#type: None,
            param: None,
            code: None,
        }));
    }

    // Deserialize response body from either error object or actual response object
    if !status.is_success() {
        let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        return Err(OpenAIError::ApiError(wrapped_error.error));
    }

    Ok((bytes, headers))
}

async fn map_stream_error(value: EventSourceError) -> OpenAIError {
    match value {
        EventSourceError::InvalidStatusCode(status_code, response) => {
            read_response(response).await.expect_err(&format!(
                "Unreachable because read_response returns err when status_code {status_code} is invalid"
            ))
        }
        _ => OpenAIError::StreamError(Box::new(StreamError::ReqwestEventSource(value))),
    }
}

/// Request which responds with SSE.
/// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
pub(crate) async fn stream<O>(
    mut event_source: EventSource,
) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
where
    O: DeserializeOwned + std::marker::Send + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(ev) = event_source.next().await {
            match ev {
                Err(e) => {
                    // Handle StreamEnded gracefully - it's a normal end of stream, not an error
                    // https://github.com/64bit/async-openai/issues/456
                    match &e {
                        EventSourceError::StreamEnded => {
                            break;
                        }
                        _ => {
                            if let Err(_e) = tx.send(Err(map_stream_error(e).await)) {
                                // rx dropped
                                break;
                            }
                        }
                    }
                }
                Ok(event) => match event {
                    Event::Message(message) => {
                        if message.data == "[DONE]" {
                            break;
                        }

                        let response = match serde_json::from_str::<O>(&message.data) {
                            Err(e) => Err(map_deserialization_error(e, message.data.as_bytes())),
                            Ok(output) => Ok(output),
                        };

                        if let Err(_e) = tx.send(response) {
                            // rx dropped
                            break;
                        }
                    }
                    Event::Open => continue,
                },
            }
        }

        event_source.close();
    });

    Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
}

pub(crate) async fn stream_mapped_raw_events<O>(
    mut event_source: EventSource,
    event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static,
) -> Pin<Box<dyn Stream<Item = Result<O, OpenAIError>> + Send>>
where
    O: DeserializeOwned + std::marker::Send + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(ev) = event_source.next().await {
            match ev {
                Err(e) => {
                    // Handle StreamEnded gracefully - it's a normal end of stream, not an error
                    // https://github.com/64bit/async-openai/issues/456
                    match &e {
                        EventSourceError::StreamEnded => {
                            break;
                        }
                        _ => {
                            if let Err(_e) = tx.send(Err(map_stream_error(e).await)) {
                                // rx dropped
                                break;
                            }
                        }
                    }
                }
                Ok(event) => match event {
                    Event::Message(message) => {
                        let mut done = false;

                        if message.data == "[DONE]" {
                            done = true;
                        }

                        let response = event_mapper(message);

                        if let Err(_e) = tx.send(response) {
                            // rx dropped
                            break;
                        }

                        if done {
                            break;
                        }
                    }
                    Event::Open => continue,
                },
            }
        }

        event_source.close();
    });

    Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
}
