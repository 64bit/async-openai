use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        AssistantFileObject, CreateAssistantFileRequest, DeleteAssistantFileResponse,
        ListAssistantFilesResponse,
    },
    Client,
};

/// Files attached to an assistant.
pub struct AssistantFiles<'c, C: Config> {
    client: &'c Client<C>,
    pub assistant_id: String,
}

impl<'c, C: Config> AssistantFiles<'c, C> {
    pub fn new(client: &'c Client<C>, assistant_id: &str) -> Self {
        Self {
            client,
            assistant_id: assistant_id.into(),
        }
    }

    /// Create an assistant file by attaching a [File](https://platform.openai.com/docs/api-reference/files) to an [assistant](https://platform.openai.com/docs/api-reference/assistants).
    pub async fn create(
        &self,
        request: CreateAssistantFileRequest,
    ) -> Result<AssistantFileObject, OpenAIError> {
        self.client
            .post(&format!("/assistants/{}/files", self.assistant_id), request)
            .await
    }

    /// Retrieves an AssistantFile.
    pub async fn retrieve(&self, file_id: &str) -> Result<AssistantFileObject, OpenAIError> {
        self.client
            .get(&format!(
                "/assistants/{}/files/{file_id}",
                self.assistant_id
            ))
            .await
    }

    /// Delete an assistant file.
    pub async fn delete(&self, file_id: &str) -> Result<DeleteAssistantFileResponse, OpenAIError> {
        self.client
            .delete(&format!(
                "/assistants/{}/files/{file_id}",
                self.assistant_id
            ))
            .await
    }

    /// Returns a list of assistant files.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListAssistantFilesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(&format!("/assistants/{}/files", self.assistant_id), query)
            .await
    }
}
