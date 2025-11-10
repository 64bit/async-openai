use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::evals::{EvalRunOutputItem, EvalRunOutputItemList},
    Client,
};

pub struct EvalRunOutputItems<'c, C: Config> {
    client: &'c Client<C>,
    pub eval_id: String,
    pub run_id: String,
}

impl<'c, C: Config> EvalRunOutputItems<'c, C> {
    pub fn new(client: &'c Client<C>, eval_id: &str, run_id: &str) -> Self {
        Self {
            client,
            eval_id: eval_id.into(),
            run_id: run_id.into(),
        }
    }

    /// Get a list of output items for an evaluation run.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<EvalRunOutputItemList, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client
            .get_with_query(
                &format!("/evals/{}/runs/{}/output_items", self.eval_id, self.run_id),
                &query,
            )
            .await
    }

    /// Get an evaluation run output item by ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, output_item_id: &str) -> Result<EvalRunOutputItem, OpenAIError> {
        self.client
            .get(&format!(
                "/evals/{}/runs/{}/output_items/{}",
                self.eval_id, self.run_id, output_item_id
            ))
            .await
    }
}
