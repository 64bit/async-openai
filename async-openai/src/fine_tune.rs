use crate::{
    error::OpenAIError,
    types::{FineTune, ListFineTuneEventsResponse, ListFineTuneResponse},
    Client,
};

/// Manage fine-tuning jobs to tailor a model to your specific training data.
///
/// Related guide: [Fine-tune models](https://beta.openai.com/docs/guides/fine-tuning)
pub struct FineTunes;

impl FineTunes {
    /// List your organization's fine-tuning jobs
    pub async fn list(client: &Client) -> Result<ListFineTuneResponse, OpenAIError> {
        client.get("/fine-tunes").await
    }

    /// Gets info about the fine-tune job.
    ///
    /// [Learn more about Fine-tuning](https://beta.openai.com/docs/guides/fine-tuning)
    pub async fn retrieve(client: &Client, fine_tune_id: &str) -> Result<FineTune, OpenAIError> {
        client
            .get(format!("/fine-tunes/{fine_tune_id}").as_str())
            .await
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel(client: &Client, fine_tune_id: &str) -> Result<FineTune, OpenAIError> {
        client
            .post(format!("/fine-tunes/{fine_tune_id}/cancel").as_str(), ())
            .await
    }

    /// Get fine-grained status updates for a fine-tune job.
    pub async fn list_events(
        client: &Client,
        fine_tune_id: &str,
    ) -> Result<ListFineTuneEventsResponse, OpenAIError> {
        client
            .get(format!("/fine-tunes/{fine_tune_id}/events").as_str())
            .await
    }
}
