use crate::{
    error::OpenAIError,
    types::{
        CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImageResponse,
    },
    Client,
};

/// Given a prompt and/or an input image, the model will generate a new image.
///
/// Related guide: [Image generation](https://platform.openai.com/docs/guides/images/introduction)
pub struct Images<'c> {
    client: &'c Client,
}

impl<'c> Images<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates an image given a prompt.
    pub async fn create(&self, request: CreateImageRequest) -> Result<ImageResponse, OpenAIError> {
        self.client.post("/images/generations", request).await
    }

    /// Creates an edited or extended image given an original image and a prompt.
    pub async fn create_edit(
        &self,
        request: CreateImageEditRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        self.client.post_form("/images/edits", request).await
    }

    /// Creates a variation of a given image.
    pub async fn create_variation(
        &self,
        request: CreateImageVariationRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        self.client.post_form("/images/variations", request).await
    }
}
