use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::skills::{
        CreateSkillRequest, DeletedSkillResource, SetDefaultSkillVersionRequest, SkillListResource,
        SkillResource,
    },
    Client, RequestOptions, SkillVersions,
};

pub struct Skills<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Skills<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// [SkillVersions] API group
    pub fn versions(&self, skill_id: &str) -> SkillVersions<'_, C> {
        SkillVersions::new(self.client, skill_id)
    }

    /// Create a new skill by uploading files.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateSkillRequest,
    ) -> Result<SkillResource, OpenAIError> {
        self.client
            .post_form("/skills", request, &self.request_options)
            .await
    }

    /// List all skills.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<SkillListResource, OpenAIError> {
        self.client.get("/skills", &self.request_options).await
    }

    /// Retrieve a skill by its ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, skill_id: &str) -> Result<SkillResource, OpenAIError> {
        self.client
            .get(
                format!("/skills/{skill_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Update the default version pointer for a skill.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        skill_id: &str,
        request: SetDefaultSkillVersionRequest,
    ) -> Result<SkillResource, OpenAIError> {
        self.client
            .post(
                format!("/skills/{skill_id}").as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Delete a skill by its ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, skill_id: &str) -> Result<DeletedSkillResource, OpenAIError> {
        self.client
            .delete(
                format!("/skills/{skill_id}").as_str(),
                &self.request_options,
            )
            .await
    }

    /// Download a skill zip bundle by its ID.
    pub async fn content(&self, skill_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                format!("/skills/{skill_id}/content").as_str(),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}
