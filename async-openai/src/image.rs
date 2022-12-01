use reqwest::Body;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    error::OpenAIError,
    types::{CreateImageEditRequest, CreateImageRequest, ImageInput, ImageResponse},
    Client,
};

pub struct Image;

impl Image {
    pub async fn create(
        client: &Client,
        request: CreateImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        client.post("/images/generations", request).await
    }

    pub(crate) async fn file_stream_body(image_input: &ImageInput) -> Result<Body, OpenAIError> {
        let file = tokio::fs::File::open(image_input.path.as_path())
            .await
            .map_err(|e| OpenAIError::ImageReadError(e.to_string()))?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        Ok(body)
    }

    pub(crate) async fn create_part(
        image_input: &ImageInput,
    ) -> Result<reqwest::multipart::Part, OpenAIError> {
        let image_file_name = image_input
            .path
            .as_path()
            .file_name()
            .ok_or(OpenAIError::ImageReadError(format!(
                "cannot extract file name from {:#?}",
                image_input.path
            )))?
            .to_str()
            .unwrap()
            .to_string();

        let image_part =
            reqwest::multipart::Part::stream(Image::file_stream_body(image_input).await?)
                .file_name(image_file_name)
                .mime_str("application/octet-stream")
                .unwrap();

        Ok(image_part)
    }

    pub async fn create_edit(
        client: &Client,
        request: CreateImageEditRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let image_part = Image::create_part(&request.image).await?;
        let mask_part = Image::create_part(&request.mask).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("image", image_part)
            .part("mask", mask_part)
            .text("prompt", request.prompt);

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

        client.post_form("/images/edits", form).await
    }
}
