use serde::Serialize;

use crate::{
    error::OpenAIError,
    types::{
        CreateFineTuningJobRequest, FineTuningJob, ListFineTuningJobEventsResponse,
        ListPaginatedFineTuningJobsResponse,
    },
    Client,
};

/// Manage fine-tuning jobs to tailor a model to your specific training data.
///
/// Related guide: [Fine-tune models](https://platform.openai.com/docs/guides/fine-tuning)
pub struct FineTuning<'c> {
    client: &'c Client,
}

impl<'c> FineTuning<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a job that fine-tunes a specified model from a given dataset.
    ///
    /// Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.
    ///
    /// [Learn more about Fine-tuning](https://platform.openai.com/docs/guides/fine-tuning)
    pub async fn create(
        &self,
        request: CreateFineTuningJobRequest,
    ) -> Result<FineTuningJob, OpenAIError> {
        self.client.post("/fine_tuning/jobs", request).await
    }

    /// List your organization's fine-tuning jobs
    pub async fn list_paginated<Q>(
        &self,
        query: &Q,
    ) -> Result<ListPaginatedFineTuningJobsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/fine_tuning/jobs", query).await
    }

    /// Gets info about the fine-tune job.
    ///
    /// [Learn more about Fine-tuning](https://platform.openai.com/docs/guides/fine-tuning)
    pub async fn retrieve(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .get(format!("/fine_tuning/jobs/{fine_tuning_job_id}").as_str())
            .await
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel(&self, fine_tuning_job_id: &str) -> Result<FineTuningJob, OpenAIError> {
        self.client
            .post(
                format!("/fine_tuning/jobs/{fine_tuning_job_id}/cancel").as_str(),
                (),
            )
            .await
    }

    /// Get fine-grained status updates for a fine-tune job.
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
                query,
            )
            .await
    }
}
