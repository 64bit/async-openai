use crate::{
    error::OpenAIError,
    types::{CreateFileRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile},
    util::create_file_part,
    Client,
};

/// Files are used to upload documents that can be used with features like [Fine-tuning](https://beta.openai.com/docs/api-reference/fine-tunes).
pub struct File;

impl File {
    /// Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit.
    pub async fn create(
        client: &Client,
        request: CreateFileRequest,
    ) -> Result<OpenAIFile, OpenAIError> {
        let file_part = create_file_part(&request.file.path).await?;
        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("purpose", request.purpose);
        client.post_form("/files", form).await
    }

    /// Returns a list of files that belong to the user's organization.
    pub async fn list(client: &Client) -> Result<ListFilesResponse, OpenAIError> {
        client.get("/files").await
    }

    /// Returns information about a specific file.
    pub async fn retrieve(client: &Client, file_id: &str) -> Result<OpenAIFile, OpenAIError> {
        client.get(format!("/files/{file_id}").as_str()).await
    }

    /// Delete a file.
    pub async fn delete(client: &Client, file_id: &str) -> Result<DeleteFileResponse, OpenAIError> {
        client.delete(format!("/files/{file_id}").as_str()).await
    }

    /// Returns the contents of the specified file
    pub async fn retrieve_content(client: &Client, file_id: &str) -> Result<String, OpenAIError> {
        client
            .get(format!("/files/{file_id}/content").as_str())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{CreateFileRequest, FileInput},
        Client,
    };

    use super::File;
    #[tokio::test]
    async fn test_file_mod() {
        let test_file_path = "/tmp/test.jsonl";
        let contents = "{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}
{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}";
        tokio::fs::write(test_file_path, contents).await.unwrap();

        let client = Client::new();

        let request = CreateFileRequest {
            file: FileInput::new(test_file_path),
            purpose: "fine-tune".to_owned(),
        };
        let openai_file = File::create(&client, request).await.unwrap();

        assert_eq!(openai_file.bytes, 135);
        assert_eq!(openai_file.filename, "test.jsonl");
        assert_eq!(openai_file.purpose, "fine-tune");
        //assert_eq!(openai_file.status, Some("processed".to_owned())); // uploaded or processed

        //println!("CREATE: \n{:#?}", openai_file);

        let list_files = File::list(&client).await.unwrap();

        assert_eq!(list_files.data.into_iter().last().unwrap(), openai_file);

        //println!("LIST: \n{:#?}", list_files);

        let retrieve_file = File::retrieve(&client, &openai_file.id).await.unwrap();

        // println!("RETRIEVE: \n{:#?}", retrieve_file);

        assert_eq!(retrieve_file, openai_file);

        /*
        // "To help mitigate abuse, downloading of fine-tune training files is disabled for free accounts."
        let retrieved_contents = File::retrieve_content(&client, &openai_file.id)
            .await
            .unwrap();

        //println!("CONTENTS:\n{}", retrieve_contents);

        assert_eq!(contents, retrieved_contents);
        */

        // Sleep to prevent "File is still processing. Check back later."
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        let delete_response = File::delete(&client, &openai_file.id).await.unwrap();

        // println!("DELETE: \n{:#?}", delete_response);

        assert_eq!(openai_file.id, delete_response.id);
        assert_eq!(delete_response.deleted, true);
    }
}
