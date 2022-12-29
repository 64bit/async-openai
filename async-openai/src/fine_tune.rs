use crate::{
    error::OpenAIError,
    types::{
        CreateFineTuneRequest, FineTune, FineTuneEventsResponseStream, ListFineTuneEventsResponse,
        ListFineTuneResponse,
    },
    Client,
};

/// Manage fine-tuning jobs to tailor a model to your specific training data.
///
/// Related guide: [Fine-tune models](https://beta.openai.com/docs/guides/fine-tuning)
pub struct FineTunes;

impl FineTunes {
    /// Creates a job that fine-tunes a specified model from a given dataset.
    ///
    /// Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.
    ///
    /// [Learn more about Fine-tuning](https://beta.openai.com/docs/guides/fine-tuning)
    pub async fn create(
        client: &Client,
        request: CreateFineTuneRequest,
    ) -> Result<FineTune, OpenAIError> {
        client.post("/fine-tunes", request).await
    }

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

    /// Get fine-grained status updates for a fine-tune job.
    ///
    /// Stream fine tuning events. [FineTuneEventsResponseStream] is a parsed SSE
    /// stream until a \[DONE\] is received from server.
    pub async fn list_events_stream(
        client: &Client,
        fine_tune_id: &str,
    ) -> Result<FineTuneEventsResponseStream, OpenAIError> {
        Ok(client
            .get_stream(
                format!("/fine-tunes/{fine_tune_id}/events").as_str(),
                &[("stream", true)],
            )
            .await)
    }
}
