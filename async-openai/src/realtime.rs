use crate::{
    config::Config,
    error::OpenAIError,
    types::realtime::{
        RealtimeCallAcceptRequest, RealtimeCallCreateRequest, RealtimeCallCreateResponse,
        RealtimeCallReferRequest, RealtimeCallRejectRequest, RealtimeCreateClientSecretRequest,
        RealtimeCreateClientSecretResponse,
    },
    Client,
};

/// Realtime API for creating sessions, managing calls, and handling WebRTC connections.
/// Related guide: [Realtime API](https://platform.openai.com/docs/guides/realtime)
pub struct Realtime<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Realtime<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
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
            .post_form_raw("/realtime/calls", request)
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
    pub async fn accept_call(
        &self,
        call_id: &str,
        request: RealtimeCallAcceptRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(&format!("/realtime/calls/{}/accept", call_id), request)
            .await
    }

    /// End an active Realtime API call, whether it was initiated over SIP or WebRTC.
    pub async fn hangup_call(&self, call_id: &str) -> Result<(), OpenAIError> {
        self.client
            .post(&format!("/realtime/calls/{}/hangup", call_id), ())
            .await
    }

    /// Transfer a SIP call to a new destination using the Realtime API.
    pub async fn refer_call(
        &self,
        call_id: &str,
        request: RealtimeCallReferRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(&format!("/realtime/calls/{}/refer", call_id), request)
            .await
    }

    /// Decline an incoming SIP call handled by the Realtime API.
    pub async fn reject_call(
        &self,
        call_id: &str,
        request: Option<RealtimeCallRejectRequest>,
    ) -> Result<(), OpenAIError> {
        self.client
            .post(
                &format!("/realtime/calls/{}/reject", call_id),
                request.unwrap_or_default(),
            )
            .await
    }

    /// Create a Realtime client secret with an associated session configuration.
    pub async fn create_client_secret(
        &self,
        request: RealtimeCreateClientSecretRequest,
    ) -> Result<RealtimeCreateClientSecretResponse, OpenAIError> {
        self.client.post("/realtime/client_secrets", request).await
    }
}
