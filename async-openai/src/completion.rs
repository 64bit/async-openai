use crate::{
    client::Client,
    error::{ApiError, OpenAIError},
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
        let reqwest_error = response.error_for_status_ref().err();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let api_error: ApiError =
                serde_json::from_slice(bytes.as_ref()).map_err(|_| reqwest_error.unwrap())?;
            return Err(OpenAIError::ApiError(api_error));
        }

        let response: CreateCompletionResponse =
            serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;
        Ok(response)
    }
}
