use crate::{
    error::OpenAIError, traits::AsyncTryFrom, types::containers::CreateContainerFileRequest,
    util::create_file_part,
};

impl AsyncTryFrom<CreateContainerFileRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateContainerFileRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new();

        // Either file or file_id should be provided
        if let Some(file_source) = request.file {
            let file_part = create_file_part(file_source).await?;
            form = form.part("file", file_part);
        } else if let Some(file_id) = request.file_id {
            form = form.text("file_id", file_id);
        }

        Ok(form)
    }
}
