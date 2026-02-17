use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::skills::{
        CreateSkillVersionRequest, DeletedSkillVersionResource, SkillVersionListResource,
        SkillVersionResource,
    },
    Client, RequestOptions,
};

/// Create and manage skill versions.
pub struct SkillVersions<'c, C: Config> {
    client: &'c Client<C>,
    skill_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> SkillVersions<'c, C> {
    pub fn new(client: &'c Client<C>, skill_id: &str) -> Self {
        Self {
            client,
            skill_id: skill_id.to_string(),
            request_options: RequestOptions::new(),
        }
    }

    /// Create a new immutable skill version by uploading files.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateSkillVersionRequest,
    ) -> Result<SkillVersionResource, OpenAIError> {
        self.client
            .post_form(
                &format!("/skills/{}/versions", self.skill_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// List skill versions.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<SkillVersionListResource, OpenAIError> {
        self.client
            .get(
                &format!("/skills/{}/versions", self.skill_id),
                &self.request_options,
            )
            .await
    }

    /// Retrieve a specific skill version.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, version: &str) -> Result<SkillVersionResource, OpenAIError> {
        self.client
            .get(
                format!("/skills/{}/versions/{version}", self.skill_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Delete a skill version.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        version: &str,
    ) -> Result<DeletedSkillVersionResource, OpenAIError> {
        self.client
            .delete(
                format!("/skills/{}/versions/{version}", self.skill_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Download a skill version zip bundle.
    pub async fn content(&self, version: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                format!("/skills/{}/versions/{version}/content", self.skill_id).as_str(),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}
