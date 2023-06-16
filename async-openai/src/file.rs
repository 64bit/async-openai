use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateFileRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile},
    util::create_file_part,
    Client,
};

/// Files are used to upload documents that can be used with features like [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tunes).
pub struct Files<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Files<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit.
    pub async fn create(&self, request: CreateFileRequest) -> Result<OpenAIFile, OpenAIError> {
        let file_part = create_file_part(&request.file.path).await?;
        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("purpose", request.purpose);
        self.client.post_form("/files", form).await
    }

    /// Returns a list of files that belong to the user's organization.
    pub async fn list(&self) -> Result<ListFilesResponse, OpenAIError> {
        self.client.get("/files").await
    }

    /// Returns information about a specific file.
    pub async fn retrieve(&self, file_id: &str) -> Result<OpenAIFile, OpenAIError> {
        self.client.get(format!("/files/{file_id}").as_str()).await
    }

    /// Delete a file.
    pub async fn delete(&self, file_id: &str) -> Result<DeleteFileResponse, OpenAIError> {
        self.client
            .delete(format!("/files/{file_id}").as_str())
            .await
    }

    /// Returns the contents of the specified file
    pub async fn retrieve_content(&self, file_id: &str) -> Result<String, OpenAIError> {
        self.client
            .get(format!("/files/{file_id}/content").as_str())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::CreateFileRequestArgs, Client};

    #[tokio::test]
    async fn test_file_mod() {
        let test_file_path = "/tmp/test.jsonl";
        let contents = concat!(
            "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}\n", // \n is to make it valid jsonl
            "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}"
        );

        tokio::fs::write(test_file_path, contents).await.unwrap();

        let client = Client::new();

        let request = CreateFileRequestArgs::default()
            .file(test_file_path)
            .purpose("fine-tune")
            .build()
            .unwrap();

        let openai_file = client.files().create(request).await.unwrap();

        assert_eq!(openai_file.bytes, 135);
        assert_eq!(openai_file.filename, "test.jsonl");
        assert_eq!(openai_file.purpose, "fine-tune");

        //assert_eq!(openai_file.status, Some("processed".to_owned())); // uploaded or processed

        let list_files = client.files().list().await.unwrap();

        assert_eq!(list_files.data.into_iter().last().unwrap(), openai_file);

        let retrieved_file = client.files().retrieve(&openai_file.id).await.unwrap();

        assert_eq!(openai_file.created_at, retrieved_file.created_at);
        assert_eq!(openai_file.bytes, retrieved_file.bytes);
        assert_eq!(openai_file.filename, retrieved_file.filename);
        assert_eq!(openai_file.purpose, retrieved_file.purpose);

        /*
        // "To help mitigate abuse, downloading of fine-tune training files is disabled for free accounts."
        let retrieved_contents = client.files().retrieve_content(&openai_file.id)
            .await
            .unwrap();

        assert_eq!(contents, retrieved_contents);
        */

        // Sleep to prevent "File is still processing. Check back later."
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        let delete_response = client.files().delete(&openai_file.id).await.unwrap();

        assert_eq!(openai_file.id, delete_response.id);
        assert!(delete_response.deleted);
    }
}
