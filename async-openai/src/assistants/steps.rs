use crate::{
    config::Config,
    error::OpenAIError,
    types::assistants::{ListRunStepsResponse, RunStepObject},
    Client, RequestOptions,
};

/// Represents a step in execution of a run.
pub struct Steps<'c, C: Config> {
    pub thread_id: String,
    pub run_id: String,
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Steps<'c, C> {
    pub fn new(client: &'c Client<C>, thread_id: &str, run_id: &str) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            run_id: run_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Retrieves a run step.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, step_id: &str) -> Result<RunStepObject, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/threads/{}/runs/{}/steps/{step_id}",
                    self.thread_id, self.run_id
                ),
                &self.request_options,
            )
            .await
    }

    /// Returns a list of run steps belonging to a run.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListRunStepsResponse, OpenAIError> {
        self.client
            .get(
                &format!("/threads/{}/runs/{}/steps", self.thread_id, self.run_id),
                &self.request_options,
            )
            .await
    }
}
