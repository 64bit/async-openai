use crate::{
    error::OpenAIError, traits::AsyncTryFrom, types::videos::CreateVideoRequest,
    util::create_file_part,
};

impl AsyncTryFrom<CreateVideoRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("model", request.model);

        form = form.text("prompt", request.prompt);

        if let Some(size) = request.size {
            form = form.text("size", size.to_string());
        }

        if let Some(seconds) = request.seconds {
            form = form.text("seconds", seconds.to_string());
        }

        if let Some(input_reference) = request.input_reference {
            let image_part = create_file_part(input_reference.source).await?;
            form = form.part("input_reference", image_part);
        }

        if let Some(character_ids) = request.character_ids {
            for character_id in character_ids {
                form = form.text("character_ids[]", character_id);
            }
        }

        Ok(form)
    }
}
