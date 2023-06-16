use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImageResponse,
    },
    util::create_file_part,
    Client,
};

/// Given a prompt and/or an input image, the model will generate a new image.
///
/// Related guide: [Image generation](https://platform.openai.com/docs/guides/images/introduction)
pub struct Images<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Images<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
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
        let image_part = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("image", image_part)
            .text("prompt", request.prompt);

        if let Some(mask) = request.mask {
            let mask_part = create_file_part(&mask.path).await?;
            form = form.part("mask", mask_part);
        }

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }

        self.client.post_form("/images/edits", form).await
    }

    /// Creates a variation of a given image.
    pub async fn create_variation(
        &self,
        request: CreateImageVariationRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let image_part = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image_part);

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }

        self.client.post_form("/images/variations", form).await
    }
}
