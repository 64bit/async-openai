use crate::{
    config::Config,
    error::OpenAIError,
    types::evals::{CreateEvalRunRequest, DeleteEvalRunResponse, EvalRun, EvalRunList},
    Client, EvalRunOutputItems, RequestOptions,
};

pub struct EvalRuns<'c, C: Config> {
    client: &'c Client<C>,
    pub eval_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> EvalRuns<'c, C> {
    pub fn new(client: &'c Client<C>, eval_id: &str) -> Self {
        Self {
            client,
            eval_id: eval_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// [EvalRunOutputItems] API group
    pub fn output_items(&self, run_id: &str) -> EvalRunOutputItems<'_, C> {
        EvalRunOutputItems::new(self.client, &self.eval_id, run_id)
    }

    /// Get a list of runs for an evaluation.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<EvalRunList, OpenAIError> {
        self.client
            .get(
                &format!("/evals/{}/runs", self.eval_id),
                &self.request_options,
            )
            .await
    }

    /// Kicks off a new run for a given evaluation.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: CreateEvalRunRequest) -> Result<EvalRun, OpenAIError> {
        self.client
            .post(
                &format!("/evals/{}/runs", self.eval_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Get an evaluation run by ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, run_id: &str) -> Result<EvalRun, OpenAIError> {
        self.client
            .get(
                &format!("/evals/{}/runs/{}", self.eval_id, run_id),
                &self.request_options,
            )
            .await
    }

    /// Cancel an ongoing evaluation run.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, run_id: &str) -> Result<EvalRun, OpenAIError> {
        self.client
            .post(
                &format!("/evals/{}/runs/{}", self.eval_id, run_id),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
    }

    /// Delete an eval run.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, run_id: &str) -> Result<DeleteEvalRunResponse, OpenAIError> {
        self.client
            .delete(
                &format!("/evals/{}/runs/{}", self.eval_id, run_id),
                &self.request_options,
            )
            .await
    }
}
