use serde::Serialize;

use crate::{config::Config, error::OpenAIError, types::admin::usage::UsageResponse, Client};

/// Manage organization usage data. Get usage details for various API endpoints including
/// completions, embeddings, images, audio, moderations, vector stores, and code interpreter sessions.
pub struct Usage<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Usage<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Get audio speeches usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn audio_speeches<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/audio_speeches", &query)
            .await
    }

    /// Get audio transcriptions usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn audio_transcriptions<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/audio_transcriptions", &query)
            .await
    }

    /// Get code interpreter sessions usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn code_interpreter_sessions<Q>(
        &self,
        query: &Q,
    ) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/code_interpreter_sessions", &query)
            .await
    }

    /// Get completions usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn completions<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/completions", &query)
            .await
    }

    /// Get embeddings usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn embeddings<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/embeddings", &query)
            .await
    }

    /// Get images usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn images<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/images", &query)
            .await
    }

    /// Get moderations usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn moderations<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/moderations", &query)
            .await
    }

    /// Get vector stores usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn vector_stores<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/vector_stores", &query)
            .await
    }

    /// Get costs details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn costs<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/costs", &query)
            .await
    }
}
