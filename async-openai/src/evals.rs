use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    eval_runs::EvalRuns,
    types::evals::{CreateEvalRequest, DeleteEvalResponse, Eval, EvalList, UpdateEvalRequest},
    Client,
};

/// Create, manage, and run evals in the OpenAI platform. Related guide:
/// [Evals](https://platform.openai.com/docs/guides/evals)
pub struct Evals<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Evals<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// [EvalRuns] API group
    pub fn runs(&self, eval_id: &str) -> EvalRuns<'_, C> {
        EvalRuns::new(self.client, eval_id)
    }

    /// List evaluations for a project.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<EvalList, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/evals", &query).await
    }

    /// Create the structure of an evaluation that can be used to test a model's performance.
    /// An evaluation is a set of testing criteria and the config for a data source, which dictates
    /// the schema of the data used in the evaluation. After creating an evaluation, you can run it
    /// on different models and model parameters. We support several types of graders and
    /// datasources. For more information, see the [Evals guide](https://platform.openai.com/docs/guides/evals).
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: CreateEvalRequest) -> Result<Eval, OpenAIError> {
        self.client.post("/evals", request).await
    }

    /// Get an evaluation by ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, eval_id: &str) -> Result<Eval, OpenAIError> {
        self.client.get(&format!("/evals/{eval_id}")).await
    }

    /// Update certain properties of an evaluation.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        eval_id: &str,
        request: UpdateEvalRequest,
    ) -> Result<Eval, OpenAIError> {
        self.client
            .post(&format!("/evals/{eval_id}"), request)
            .await
    }

    /// Delete an evaluation.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, eval_id: &str) -> Result<DeleteEvalResponse, OpenAIError> {
        self.client.delete(&format!("/evals/{eval_id}")).await
    }
}
