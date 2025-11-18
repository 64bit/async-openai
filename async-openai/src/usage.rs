use serde::Serialize;

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
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn audio_speeches<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/audio_speeches",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get audio transcriptions usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn audio_transcriptions<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/audio_transcriptions",
                &query,
                &self.request_options,
            )
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
            .get_with_query(
                "/organization/usage/code_interpreter_sessions",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get completions usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn completions<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/completions",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get embeddings usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn embeddings<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/embeddings",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get images usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn images<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/usage/images", &query, &self.request_options)
            .await
    }

    /// Get moderations usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn moderations<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/moderations",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get vector stores usage details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn vector_stores<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                "/organization/usage/vector_stores",
                &query,
                &self.request_options,
            )
            .await
    }

    /// Get costs details for the organization.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn costs<Q>(&self, query: &Q) -> Result<UsageResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/organization/costs", &query, &self.request_options)
            .await
    }
}
