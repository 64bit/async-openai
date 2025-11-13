use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::audio::{
        CreateTranslationRequest, CreateTranslationResponseJson,
        CreateTranslationResponseVerboseJson,
    },
    Client,
};

pub struct Translations<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Translations<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Translates audio into English.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponseJson, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }

    /// Translates audio into English.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create_verbose_json(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<CreateTranslationResponseVerboseJson, OpenAIError> {
        self.client.post_form("/audio/translations", request).await
    }

    /// Transcribes audio into the input language.
    pub async fn create_raw(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .post_form_raw("/audio/translations", request)
            .await?;
        Ok(bytes)
    }
}
