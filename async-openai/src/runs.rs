use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{CreateRunRequest, ListRunsResponse, ModifyRunRequest, RunObject},
    Client,
};

pub struct Runs<'c, C: Config> {
    pub thread_id: String,
    client: &'c Client<C>,
}

impl<'c, C: Config> Runs<'c, C> {
    pub fn new(client: &'c Client<C>, thread_id: &str) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }

    /// Create a run.
    pub async fn create(&self, request: CreateRunRequest) -> Result<RunObject, OpenAIError> {
        self.client
            .post(&format!("/threads/{}/runs", self.thread_id), request)
            .await
    }

    /// Retrieves a run.
    pub async fn retrieve(&self, run_id: &str) -> Result<RunObject, OpenAIError> {
        self.client
            .get(&format!("/threads/{}/runs/{run_id}", self.thread_id))
            .await
    }

    /// Modifies a run.
    pub async fn update(
        &self,
        run_id: &str,
        request: ModifyRunRequest,
    ) -> Result<RunObject, OpenAIError> {
        self.client
            .post(
                &format!("/threads/{}/runs/{run_id}", self.thread_id),
                request,
            )
            .await
    }

    /// Returns a list of runs belonging to a thread.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListRunsResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(&format!("/threads/{}/runs", self.thread_id), query)
            .await
    }
}
