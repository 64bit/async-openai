use crate::{
    config::Config,
    error::OpenAIError,
    types::content_provenance_checks::{CreateContentProvenanceBody, ProvenanceResource},
    Client, RequestOptions,
};

/// Check files for supported OpenAI provenance signals.
pub struct ContentProvenanceChecks<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ContentProvenanceChecks<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Check an image or audio file for supported OpenAI provenance signals.
    /// [Learn more about content provenance](https://developers.openai.com/api/docs/guides/content-provenance).
    pub async fn create(
        &self,
        request: CreateContentProvenanceBody,
    ) -> Result<ProvenanceResource, OpenAIError> {
        self.client
            .post_form("/content_provenance_checks", request, &self.request_options)
            .await
    }
}
