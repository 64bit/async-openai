use crate::{
    config::Config,
    error::OpenAIError,
    types::responses::{
        CreateResponse, DeleteResponse, Response, ResponseItemList, ResponseStream,
        TokenCountsBody, TokenCountsResource,
    },
    Client, RequestOptions,
};

pub struct Responses<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Responses<'c, C> {
    /// Constructs a new Responses client.
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates a model response. Provide [text](https://platform.openai.com/docs/guides/text) or
    /// [image](https://platform.openai.com/docs/guides/images) inputs to generate
    /// [text](https://platform.openai.com/docs/guides/text) or
    /// [JSON](https://platform.openai.com/docs/guides/structured-outputs) outputs. Have the model call
    /// your own [custom code](https://platform.openai.com/docs/guides/function-calling) or use
    /// built-in [tools](https://platform.openai.com/docs/guides/tools) like
    /// [web search](https://platform.openai.com/docs/guides/tools-web-search)
    /// or [file search](https://platform.openai.com/docs/guides/tools-file-search) to use your own data
    /// as input for the model's response.
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn create(&self, request: CreateResponse) -> Result<Response, OpenAIError> {
        self.client
            .post("/responses", request, &self.request_options)
            .await
    }

    /// Creates a model response for the given input with streaming.
    ///
    /// Response events will be sent as server-sent events as they become available,
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static"
    )]
    #[allow(unused_mut)]
    pub async fn create_stream(
        &self,
        mut request: CreateResponse,
    ) -> Result<ResponseStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if matches!(request.stream, Some(false)) {
                return Err(OpenAIError::InvalidArgument(
                    "When stream is false, use Responses::create".into(),
                ));
            }
            request.stream = Some(true);
        }
        Ok(self
            .client
            .post_stream("/responses", request, &self.request_options)
            .await)
    }

    /// Retrieves a model response with the given ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, response_id: &str) -> Result<Response, OpenAIError> {
        self.client
            .get(
                &format!("/responses/{}", response_id),
                &self.request_options,
            )
            .await
    }

    /// Retrieves a model response with the given ID with streaming.
    ///
    /// Response events will be sent as server-sent events as they become available.
    #[crate::byot(
        T0 = std::fmt::Display,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static"
    )]
    pub async fn retrieve_stream(&self, response_id: &str) -> Result<ResponseStream, OpenAIError> {
        let mut request_options = self.request_options.clone();
        request_options.with_query(&[("stream", "true")])?;

        Ok(self
            .client
            .get_stream(&format!("/responses/{}", response_id), &request_options)
            .await)
    }

    /// Deletes a model response with the given ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, response_id: &str) -> Result<DeleteResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/responses/{}", response_id),
                &self.request_options,
            )
            .await
    }

    /// Cancels a model response with the given ID. Only responses created with the
    /// `background` parameter set to `true` can be cancelled.
    /// [Learn more](https://platform.openai.com/docs/guides/background).
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, response_id: &str) -> Result<Response, OpenAIError> {
        self.client
            .post(
                &format!("/responses/{}/cancel", response_id),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of input items for a given response.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn list_input_items(
        &self,
        response_id: &str,
    ) -> Result<ResponseItemList, OpenAIError> {
        self.client
            .get(
                &format!("/responses/{}/input_items", response_id),
                &self.request_options,
            )
            .await
    }

    /// Get input token counts
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn get_input_token_counts(
        &self,
        request: TokenCountsBody,
    ) -> Result<TokenCountsResource, OpenAIError> {
        self.client
            .post("/responses/input_tokens", request, &self.request_options)
            .await
    }
}
