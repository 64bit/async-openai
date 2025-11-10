use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::finetuning::{
        CreateFineTuningCheckpointPermissionRequest, CreateFineTuningJobRequest,
        DeleteFineTuningCheckpointPermissionResponse, FineTuningJob,
        ListFineTuningCheckpointPermissionResponse, ListFineTuningJobCheckpointsResponse,
        ListFineTuningJobEventsResponse, ListPaginatedFineTuningJobsResponse,
    },
    Client,
};

/// Manage fine-tuning jobs to tailor a model to your specific training data.
///
/// Related guide: [Fine-tune models](https://platform.openai.com/docs/guides/fine-tuning)
pub struct FineTuning<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> FineTuning<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a fine-tuning job which begins the process of creating a new model from a given dataset.
    ///
    /// Response includes details of the enqueued job including job status and the name of the fine-tuned
    /// models once complete.
    ///
    /// [Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization)
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateFineTuningJobRequest,
    ) -> Result<FineTuningJob, OpenAIError> {
        self.client.post("/fine_tuning/jobs", request).await
    }

    /// List your organization's fine-tuning jobs
    #[crate::byot(T0 = serde::Serialize, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list_paginated<Q>(
        &self,
        query: &Q,
    ) -> Result<ListPaginatedFineTuningJobsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query("/fine_tuning/jobs", &query)
            .await
    }

    /// Get info about a fine-tuning job.
    ///
    /// [Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization)
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .get(format!("/fine_tuning/jobs/{fine_tuning_job_id}").as_str())
            .await
    }

    /// Immediately cancel a fine-tune job.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .post(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/cancel").as_str(),
                (),
            )
            .await
    }

    /// Pause a fine-tune job.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn pause(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .post(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/pause").as_str(),
                (),
            )
            .await
    }

    /// Resume a fine-tune job.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn resume(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .post(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/resume").as_str(),
                (),
            )
            .await
    }

    /// Get status updates for a fine-tuning job.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list_events<Q>(
        &self,
        fine_tuning_job_id: &str,
        query: &Q,
    ) -> Result<ListFineTuningJobEventsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/events").as_str(),
                &query,
            )
            .await
    }

    /// List checkpoints for a fine-tuning job.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list_checkpoints<Q>(
        &self,
        fine_tuning_job_id: &str,
        query: &Q,
    ) -> Result<ListFineTuningJobCheckpointsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints").as_str(),
                &query,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create_checkpoint_permissions(
        &self,
        fine_tuned_model_checkpoint: &str,
        request: CreateFineTuningCheckpointPermissionRequest,
    ) -> Result<ListFineTuningCheckpointPermissionResponse, OpenAIError> {
        self.client
            .post(
                format!("/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions")
                    .as_str(),
                request,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list_checkpoint_permissions<Q>(
        &self,
        fine_tuned_model_checkpoint: &str,
        query: &Q,
    ) -> Result<ListFineTuningCheckpointPermissionResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                format!("/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions")
                    .as_str(),
                &query,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete_checkpoint_permissions(
        &self,
        fine_tuned_model_checkpoint: &str,
        permission_id: &str,
    ) -> Result<DeleteFineTuningCheckpointPermissionResponse, OpenAIError> {
        self.client
            .delete(
                format!("/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id}")
                    .as_str(),
            )
            .await
    }
}
