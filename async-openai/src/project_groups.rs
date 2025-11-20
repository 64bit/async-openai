use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::projects::{
        InviteProjectGroupBody, ProjectGroup, ProjectGroupDeletedResource, ProjectGroupListResource,
    },
    Client, RequestOptions,
};

/// Manage which groups have access to a project and the role they receive.
pub struct ProjectGroups<'c, C: Config> {
    client: &'c Client<C>,
    pub project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectGroups<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Lists all groups that have access to a project.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ProjectGroupListResource, OpenAIError> {
        self.client
            .get(
                format!("/organization/projects/{}/groups", self.project_id).as_str(),
                &self.request_options,
            )
            .await
    }

    /// Grants a group access to a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn add(&self, request: InviteProjectGroupBody) -> Result<ProjectGroup, OpenAIError> {
        self.client
            .post(
                format!("/organization/projects/{}/groups", self.project_id).as_str(),
                request,
                &self.request_options,
            )
            .await
    }

    /// Removes a group from a project.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn remove(&self, group_id: &str) -> Result<ProjectGroupDeletedResource, OpenAIError> {
        self.client
            .delete(
                format!(
                    "/organization/projects/{}/groups/{group_id}",
                    self.project_id
                )
                .as_str(),
                &self.request_options,
            )
            .await
    }
}
