use crate::{
    error::OpenAIError, traits::AsyncTryFrom, types::files::CreateFileRequest,
    util::create_file_part,
};

impl AsyncTryFrom<CreateFileRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateFileRequest) -> Result<Self, Self::Error> {
        let file_part = create_file_part(request.file.source).await?;
        let mut form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("purpose", request.purpose.to_string());

        if let Some(expires_after) = request.expires_after {
            form = form
                .text("expires_after[anchor]", expires_after.anchor.to_string())
                .text("expires_after[seconds]", expires_after.seconds.to_string());
        }
        Ok(form)
    }
}
