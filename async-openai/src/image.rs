use crate::{
    error::OpenAIError,
    types::{CreateImageRequest, ImageResponse},
    Client,
};

pub struct Image;

impl Image {
    pub async fn create(
        client: &Client,
        request: CreateImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let response = reqwest::Client::new()
            .post(format!("{}/images/generations", client.api_base()))
            .bearer_auth(client.api_key())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let wrapped_error: crate::error::WrappedError =
                serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;

            return Err(OpenAIError::ApiError(wrapped_error.error));
        }

        let response: ImageResponse =
            serde_json::from_slice(bytes.as_ref()).map_err(OpenAIError::JSONDeserialize)?;
        Ok(response)
    }
}
