use crate::{
    error::OpenAIError,
    types::{CreateEditRequest, CreateEditResponse},
    Client,
};

/// Given a prompt and an instruction, the model will return
/// an edited version of the prompt.
pub struct Edit;

impl Edit {
    /// Creates a new edit for the provided input, instruction, and parameters
    pub async fn create(
        client: &Client,
        request: CreateEditRequest,
    ) -> Result<CreateEditResponse, OpenAIError> {
        client.post("/edits", request).await
    }
}
