use bytes::Bytes;

use crate::{
    config::Config,
    error::OpenAIError,
    types::live::{
        LiveCallAcceptRequest, LiveCallReferRequest, LiveCallRejectRequest, LiveCreateResponse,
        LiveForkRequest, LiveSessionCreateRequest, LiveSessionCreateResponse,
    },
    Client, RequestOptions,
};

/// Create and control Live WebRTC and SIP sessions.
pub struct Live<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Live<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create a Live session.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: LiveSessionCreateRequest,
    ) -> Result<LiveSessionCreateResponse, OpenAIError> {
        self.client
            .post("/live/sessions", request, &self.request_options)
            .await
    }

    /// Accept an incoming SIP call.
    pub async fn accept(
        &self,
        session_id: &str,
        request: LiveCallAcceptRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post_raw(
                &format!("/live/sessions/{session_id}/accept"),
                request,
                &self.request_options,
            )
            .await
            .map(|_| ())
    }

    /// Fork a stored Live session onto a new WebRTC connection.
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn fork(
        &self,
        session_id: &str,
        request: LiveForkRequest,
    ) -> Result<LiveCreateResponse, OpenAIError> {
        self.client
            .post(
                &format!("/live/sessions/{session_id}/fork"),
                request,
                &self.request_options,
            )
            .await
    }

    /// End a SIP call.
    pub async fn hangup(&self, session_id: &str) -> Result<(), OpenAIError> {
        self.client
            .post_raw(
                &format!("/live/sessions/{session_id}/hangup"),
                serde_json::json!({}),
                &self.request_options,
            )
            .await
            .map(|_| ())
    }

    /// Transfer a SIP call to another destination.
    pub async fn refer(
        &self,
        session_id: &str,
        request: LiveCallReferRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post_raw(
                &format!("/live/sessions/{session_id}/refer"),
                request,
                &self.request_options,
            )
            .await
            .map(|_| ())
    }

    /// Reject an incoming SIP call.
    pub async fn reject(
        &self,
        session_id: &str,
        request: LiveCallRejectRequest,
    ) -> Result<(), OpenAIError> {
        self.client
            .post_raw(
                &format!("/live/sessions/{session_id}/reject"),
                request,
                &self.request_options,
            )
            .await
            .map(|_| ())
    }

    /// Download a Live session recording.
    pub async fn content(&self, session_id: &str) -> Result<Bytes, OpenAIError> {
        self.client
            .get_raw(
                &format!("/live/sessions/{session_id}/content"),
                &self.request_options,
            )
            .await
            .map(|(bytes, _)| bytes)
    }
}
