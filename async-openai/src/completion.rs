use crate::{
    client::Client,
    error::{OpenAIError, WrappedError},
    types::{CreateCompletionRequest, CreateCompletionResponse},
};

pub struct Completion;

impl Completion {
    pub async fn create(
        client: &Client,
        request: CreateCompletionRequest,
    ) -> Result<CreateCompletionResponse, OpenAIError> {
        let response = reqwest::Client::new()
            .post(format!("{}/completions", client.api_base()))
            .bearer_auth(client.api_key())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let wrapped_error: WrappedError =
                serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;

            return Err(OpenAIError::ApiError(wrapped_error.error));
        }

        let response: CreateCompletionResponse =
            serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;
        Ok(response)
    }
}
