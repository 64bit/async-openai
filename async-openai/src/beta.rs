use std::cell::OnceCell;

use crate::{
    chatkit::Chatkit,
    config::{Config, OpenAIConfig},
    Client,
};

/// Beta API accessor for beta features like ChatKit.
///
/// Use `client.beta().chatkit()` to access ChatKit APIs with the required beta header.
pub struct Beta<'c, C: Config> {
    client: &'c Client<C>,
    beta_client: OnceCell<Client<OpenAIConfig>>,
}

impl<'c> Beta<'c, OpenAIConfig> {
    /// Access ChatKit APIs.
    ///
    /// This automatically includes the `OpenAI-Beta: chatkit_beta=v1` header in all requests.
    pub fn chatkit(&self) -> Chatkit<'_, OpenAIConfig> {
        let beta_client = self.beta_client.get_or_init(|| {
            let mut beta_config = self.client.config().clone();
            beta_config = beta_config
                .with_header("OpenAI-Beta", "chatkit_beta=v1")
                .unwrap();
            Client::with_config(beta_config)
                .with_http_client(self.client.http_client().clone())
                .with_backoff(self.client.backoff().clone())
        });
        Chatkit::new(beta_client)
    }
}

impl<'c, C: Config> Beta<'c, C> {
    pub(crate) fn new_generic(client: &'c Client<C>) -> Self {
        Self {
            client,
            beta_client: OnceCell::new(),
        }
    }
}
