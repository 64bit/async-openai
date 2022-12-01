use crate::{
    error::OpenAIError,
    types::{CreateEditRequest, CreateEditResponse},
    Client,
};

pub struct Edit;

impl Edit {
    pub async fn create(
        client: &Client,
        request: CreateEditRequest,
    ) -> Result<CreateEditResponse, OpenAIError> {
        client.post("/edits", request).await
    }
}
