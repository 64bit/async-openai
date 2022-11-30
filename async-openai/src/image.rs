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
        client.execute("/images/generations", request).await
    }
}
