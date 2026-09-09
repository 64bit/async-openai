use crate::{
    error::OpenAIError, traits::AsyncTryFrom,
    types::content_provenance_checks::CreateContentProvenanceBody, util::create_file_part,
};

impl AsyncTryFrom<CreateContentProvenanceBody> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateContentProvenanceBody) -> Result<Self, Self::Error> {
        let file = create_file_part(request.file).await?;
        Ok(Self::new().part("file", file))
    }
}
