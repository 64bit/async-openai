#[cfg(feature = "realtime")]
use crate::{error::OpenAIError, traits::AsyncTryFrom};

#[cfg(feature = "realtime")]
impl AsyncTryFrom<crate::types::realtime::RealtimeCallCreateRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(
        request: crate::types::realtime::RealtimeCallCreateRequest,
    ) -> Result<Self, Self::Error> {
        use reqwest::multipart::Part;

        // Create SDP part with content type application/sdp
        let sdp_part = Part::text(request.sdp)
            .mime_str("application/sdp")
            .map_err(|e| OpenAIError::InvalidArgument(format!("Invalid content type: {}", e)))?;

        let mut form = reqwest::multipart::Form::new().part("sdp", sdp_part);

        // Add session as JSON if present
        if let Some(session) = request.session {
            let session_json = serde_json::to_string(&session).map_err(|e| {
                OpenAIError::InvalidArgument(format!("Failed to serialize session: {}", e))
            })?;
            let session_part = Part::text(session_json)
                .mime_str("application/json")
                .map_err(|e| {
                    OpenAIError::InvalidArgument(format!("Invalid content type: {}", e))
                })?;
            form = form.part("session", session_part);
        }

        Ok(form)
    }
}
