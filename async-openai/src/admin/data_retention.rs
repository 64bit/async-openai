use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::data_retention::{
        OrganizationDataRetention, ProjectDataRetention, UpdateOrganizationDataRetentionBody,
        UpdateProjectDataRetentionBody,
    },
    Client, RequestOptions,
};

/// Manage organization data-retention controls.
pub struct DataRetention<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> DataRetention<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Retrieves organization data-retention controls.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<OrganizationDataRetention, OpenAIError> {
        self.client
            .get("/organization/data_retention", &self.request_options)
            .await
    }

    /// Updates organization data-retention controls.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateOrganizationDataRetentionBody,
    ) -> Result<OrganizationDataRetention, OpenAIError> {
        self.client
            .post(
                "/organization/data_retention",
                request,
                &self.request_options,
            )
            .await
    }
}

/// Manage data-retention controls for one project.
pub struct ProjectDataRetentionSettings<'c, C: Config> {
    client: &'c Client<C>,
    project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectDataRetentionSettings<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Retrieves project data-retention controls.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectDataRetention, OpenAIError> {
        self.client
            .get(
                &format!("/organization/projects/{}/data_retention", self.project_id),
                &self.request_options,
            )
            .await
    }

    /// Updates project data-retention controls.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateProjectDataRetentionBody,
    ) -> Result<ProjectDataRetention, OpenAIError> {
        self.client
            .post(
                &format!("/organization/projects/{}/data_retention", self.project_id),
                request,
                &self.request_options,
            )
            .await
    }
}
