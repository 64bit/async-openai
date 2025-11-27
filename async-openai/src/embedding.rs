use crate::{
    config::Config,
    error::OpenAIError,
    types::embeddings::{
        CreateBase64EmbeddingResponse, CreateEmbeddingRequest, CreateEmbeddingResponse,
    },
    Client, RequestOptions,
};

#[cfg(not(feature = "byot"))]
use crate::types::embeddings::EncodingFormat;

/// Get a vector representation of a given input that can be easily
/// consumed by machine learning models and algorithms.
///
/// Related guide: [Embeddings](https://platform.openai.com/docs/guides/embeddings)
pub struct Embeddings<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Embeddings<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates an embedding vector representing the input text.
    ///
    /// byot: In serialized `request` you must ensure "encoding_format" is not "base64"
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateEmbeddingResponse, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if matches!(request.encoding_format, Some(EncodingFormat::Base64)) {
                return Err(OpenAIError::InvalidArgument(
                    "When encoding_format is base64, use Embeddings::create_base64".into(),
                ));
            }
        }
        self.client
            .post("/embeddings", request, &self.request_options)
            .await
    }

    /// Creates an embedding vector representing the input text.
    ///
    /// The response will contain the embedding in base64 format.
    ///
    /// byot: In serialized `request` you must ensure "encoding_format" is "base64"
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create_base64(
        &self,
        request: CreateEmbeddingRequest,
    ) -> Result<CreateBase64EmbeddingResponse, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if !matches!(request.encoding_format, Some(EncodingFormat::Base64)) {
                return Err(OpenAIError::InvalidArgument(
                    "When encoding_format is not base64, use Embeddings::create".into(),
                ));
            }
        }
        self.client
            .post("/embeddings", request, &self.request_options)
            .await
    }
}

#[cfg(all(test, feature = "embedding"))]
mod tests {
    use crate::error::OpenAIError;
    use crate::types::embeddings::{CreateEmbeddingResponse, Embedding, EncodingFormat};
    use crate::{types::embeddings::CreateEmbeddingRequestArgs, Client};

    #[tokio::test]
    async fn test_embedding_string() {
        let client = Client::new();

        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-3-small")
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
            .model("text-embedding-3-small")
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
            .model("text-embedding-3-small")
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
            .model("text-embedding-3-small")
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
            .model("text-embedding-3-small")
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
    async fn test_cannot_use_base64_encoding_with_normal_create_request() {
        let client = Client::new();

        const MODEL: &str = "text-embedding-3-small";
        const INPUT: &str = "You shall not pass.";

        let b64_request = CreateEmbeddingRequestArgs::default()
            .model(MODEL)
            .input(INPUT)
            .encoding_format(EncodingFormat::Base64)
            .build()
            .unwrap();
        let b64_response = client.embeddings().create(b64_request).await;
        assert!(matches!(b64_response, Err(OpenAIError::InvalidArgument(_))));
    }

    #[tokio::test]
    async fn test_embedding_create_base64() {
        let client = Client::new();

        const MODEL: &str = "text-embedding-3-small";
        const INPUT: &str = "a head full of dreams";

        let b64_request = CreateEmbeddingRequestArgs::default()
            .model(MODEL)
            .input(INPUT)
            .encoding_format(EncodingFormat::Base64)
            .build()
            .unwrap();
        let b64_response = client
            .embeddings()
            .create_base64(b64_request)
            .await
            .unwrap();
        let b64_embedding = b64_response.data.into_iter().next().unwrap().embedding;
        let b64_embedding: Vec<f32> = b64_embedding.into();

        let request = CreateEmbeddingRequestArgs::default()
            .model(MODEL)
            .input(INPUT)
            .build()
            .unwrap();
        let response = client.embeddings().create(request).await.unwrap();
        let embedding = response.data.into_iter().next().unwrap().embedding;

        assert_eq!(b64_embedding.len(), embedding.len());
    }
}
