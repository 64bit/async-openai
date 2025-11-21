use crate::{
    error::OpenAIError, traits::AsyncTryFrom, types::videos::CreateVideoRequest,
    util::create_file_part,
};

impl AsyncTryFrom<CreateVideoRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("model", request.model);

        form = form.text("prompt", request.prompt);

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string());
        }

        if request.seconds.is_some() {
            form = form.text("seconds", request.seconds.unwrap());
        }

        if request.input_reference.is_some() {
            let image_part = create_file_part(request.input_reference.unwrap().source).await?;
            form = form.part("input_reference", image_part);
        }

        Ok(form)
    }
}
