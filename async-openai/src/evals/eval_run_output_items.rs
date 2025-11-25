use crate::{
    config::Config,
    error::OpenAIError,
    types::evals::{EvalRunOutputItem, EvalRunOutputItemList},
    Client, RequestOptions,
};

pub struct EvalRunOutputItems<'c, C: Config> {
    client: &'c Client<C>,
    pub eval_id: String,
    pub run_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> EvalRunOutputItems<'c, C> {
    pub fn new(client: &'c Client<C>, eval_id: &str, run_id: &str) -> Self {
        Self {
            client,
            eval_id: eval_id.into(),
            run_id: run_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    /// Get a list of output items for an evaluation run.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<EvalRunOutputItemList, OpenAIError> {
        self.client
            .get(
                &format!("/evals/{}/runs/{}/output_items", self.eval_id, self.run_id),
                &self.request_options,
            )
            .await
    }

    /// Get an evaluation run output item by ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, output_item_id: &str) -> Result<EvalRunOutputItem, OpenAIError> {
        self.client
            .get(
                &format!(
                    "/evals/{}/runs/{}/output_items/{}",
                    self.eval_id, self.run_id, output_item_id
                ),
                &self.request_options,
            )
            .await
    }
}
