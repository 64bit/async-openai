use crate::{
    config::Config,
    error::OpenAIError,
    types::realtime::{
        translation::{
            RealtimeTranslationClientSecretCreateRequest,
            RealtimeTranslationClientSecretCreateResponse,
        },
        RealtimeCallAcceptRequest, RealtimeCallCreateRequest, RealtimeCallCreateResponse,
        RealtimeCallReferRequest, RealtimeCallRejectRequest, RealtimeCreateClientSecretRequest,
        RealtimeCreateClientSecretResponse,
    },
    Client, RequestOptions,
};

/// Realtime API for creating sessions, managing calls, and handling WebRTC connections.
/// Related guide: [Realtime API](https://platform.openai.com/docs/guides/realtime)
pub struct Realtime<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Realtime<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// call [RealtimeTranslations] group APIs
    pub fn translations(&self) -> RealtimeTranslations<'_, C> {
        RealtimeTranslations::new(self.client)
    }

    /// Create a new Realtime API call over WebRTC and receive the SDP answer needed
    /// to complete the peer connection.
    ///
    /// Returns the SDP answer in the response body and the call ID in the Location header.
    pub async fn create_call(
        &self,
        request: RealtimeCallCreateRequest,
    ) -> Result<RealtimeCallCreateResponse, OpenAIError> {
        let (bytes, headers) = self
            .client
            .post_form_raw("/realtime/calls", request, &self.request_options)
            .await?;

        // Extract Location header
        let location = headers
            .get("location")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if location.is_none() {
            tracing::warn!("Location header not found in Realtime call creation response");
        }

        // Use from_utf8_lossy to handle any invalid UTF-8 bytes in SDP
        let sdp = String::from_utf8_lossy(&bytes).into_owned();

        Ok(RealtimeCallCreateResponse { sdp, location })
    }

    /// Accept an incoming SIP call and configure the realtime session that will
    /// handle the call.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn accept_call(
        &self,
        call_id: &str,
        request: RealtimeCallAcceptRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(
                &format!("/realtime/calls/{}/accept", call_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// End an active Realtime API call, whether it was initiated over SIP or WebRTC.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn hangup_call(&self, call_id: &str) -> Result<(), OpenAIError> {
        self.client
            .post(
                &format!("/realtime/calls/{}/hangup", call_id),
                (),
                &self.request_options,
            )
            .await
    }

    /// Transfer a SIP call to a new destination using the Realtime API.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn refer_call(
        &self,
        call_id: &str,
        request: RealtimeCallReferRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(
                &format!("/realtime/calls/{}/refer", call_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Decline an incoming SIP call handled by the Realtime API.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn reject_call(
        &self,
        call_id: &str,
        request: RealtimeCallRejectRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(
                &format!("/realtime/calls/{}/reject", call_id),
                request,
                &self.request_options,
            )
            .await
    }

    /// Create a Realtime client secret with an associated session configuration.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create_client_secret(
        &self,
        request: RealtimeCreateClientSecretRequest,
    ) -> Result<RealtimeCreateClientSecretResponse, OpenAIError> {
        self.client
            .post("/realtime/client_secrets", request, &self.request_options)
            .await
    }
}

/// Realtime translations API. Translation sessions continuously translate input
/// audio into the configured output language.
pub struct RealtimeTranslations<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> RealtimeTranslations<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create a Realtime translation client secret with an associated translation session configuration.
    ///
    /// Client secrets are short-lived tokens that can be passed to a client
    /// app, such as a web frontend or mobile client, which grants access to the
    /// Realtime Translation API without leaking your main API key. You can configure a
    /// custom TTL for each client secret.
    ///
    /// Returns the created client secret and the effective translation session
    /// object.
    ///
    /// The client secret is a string that looks like `ek_1234`.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create_client_secret(
        &self,
        request: RealtimeTranslationClientSecretCreateRequest,
    ) -> Result<RealtimeTranslationClientSecretCreateResponse, OpenAIError> {
        self.client
            .post(
                "/realtime/translations/client_secrets",
                request,
                &self.request_options,
            )
            .await
    }
}
