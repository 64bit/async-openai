use serde::{de::DeserializeOwned, Serialize};

use crate::error::{OpenAIError, WrappedError};

#[derive(Debug, Default)]
pub struct Client {
    api_key: String,
    api_base: String,
    org_id: String,
    //headers: reqwest::header::HeaderMap,
}

const API_BASE: &str = "https://api.openai.com/v1";

impl Client {
    pub fn new() -> Self {
        Self {
            api_base: API_BASE.to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
            ..Default::default()
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn with_org_id(mut self, org_id: String) -> Self {
        self.org_id = org_id;
        self
    }

    pub fn with_api_base(mut self, api_base: String) -> Self {
        self.api_base = api_base;
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

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
