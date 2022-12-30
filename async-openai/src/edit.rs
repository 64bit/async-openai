use crate::{
    error::OpenAIError,
    types::{CreateEditRequest, CreateEditResponse},
    Client,
};

/// Given a prompt and an instruction, the model will return
/// an edited version of the prompt.
pub struct Edits<'c> {
    client: &'c Client,
}

impl<'c> Edits<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a new edit for the provided input, instruction, and parameters
    pub async fn create(
        &self,
        request: CreateEditRequest,
    ) -> Result<CreateEditResponse, OpenAIError> {
        self.client.post("/edits", request).await
    }
}
