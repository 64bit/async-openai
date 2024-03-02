use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateEmbeddingRequest, CreateEmbeddingResponse},
    Client,
};

/// Get a vector representation of a given input that can be easily
/// consumed by machine learning models and algorithms.
///
/// Related guide: [Embeddings](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings)
pub struct Embeddings<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Embeddings<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates an embedding vector representing the input text.
    pub async fn create(
        &self,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateEmbeddingResponse, OpenAIError> {
        self.client.post("/embeddings", request).await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{CreateEmbeddingResponse, Embedding};
    use crate::{types::CreateEmbeddingRequestArgs, Client};

    #[tokio::test]
    async fn test_embedding_string() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input("The food was delicious and the waiter...")
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_string_array() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input(["The food was delicious", "The waiter was good"])
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_integer_array() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input([1, 2, 3])
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_array_of_integer_array_matrix() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input([[1, 2, 3], [4, 5, 6], [7, 8, 10]])
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_array_of_integer_array() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input([vec![1, 2, 3], vec![4, 5, 6, 7], vec![7, 8, 10, 11, 100257]])
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_embedding_with_reduced_dimensions() {
        let client = Client::new();
        let dimensions = 256u32;
        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-3-small")
            .input("The food was delicious and the waiter...")
            .dimensions(dimensions)
            .build()
            .unwrap();

        let response = client.embeddings().create(request).await;

        assert!(response.is_ok());

        let CreateEmbeddingResponse { mut data, .. } = response.unwrap();
        assert_eq!(data.len(), 1);
        let Embedding { embedding, .. } = data.pop().unwrap();
        assert_eq!(embedding.len(), dimensions as usize);
    }
}
