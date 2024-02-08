use base64::engine::{general_purpose, Engine};

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        Base64Embedding, CreateEmbeddingRequest, CreateEmbeddingResponse, Embedding, EncodingFormat,
    },
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
        if !matches!(request.encoding_format, Some(EncodingFormat::Base64)) {
            self.client.post("/embeddings", request).await
        } else {
            let response: CreateEmbeddingResponse<Base64Embedding> =
                self.client.post("/embeddings", request).await?;
            response.try_into()
        }
    }
}

impl TryFrom<CreateEmbeddingResponse<Base64Embedding>> for CreateEmbeddingResponse<Embedding> {
    type Error = OpenAIError;

    fn try_from(response: CreateEmbeddingResponse<Base64Embedding>) -> Result<Self, Self::Error> {
        let response = CreateEmbeddingResponse {
            model: response.model,
            object: response.object,
            usage: response.usage,
            data: response
                .data
                .into_iter()
                .map(Embedding::try_from)
                .collect::<Result<_, _>>()?,
        };
        Ok(response)
    }
}

impl TryFrom<Base64Embedding> for Embedding {
    type Error = OpenAIError;

    fn try_from(embedding: Base64Embedding) -> Result<Self, Self::Error> {
        let bytes = general_purpose::STANDARD.decode(embedding.embedding)?;
        let chunks = bytes.chunks_exact(4);
        debug_assert!(chunks.remainder().len() == 0);
        let floats = chunks
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        Ok(Embedding {
            index: embedding.index,
            object: embedding.object,
            embedding: floats,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{CreateEmbeddingResponse, Embedding, EncodingFormat};
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

    #[tokio::test]
    async fn test_base64_embedding() {
        let client = Client::new();

        const MODEL: &str = "text-embedding-ada-002";
        const INPUT: &str = "CoLoop will eat the other qual research tools...";

        let b64_request = CreateEmbeddingRequestArgs::default()
            .model(MODEL)
            .input(INPUT)
            .encoding_format(EncodingFormat::Base64)
            .build()
            .unwrap();
        let b64_response = client.embeddings().create(b64_request).await.unwrap();
        let b64_embedding = b64_response.data.into_iter().next().unwrap().embedding;

        let request = CreateEmbeddingRequestArgs::default()
            .model(MODEL)
            .input(INPUT)
            .build()
            .unwrap();
        let response = client.embeddings().create(request).await.unwrap();
        let embedding = response.data.into_iter().next().unwrap().embedding;

        assert_eq!(b64_embedding.len(), embedding.len());
        for (b64, normal) in b64_embedding.iter().zip(embedding.iter()) {
            assert!((b64 - normal).abs() < 1e-6);
        }
    }
}
