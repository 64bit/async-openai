use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use std::sync::Mutex;

use bytes::Bytes;
use futures::stream::StreamExt;
use reqwest::{header::HeaderMap, multipart::Form, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::StreamError;
#[cfg(feature = "middleware")]
use crate::executor::TowerExecutor;
use crate::{
    config::{Config, OpenAIConfig},
    error::{map_deserialization_error, ApiError, OpenAIError, WrappedError},
    executor::{HttpRequestFactory, ReqwestExecutor, SharedExecutor},
    traits::AsyncTryFrom,
    RequestOptions,
};

struct RequestParts {
    request_client: reqwest::Client,
    method: reqwest::Method,
    url: String,
    headers: HeaderMap,
    query: Vec<(String, String)>,
}

impl RequestParts {
    fn build_request_builder(&self) -> reqwest::RequestBuilder {
        self.request_client
            .request(self.method.clone(), self.url.clone())
            .query(&self.query)
            .headers(self.headers.clone())
    }
}

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
#[allow(deprecated)]
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
#[cfg(feature = "skill")]
use crate::Skills;
#[cfg(feature = "assistant")]
#[allow(deprecated)]
use crate::Threads;
#[cfg(feature = "upload")]
use crate::Uploads;
#[cfg(feature = "vectorstore")]
use crate::VectorStores;
#[cfg(feature = "video")]
use crate::Videos;

#[derive(Clone)]
/// Client is a container for config and HTTP execution
/// used to make API calls.
pub struct Client<C: Config> {
    request_client: reqwest::Client,
    executor: SharedExecutor,
    config: C,
}

impl<C> std::fmt::Debug for Client<C>
where
    C: Config + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("request_client", &self.request_client)
            .field("config", &self.config)
            .finish()
    }
}

impl<C: Config> Default for Client<C>
where
    C: Default,
{
    fn default() -> Self {
        let request_client = reqwest::Client::new();
        Self {
            executor: Arc::new(ReqwestExecutor::new(request_client.clone())),
            request_client,
            config: C::default(),
        }
    }
}

impl Client<OpenAIConfig> {
    /// Client with default [OpenAIConfig]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<C: Config> Client<C> {
    /// Create client with a custom HTTP client and config.
    pub fn build(http_client: reqwest::Client, config: C) -> Self {
        Self {
            executor: Arc::new(ReqwestExecutor::new(http_client.clone())),
            request_client: http_client,
            config,
        }
    }

    /// Create client with [OpenAIConfig] or [crate::config::AzureConfig]
    pub fn with_config(config: C) -> Self {
        let request_client = reqwest::Client::new();
        Self {
            executor: Arc::new(ReqwestExecutor::new(request_client.clone())),
            request_client,
            config,
        }
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.executor = Arc::new(ReqwestExecutor::new(http_client.clone()));
        self.request_client = http_client;
        self
    }

    /// Provide your own tower-compatible service to execute HTTP requests.
    #[cfg(all(feature = "middleware", not(target_family = "wasm")))]
    pub fn with_http_service<S>(mut self, service: S) -> Self
    where
        S: tower::Service<HttpRequestFactory, Response = Response> + Clone + Send + Sync + 'static,
        S::Future: Send + 'static,
        S::Error: Into<OpenAIError> + Send + Sync + 'static,
    {
        // This is the public middleware escape hatch. We erase the concrete
        // tower stack here so the rest of the client does not become generic
        // over the service type, which would otherwise leak through every API
        // group and make the crate much harder to use.
        self.executor = Arc::new(TowerExecutor::new(service));
        self
    }

    /// Provide your own tower-compatible service to execute HTTP requests.
    #[cfg(all(feature = "middleware", target_family = "wasm"))]
    pub fn with_http_service<S>(mut self, service: S) -> Self
    where
        S: tower::Service<HttpRequestFactory, Response = Response> + Clone + 'static,
        S::Future: 'static,
        S::Error: Into<OpenAIError> + 'static,
    {
        // wasm futures produced by reqwest are not `Send`, so the wasm version
        // intentionally avoids native thread-safety bounds. Users are still
        // responsible for choosing tower layers that work in their wasm
        // runtime.
        self.executor = Arc::new(TowerExecutor::new(service));
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
    #[deprecated(
        note = "Assistants API is deprecated and will be removed in August 2026. Use the Responses API."
    )]
    #[allow(deprecated)]
    pub fn assistants(&self) -> Assistants<'_, C> {
        Assistants::new(self)
    }

    /// To call [Threads] group related APIs using this client.
    #[cfg(feature = "assistant")]
    #[deprecated(
        note = "Assistants API is deprecated and will be removed in August 2026. Use the Responses API."
    )]
    #[allow(deprecated)]
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

    /// To call [Skills] group related APIs using this client.
    #[cfg(feature = "skill")]
    pub fn skills(&self) -> Skills<'_, C> {
        Skills::new(self)
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

    fn build_request_parts(
        &self,
        method: reqwest::Method,
        path: &str,
        request_options: &RequestOptions,
    ) -> Arc<RequestParts> {
        let url = if let Some(path) = request_options.path() {
            self.config.url(path.as_str())
        } else {
            self.config.url(path)
        };
        let mut headers = self.config.headers();
        if let Some(request_headers) = request_options.headers() {
            headers.extend(request_headers.clone());
        }

        let mut query = self
            .config
            .query()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<Vec<_>>();
        query.extend_from_slice(request_options.query());

        Arc::new(RequestParts {
            request_client: self.request_client.clone(),
            method,
            url,
            headers,
            query,
        })
    }

    fn build_request_factory(
        &self,
        method: reqwest::Method,
        path: &str,
        request_options: &RequestOptions,
    ) -> HttpRequestFactory {
        let request_parts = self.build_request_parts(method, path, request_options);

        HttpRequestFactory::new(move || {
            let request_parts = request_parts.clone();

            async move {
                let request = request_parts.build_request_builder().build()?;
                Ok(request)
            }
        })
    }

    fn build_request_factory_with_json<I>(
        &self,
        method: reqwest::Method,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<HttpRequestFactory, OpenAIError>
    where
        I: Serialize,
    {
        // JSON bodies are materialized once so the base BYOT path can keep
        // accepting borrowed inputs. Middleware-enabled BYOT still adds owned
        // replay bounds in the macro, but the core client does not force those
        // bounds onto non-middleware users.
        let request = Bytes::from(serde_json::to_vec(&request).map_err(|error| {
            OpenAIError::InvalidArgument(format!("failed to serialize request: {error}"))
        })?);
        let request_parts = self.build_request_parts(method, path, request_options);

        Ok(HttpRequestFactory::new(move || {
            let request_parts = request_parts.clone();
            let request = request.clone();

            async move {
                let request_builder = request_parts
                    .build_request_builder()
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .body(request.clone());

                Ok(request_builder.build()?)
            }
        }))
    }

    fn build_request_factory_with_form<F>(
        &self,
        method: reqwest::Method,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<HttpRequestFactory, OpenAIError>
    where
        F: Clone + crate::traits::MaybeSend + 'static,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
    {
        // Multipart is the reason the factory exists.
        //
        // `Mutex` is only here to make the captured state `Sync` on native targets.
        #[cfg(not(target_family = "wasm"))]
        let form = Arc::new(Mutex::new(form));
        let request_parts = self.build_request_parts(method, path, request_options);

        Ok(HttpRequestFactory::new(move || {
            let request_parts = request_parts.clone();
            let form = form.clone();

            async move {
                #[cfg(not(target_family = "wasm"))]
                let form = form
                    .lock()
                    .expect("multipart request factory mutex poisoned")
                    .clone();
                #[cfg(target_family = "wasm")]
                let form = form.clone();
                let form = <Form as AsyncTryFrom<F>>::try_from(form).await?;
                let request_builder = request_parts.build_request_builder().multipart(form);

                Ok(request_builder.build()?)
            }
        }))
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
        let request_factory =
            self.build_request_factory(reqwest::Method::GET, path, request_options);
        self.execute(request_factory).await
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
        let request_factory =
            self.build_request_factory(reqwest::Method::DELETE, path, request_options);
        self.execute(request_factory).await
    }

    /// Make a GET request to {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn get_raw(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError> {
        let request_factory =
            self.build_request_factory(reqwest::Method::GET, path, request_options);
        self.execute_raw(request_factory).await
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
        let request_factory = self.build_request_factory_with_json(
            reqwest::Method::POST,
            path,
            request,
            request_options,
        )?;
        self.execute_raw(request_factory).await
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
        let request_factory = self.build_request_factory_with_json(
            reqwest::Method::POST,
            path,
            request,
            request_options,
        )?;
        self.execute(request_factory).await
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
        F: Clone + crate::traits::MaybeSend + 'static,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
    {
        let request_factory = self.build_request_factory_with_form(
            reqwest::Method::POST,
            path,
            form,
            request_options,
        )?;
        self.execute_raw(request_factory).await
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
        F: Clone + crate::traits::MaybeSend + 'static,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
    {
        let request_factory = self.build_request_factory_with_form(
            reqwest::Method::POST,
            path,
            form,
            request_options,
        )?;
        self.execute(request_factory).await
    }

    #[allow(unused)]
    pub(crate) async fn post_form_stream<O, F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        F: Clone + crate::traits::MaybeSend + 'static,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let request_factory = self.build_request_factory_with_form(
            reqwest::Method::POST,
            path,
            form,
            request_options,
        )?;

        self.execute_stream(request_factory).await
    }

    async fn execute_raw(
        &self,
        request_factory: HttpRequestFactory,
    ) -> Result<(Bytes, HeaderMap), OpenAIError> {
        let response = self.execute_response(request_factory).await?;
        read_response(response).await
    }

    async fn execute<O>(&self, request_factory: HttpRequestFactory) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let (bytes, _headers) = self.execute_raw(request_factory).await?;

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }

    async fn execute_response(
        &self,
        request_factory: HttpRequestFactory,
    ) -> Result<Response, OpenAIError> {
        self.executor.execute(request_factory).await
    }

    async fn execute_stream<O>(
        &self,
        request_factory: HttpRequestFactory,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let response = self.execute_response(request_factory).await?;
        Ok(stream(response).await)
    }

    async fn execute_stream_mapped_raw_events<O>(
        &self,
        request_factory: HttpRequestFactory,
        event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError>
            + crate::traits::MaybeSend
            + 'static,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let response = self.execute_response(request_factory).await?;
        Ok(stream_mapped_raw_events(response, event_mapper).await)
    }

    /// Make HTTP POST request to receive SSE
    #[allow(unused)]
    pub(crate) async fn post_stream<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let request_factory = self.build_request_factory_with_json(
            reqwest::Method::POST,
            path,
            request,
            request_options,
        )?;
        // Stream setup is still request/response first. We only create the SSE
        // stream after the HTTP layer has returned a response object.
        self.execute_stream(request_factory).await
    }

    #[allow(unused)]
    pub(crate) async fn post_stream_mapped_raw_events<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
        event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError>
            + crate::traits::MaybeSend
            + 'static,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let request_factory = self.build_request_factory_with_json(
            reqwest::Method::POST,
            path,
            request,
            request_options,
        )?;
        self.execute_stream_mapped_raw_events(request_factory, event_mapper)
            .await
    }

    /// Make HTTP GET request to receive SSE
    #[allow(unused)]
    pub(crate) async fn get_stream<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<crate::types::stream::StreamResponse<O>, OpenAIError>
    where
        O: DeserializeOwned + crate::traits::MaybeSend + 'static,
    {
        let request_factory =
            self.build_request_factory(reqwest::Method::GET, path, request_options);
        self.execute_stream(request_factory).await
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

/// Request which responds with SSE.
/// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
pub(crate) async fn stream<O>(response: Response) -> crate::types::stream::StreamResponse<O>
where
    O: DeserializeOwned + crate::traits::MaybeSend + 'static,
{
    stream_mapped_raw_events(response, |event| {
        serde_json::from_str::<O>(&event.data)
            .map_err(|error| map_deserialization_error(error, event.data.as_bytes()))
    })
    .await
}

#[cfg(target_family = "wasm")]
pub(crate) async fn stream_mapped_raw_events<O>(
    response: Response,
    event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + 'static,
) -> crate::types::stream::StreamResponse<O>
where
    O: DeserializeOwned + 'static,
{
    if !response.status().is_success() {
        return Box::pin(futures::stream::once(async move {
            match read_response(response).await {
                Ok(_) => Err(OpenAIError::InvalidArgument(
                    "stream request failed without an error body".into(),
                )),
                Err(error) => Err(error),
            }
        }));
    }

    let byte_stream = response
        .bytes_stream()
        .map(|result| result.map_err(std::io::Error::other));
    let event_stream = Box::pin(eventsource_stream::EventStream::new(byte_stream));

    Box::pin(futures::stream::unfold(
        (event_stream, event_mapper),
        |(mut event_stream, event_mapper)| async move {
            loop {
                let event = match event_stream.next().await {
                    Some(Ok(event)) => event,
                    Some(Err(error)) => {
                        return Some((
                            Err(OpenAIError::StreamError(Box::new(
                                StreamError::EventStream(error.to_string()),
                            ))),
                            (event_stream, event_mapper),
                        ));
                    }
                    None => return None,
                };

                if event.data == "[DONE]" {
                    return None;
                }

                if event.event == "keepalive" {
                    continue;
                }

                let response = event_mapper(event);
                return Some((response, (event_stream, event_mapper)));
            }
        },
    ))
}

#[cfg(not(target_family = "wasm"))]
pub(crate) async fn stream_mapped_raw_events<O>(
    response: Response,
    event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static,
) -> crate::types::stream::StreamResponse<O>
where
    O: DeserializeOwned + std::marker::Send + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        if !response.status().is_success() {
            if let Err(e) = read_response(response).await {
                let _ = tx.send(Err(e));
            }
            return;
        }
        let byte_stream = response
            .bytes_stream()
            .map(|r| r.map_err(std::io::Error::other));
        let mut event_stream = std::pin::pin!(eventsource_stream::EventStream::new(byte_stream));

        while let Some(ev) = event_stream.next().await {
            let event = match ev {
                Ok(e) => e,
                Err(e) => {
                    let _ = tx.send(Err(OpenAIError::StreamError(Box::new(
                        StreamError::EventStream(e.to_string()),
                    ))));
                    break;
                }
            };
            if event.data == "[DONE]" {
                break;
            }

            if event.event == "keepalive" {
                continue;
            }

            let response = event_mapper(event);

            if tx.send(response).is_err() {
                break;
            }
        }
    });

    Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
}

#[cfg(all(test, feature = "middleware", not(target_family = "wasm")))]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use futures::StreamExt;
    use http::Response as HttpResponse;
    use serde_json::json;
    use tower::{service_fn, ServiceBuilder};

    use super::Client;
    use crate::{
        config::OpenAIConfig, error::OpenAIError, executor::HttpRequestFactory,
        retry::SimpleRetryPolicy, traits::AsyncTryFrom, RequestOptions,
    };

    #[tokio::test]
    async fn unary_requests_dispatch_through_middleware_service() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let service = {
            let request_count = request_count.clone();
            ServiceBuilder::new()
                .concurrency_limit(1)
                .service(service_fn(move |factory: HttpRequestFactory| {
                    let request_count = request_count.clone();
                    async move {
                        let request = factory.build().await?;
                        assert_eq!(request.url().path(), "/models");
                        request_count.fetch_add(1, Ordering::SeqCst);
                        Ok::<reqwest::Response, OpenAIError>(
                            HttpResponse::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(reqwest::Body::from(
                                    "{\"object\":\"list\",\"data\":[{\"id\":\"model\"}]}",
                                ))
                                .unwrap()
                                .into(),
                        )
                    }
                }))
        };

        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base("http://example.test")
                .with_api_key("test-key"),
        )
        .with_http_service(service);

        let value: serde_json::Value = client.get("/models", &RequestOptions::new()).await.unwrap();

        assert_eq!(value["object"], "list");
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn stream_requests_open_through_middleware_service() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let service = {
            let request_count = request_count.clone();
            ServiceBuilder::new()
                .concurrency_limit(1)
                .service(service_fn(move |factory: HttpRequestFactory| {
                    let request_count = request_count.clone();
                    async move {
                        let request = factory.build().await?;
                        assert_eq!(request.url().path(), "/responses");
                        request_count.fetch_add(1, Ordering::SeqCst);
                        Ok::<reqwest::Response, OpenAIError>(
                            HttpResponse::builder()
                                .status(200)
                                .header("content-type", "text/event-stream")
                                .body(reqwest::Body::from(
                                    "data: {\"ok\":true}\n\ndata: [DONE]\n\n",
                                ))
                                .unwrap()
                                .into(),
                        )
                    }
                }))
        };

        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base("http://example.test")
                .with_api_key("test-key"),
        )
        .with_http_service(service);

        let mut stream = client
            .post_stream::<_, serde_json::Value>(
                "/responses",
                json!({ "stream": true }),
                &RequestOptions::new(),
            )
            .await
            .unwrap();

        let first = stream.next().await.unwrap().unwrap();

        assert_eq!(first, json!({ "ok": true }));
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn middleware_retry_policy_retries_429_responses() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let service = {
            let request_count = request_count.clone();
            ServiceBuilder::new()
                .retry(SimpleRetryPolicy::default())
                .service(service_fn(move |factory: HttpRequestFactory| {
                    let request_count = request_count.clone();
                    async move {
                        let request = factory.build().await?;
                        assert_eq!(request.url().path(), "/models");
                        let attempt = request_count.fetch_add(1, Ordering::SeqCst);

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
                                .header("content-type", "application/json")
                                .body(reqwest::Body::from(
                                    r#"{"object":"list","data":[{"id":"retry-model"}]}"#,
                                ))
                                .unwrap()
                        };

                        Ok::<reqwest::Response, OpenAIError>(response.into())
                    }
                }))
        };

        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base("http://example.test")
                .with_api_key("test-key"),
        )
        .with_http_service(service);

        let value: serde_json::Value = client.get("/models", &RequestOptions::new()).await.unwrap();

        assert_eq!(value["data"][0]["id"], "retry-model");
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[derive(Clone)]
    struct RetryableMultipartInput {
        conversions: Arc<AtomicUsize>,
    }

    impl AsyncTryFrom<RetryableMultipartInput> for reqwest::multipart::Form {
        type Error = OpenAIError;

        async fn try_from(value: RetryableMultipartInput) -> Result<Self, Self::Error> {
            value.conversions.fetch_add(1, Ordering::SeqCst);
            Ok(reqwest::multipart::Form::new().text("field", "value"))
        }
    }

    #[tokio::test]
    async fn middleware_retry_policy_rebuilds_multipart_form_per_attempt() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let conversion_count = Arc::new(AtomicUsize::new(0));

        let service = {
            let request_count = request_count.clone();
            ServiceBuilder::new()
                .retry(SimpleRetryPolicy::default())
                .service(service_fn(move |factory: HttpRequestFactory| {
                    let request_count = request_count.clone();
                    async move {
                        let request = factory.build().await?;
                        assert_eq!(request.method(), reqwest::Method::POST);
                        assert_eq!(request.url().path(), "/files");
                        let attempt = request_count.fetch_add(1, Ordering::SeqCst);

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
                                .header("content-type", "application/json")
                                .body(reqwest::Body::from(r#"{"ok":true}"#))
                                .unwrap()
                        };

                        Ok::<reqwest::Response, OpenAIError>(response.into())
                    }
                }))
        };

        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base("http://example.test")
                .with_api_key("test-key"),
        )
        .with_http_service(service);

        let value: serde_json::Value = client
            .post_form(
                "/files",
                RetryableMultipartInput {
                    conversions: conversion_count.clone(),
                },
                &RequestOptions::new(),
            )
            .await
            .unwrap();

        assert_eq!(value, json!({ "ok": true }));
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
        assert_eq!(conversion_count.load(Ordering::SeqCst), 2);
    }
}
