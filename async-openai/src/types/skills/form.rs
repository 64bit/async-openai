use crate::{
    error::OpenAIError,
    traits::AsyncTryFrom,
    types::skills::{CreateSkillRequest, CreateSkillVersionRequest},
    util::create_file_part,
};

impl AsyncTryFrom<CreateSkillRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateSkillRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new();

        for file_source in request.files {
            let file_part = create_file_part(file_source).await?;
            form = form.part("files[]", file_part);
        }

        Ok(form)
    }
}

impl AsyncTryFrom<CreateSkillVersionRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateSkillVersionRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new();

        for file_source in request.files {
            let file_part = create_file_part(file_source).await?;
            form = form.part("files[]", file_part);
        }

        if let Some(default) = request.default {
            form = form.text("default", default.to_string());
        }

        Ok(form)
    }
}
