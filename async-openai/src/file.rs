use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::files::{CreateFileRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile},
    Client, RequestOptions,
};

/// Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
pub struct Files<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Files<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Upload a file that can be used across various endpoints. Individual files can be up to 512 MB, and the size of all files uploaded by one organization can be up to 1 TB.
    ///
    /// The Assistants API supports files up to 2 million tokens and of specific file types. See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) for details.
    ///
    /// The Fine-tuning API only supports `.jsonl` files. The input also has certain required formats for fine-tuning [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input) or [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) models.
    ///
    /// The Batch API only supports `.jsonl` files up to 200 MB in size. The input also has a specific required [format](https://platform.openai.com/docs/api-reference/batch/request-input).
    ///
    /// Please [contact us](https://help.openai.com/) if you need to increase these storage limits.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(&self, request: CreateFileRequest) -> Result<OpenAIFile, OpenAIError> {
        self.client
            .post_form("/files", request, &self.request_options)
            .await
    }

    /// Returns a list of files that belong to the user's organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListFilesResponse, OpenAIError> {
        self.client.get("/files", &self.request_options).await
    }

    /// Returns information about a specific file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, file_id: &str) -> Result<OpenAIFile, OpenAIError> {
        self.client
            .get(format!("/files/{file_id}").as_str(), &self.request_options)
            .await
    }

    /// Delete a file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, file_id: &str) -> Result<DeleteFileResponse, OpenAIError> {
        self.client
            .delete(format!("/files/{file_id}").as_str(), &self.request_options)
            .await
    }

    /// Returns the contents of the specified file
    pub async fn content(&self, file_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                format!("/files/{file_id}/content").as_str(),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}

#[cfg(all(test, feature = "file"))]
mod tests {
    use crate::{
        traits::RequestOptionsBuilder,
        types::files::{
            CreateFileRequestArgs, FileExpirationAfter, FileExpirationAfterAnchor, FilePurpose,
        },
        Client,
    };

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
            .purpose(FilePurpose::FineTune)
            .expires_after(FileExpirationAfter {
                anchor: FileExpirationAfterAnchor::CreatedAt,
                seconds: 3600,
            })
            .build()
            .unwrap();

        let openai_file = client.files().create(request).await.unwrap();

        assert_eq!(openai_file.bytes, 135);
        assert_eq!(openai_file.filename, "test.jsonl");
        //assert_eq!(openai_file.purpose, "fine-tune");

        //assert_eq!(openai_file.status, Some("processed".to_owned())); // uploaded or processed
        let query = [("purpose", "fine-tune")];

        let list_files = client.files().query(&query).unwrap().list().await.unwrap();

        assert_eq!(list_files.data.into_iter().last().unwrap(), openai_file);

        let retrieved_file = client.files().retrieve(&openai_file.id).await.unwrap();

        assert_eq!(openai_file.created_at, retrieved_file.created_at);
        assert_eq!(openai_file.bytes, retrieved_file.bytes);
        assert_eq!(openai_file.filename, retrieved_file.filename);
        assert_eq!(openai_file.purpose, retrieved_file.purpose);
        assert_eq!(openai_file.expires_at, retrieved_file.expires_at);

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
