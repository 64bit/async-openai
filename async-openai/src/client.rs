use serde::{de::DeserializeOwned, Serialize};

use crate::error::{OpenAIError, WrappedError};

#[derive(Debug, Default)]
/// Client container for api key, base url and other metadata
/// required to make API calls.
pub struct Client {
    api_key: String,
    api_base: String,
    org_id: String,
    //headers: reqwest::header::HeaderMap,
}

/// Default v1 API base url
pub const API_BASE: &str = "https://api.openai.com/v1";

impl Client {
    /// Create client with default [API_BASE] url and default API key from OPENAI_API_KEY env var
    pub fn new() -> Self {
        Self {
            api_base: API_BASE.to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
            ..Default::default()
        }
    }

    /// To use a different API key different from default OPENAI_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = api_key.into();
        self
    }

    pub fn with_org_id<S: Into<String>>(mut self, org_id: S) -> Self {
        self.org_id = org_id.into();
        self
    }

    /// To use a API base url different from default [API_BASE]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Deserialize response body from either error object or actual response object
    async fn process_response<O>(&self, response: reqwest::Response) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let wrapped_error: WrappedError =
                serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;

            return Err(OpenAIError::ApiError(wrapped_error.error));
        }

        let response: O =
            serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;
        Ok(response)
    }

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .get(format!("{}{path}", self.api_base()))
            .bearer_auth(self.api_key())
            .send()
            .await?;

        self.process_response(response).await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    pub(crate) async fn delete<O>(&self, path: &str) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .delete(format!("{}{path}", self.api_base()))
            .bearer_auth(self.api_key())
            .send()
            .await?;

        self.process_response(response).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .post(format!("{}{path}", self.api_base()))
            .bearer_auth(self.api_key())
            .json(&request)
            .send()
            .await?;

        self.process_response(response).await
    }

    /// POST a form at {path} and deserialize the response body
    pub(crate) async fn post_form<O>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .post(format!("{}{path}", self.api_base()))
            .bearer_auth(self.api_key())
            .multipart(form)
            .send()
            .await?;

        self.process_response(response).await
    }
}
