use crate::{
    error::OpenAIError,
    types::{CreateEmbeddingRequest, CreateEmbeddingResponse},
    Client,
};

/// Get a vector representation of a given input that can be easily
/// consumed by machine learning models and algorithms.
///
/// Related guide: [Embeddings](https://beta.openai.com/docs/guides/embeddings/what-are-embeddings)
pub struct Embeddings;

impl Embeddings {
    /// Creates an embedding vector representing the input text.
    pub async fn create(
        client: &Client,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateEmbeddingResponse, OpenAIError> {
        client.post("/embeddings", request).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::CreateEmbeddingRequest, Client, Embeddings};

    #[tokio::test]
    async fn test_embedding_string() {
        let client = Client::new();
        let request = CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_owned(),
            input: crate::types::EmbeddingInput::String(
                "The food was delicious and the waiter...".to_owned(),
            ),
            ..Default::default()
        };

        let response = Embeddings::create(&client, request).await;

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_embedding_string_array() {
        let client = Client::new();
        let request = CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_owned(),
            input: crate::types::EmbeddingInput::StringArray(vec![
                "The food was delicious".to_owned(),
                "The waiter was good".to_owned(),
            ]),
            ..Default::default()
        };

        let response = Embeddings::create(&client, request).await;

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_embedding_integer_array() {
        let client = Client::new();
        let request = CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_owned(),
            input: crate::types::EmbeddingInput::IntegerArray(vec![1, 2, 3]),
            ..Default::default()
        };

        let response = Embeddings::create(&client, request).await;

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_embedding_array_of_integer_array() {
        let client = Client::new();
        let request = CreateEmbeddingRequest {
            model: "text-embedding-ada-002".to_owned(),
            input: crate::types::EmbeddingInput::ArrayOfIntegerArray(vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ]),
            ..Default::default()
        };

        let response = Embeddings::create(&client, request).await;

        println!("{:#?}", response);
    }
}
