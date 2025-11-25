use crate::{
    config::Config, error::OpenAIError, types::admin::usage::UsageResponse, Client, RequestOptions,
};

/// Manage organization usage data. Get usage details for various API endpoints including
/// completions, embeddings, images, audio, moderations, vector stores, and code interpreter sessions.
pub struct Usage<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Usage<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Get audio speeches usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn audio_speeches(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/audio_speeches", &self.request_options)
            .await
    }

    /// Get audio transcriptions usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn audio_transcriptions(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get(
                "/organization/usage/audio_transcriptions",
                &self.request_options,
            )
            .await
    }

    /// Get code interpreter sessions usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn code_interpreter_sessions(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get(
                "/organization/usage/code_interpreter_sessions",
                &self.request_options,
            )
            .await
    }

    /// Get completions usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn completions(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/completions", &self.request_options)
            .await
    }

    /// Get embeddings usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn embeddings(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/embeddings", &self.request_options)
            .await
    }

    /// Get images usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn images(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/images", &self.request_options)
            .await
    }

    /// Get moderations usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn moderations(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/moderations", &self.request_options)
            .await
    }

    /// Get vector stores usage details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn vector_stores(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/usage/vector_stores", &self.request_options)
            .await
    }

    /// Get costs details for the organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn costs(&self) -> Result<UsageResponse, OpenAIError> {
        self.client
            .get("/organization/costs", &self.request_options)
            .await
    }
}
