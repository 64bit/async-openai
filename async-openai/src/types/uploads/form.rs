use crate::{
    error::OpenAIError, traits::AsyncTryFrom, types::uploads::AddUploadPartRequest,
    util::create_file_part,
};

impl AsyncTryFrom<AddUploadPartRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: AddUploadPartRequest) -> Result<Self, Self::Error> {
        let file_part = create_file_part(request.data).await?;
        let form = reqwest::multipart::Form::new().part("data", file_part);
        Ok(form)
    }
}
