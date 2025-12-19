use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{
        CreateVoiceConsentRequest, UpdateVoiceConsentRequest, VoiceConsentDeletedResource,
        VoiceConsentListResource, VoiceConsentResource,
    },
    Client, RequestOptions,
};

/// Voice consent API group
pub struct VoiceConsents<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> VoiceConsents<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Upload a voice consent recording.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateVoiceConsentRequest,
    ) -> Result<VoiceConsentResource, OpenAIError> {
        self.client
            .post_form("/audio/voice_consents", request, &self.request_options)
            .await
    }

    /// Returns a list of voice consent recordings.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<VoiceConsentListResource, OpenAIError> {
        self.client
            .get("/audio/voice_consents", &self.request_options)
            .await
    }

    /// Retrieves a voice consent recording.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, consent_id: &str) -> Result<VoiceConsentResource, OpenAIError> {
        self.client
            .get(
                &format!("/audio/voice_consents/{}", consent_id),
                &self.request_options,
            )
            .await
    }

    /// Updates a voice consent recording (metadata only).
    #[crate::byot(
        T0 = std::fmt::Display,
        T1 = serde::Serialize,
        R = serde::de::DeserializeOwned
    )]
    pub async fn update(
        &self,
        consent_id: &str,
        request: UpdateVoiceConsentRequest,
    ) -> Result<VoiceConsentResource, OpenAIError> {
        self.client
            .post(
                &format!("/audio/voice_consents/{}", consent_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Deletes a voice consent recording.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        consent_id: &str,
    ) -> Result<VoiceConsentDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!("/audio/voice_consents/{}", consent_id),
                &self.request_options,
            )
            .await
    }
}
