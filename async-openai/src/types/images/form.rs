use crate::{
    error::OpenAIError,
    traits::AsyncTryFrom,
    types::images::{CreateImageEditRequest, CreateImageVariationRequest, ImageEditInput},
    util::create_file_part,
};

impl AsyncTryFrom<CreateImageEditRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateImageEditRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("prompt", request.prompt);

        match request.image {
            ImageEditInput::Image(image) => {
                let image_part = create_file_part(image.source).await?;
                form = form.part("image", image_part);
            }
            ImageEditInput::Images(images) => {
                for image in images {
                    let image_part = create_file_part(image.source).await?;
                    form = form.part("image[]", image_part);
                }
            }
        }

        if let Some(mask) = request.mask {
            let mask_part = create_file_part(mask.source).await?;
            form = form.part("mask", mask_part);
        }

        if let Some(background) = request.background {
            form = form.text("background", background.to_string())
        }

        if let Some(model) = request.model {
            form = form.text("model", model.to_string())
        }

        if let Some(n) = request.n {
            form = form.text("n", n.to_string())
        }

        if let Some(size) = request.size {
            form = form.text("size", size.to_string())
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string())
        }

        if let Some(output_format) = request.output_format {
            form = form.text("output_format", output_format.to_string())
        }

        if let Some(output_compression) = request.output_compression {
            form = form.text("output_compression", output_compression.to_string())
        }

        if let Some(output_compression) = request.output_compression {
            form = form.text("output_compression", output_compression.to_string())
        }

        if let Some(user) = request.user {
            form = form.text("user", user)
        }

        if let Some(input_fidelity) = request.input_fidelity {
            form = form.text("input_fidelity", input_fidelity.to_string())
        }

        if let Some(stream) = request.stream {
            form = form.text("stream", stream.to_string())
        }

        if let Some(partial_images) = request.partial_images {
            form = form.text("partial_images", partial_images.to_string())
        }

        if let Some(quality) = request.quality {
            form = form.text("quality", quality.to_string())
        }

        Ok(form)
    }
}

impl AsyncTryFrom<CreateImageVariationRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateImageVariationRequest) -> Result<Self, Self::Error> {
        let image_part = create_file_part(request.image.source).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image_part);

        if let Some(model) = request.model {
            form = form.text("model", model.to_string())
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
        Ok(form)
    }
}
