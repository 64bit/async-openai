use serde::{Deserialize, Serialize};

/// Sent when a batch API request has been cancelled.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookBatchCancelled {
    /// The Unix timestamp (in seconds) of when the batch API request was cancelled.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookBatchData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a batch API request has been completed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookBatchCompleted {
    /// The Unix timestamp (in seconds) of when the batch API request was completed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookBatchData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a batch API request has expired.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookBatchExpired {
    /// The Unix timestamp (in seconds) of when the batch API request expired.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookBatchData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a batch API request has failed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookBatchFailed {
    /// The Unix timestamp (in seconds) of when the batch API request failed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookBatchData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Data payload for batch webhook events.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookBatchData {
    /// The unique ID of the batch API request.
    pub id: String,
}

/// Sent when an eval run has been canceled.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookEvalRunCanceled {
    /// The Unix timestamp (in seconds) of when the eval run was canceled.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookEvalRunData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when an eval run has failed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookEvalRunFailed {
    /// The Unix timestamp (in seconds) of when the eval run failed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookEvalRunData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when an eval run has succeeded.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookEvalRunSucceeded {
    /// The Unix timestamp (in seconds) of when the eval run succeeded.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookEvalRunData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Data payload for eval run webhook events.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookEvalRunData {
    /// The unique ID of the eval run.
    pub id: String,
}

/// Sent when a fine-tuning job has been cancelled.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookFineTuningJobCancelled {
    /// The Unix timestamp (in seconds) of when the fine-tuning job was cancelled.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookFineTuningJobData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a fine-tuning job has failed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookFineTuningJobFailed {
    /// The Unix timestamp (in seconds) of when the fine-tuning job failed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookFineTuningJobData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a fine-tuning job has succeeded.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookFineTuningJobSucceeded {
    /// The Unix timestamp (in seconds) of when the fine-tuning job succeeded.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookFineTuningJobData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Data payload for fine-tuning job webhook events.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookFineTuningJobData {
    /// The unique ID of the fine-tuning job.
    pub id: String,
}

// EventType and EventId implementations for fine-tuning job events

/// Sent when Realtime API receives an incoming SIP call.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookRealtimeCallIncoming {
    /// The Unix timestamp (in seconds) of when the model response was completed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookRealtimeCallData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Data payload for realtime call webhook events.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookRealtimeCallData {
    /// The unique ID of this call.
    pub call_id: String,

    /// Headers from the SIP INVITE, excluding SIP authorization headers.
    /// Retained names, values, repeated entries, and order are preserved.
    /// Treat these values as untrusted call metadata.
    pub sip_headers: Vec<SipHeader>,
}

/// A header from the SIP Invite.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SipHeader {
    /// Name of the SIP Header.
    pub name: String,

    /// Value of the SIP Header.
    pub value: String,
}

/// Sent when a background response has been cancelled.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookResponseCancelled {
    /// The Unix timestamp (in seconds) of when the model response was cancelled.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookResponseData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a background response has been completed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookResponseCompleted {
    /// The Unix timestamp (in seconds) of when the model response was completed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookResponseData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a background response has failed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookResponseFailed {
    /// The Unix timestamp (in seconds) of when the model response failed.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookResponseData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Sent when a background response has been interrupted.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookResponseIncomplete {
    /// The Unix timestamp (in seconds) of when the model response was interrupted.
    pub created_at: u64,

    /// The unique ID of the event.
    pub id: String,

    /// Event data payload.
    pub data: WebhookResponseData,

    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Data payload for response webhook events.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookResponseData {
    /// The unique ID of the model response.
    pub id: String,
}

/// Sent when an incoming API SIP session is available for Live acceptance. The
/// same pending session can also emit `realtime.call.incoming`; the first
/// successful Realtime or Live accept endpoint selects the runtime surface.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookLiveCallIncoming {
    /// The Unix timestamp (in seconds) of when the event was created.
    pub created_at: u64,
    /// The unique ID of the event.
    pub id: String,
    /// Event data payload.
    pub data: WebhookLiveCallIncomingData,
    /// The object of the event. Always `event`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Event data payload.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookLiveCallIncomingData {
    /// The `live_...` ID of the pending SIP session. Forward this value
    /// unchanged when accepting or rejecting the call through the Live API.
    pub session_id: String,
    /// Headers from the SIP INVITE, excluding SIP authorization headers.
    /// Retained names, values, repeated entries, and order are preserved.
    /// Treat these values as untrusted call metadata.
    pub sip_headers: Vec<WebhookLiveCallIncomingDataSipHeadersItem>,
}

/// A header from the SIP Invite.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookLiveCallIncomingDataSipHeadersItem {
    /// Name of the SIP Header.
    pub name: String,
    /// Value of the SIP Header.
    pub value: String,
}

/// Sent when an approved safety alert is available for an API project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookSafetyAlertCreated {
    /// The unique ID of the webhook event.
    pub id: String,
    /// Always `event`.
    pub object: String,
    /// The Unix timestamp in seconds when the event was created.
    pub created_at: u64,
    pub data: WebhookSafetyAlertCreatedData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookSafetyAlertCreatedData {
    /// The safety alert ID to pass to `GET /v1/safety/alerts/{id}`.
    pub id: String,
}

/// Always `safety.alert.created`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WebhookSafetyAlertCreatedType {
    #[serde(rename = "safety.alert.created")]
    SafetyAlertCreated,
}

/// Sent when an approved safety alert is available for an enterprise workspace.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookSafetyOrgAlertCreated {
    /// The unique ID of the webhook event.
    pub id: String,
    /// Always `event`.
    pub object: String,
    /// The Unix timestamp in seconds when the event was created.
    pub created_at: u64,
    pub data: WebhookSafetyOrgAlertCreatedData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookSafetyOrgAlertCreatedData {
    /// The safety alert ID to pass to `GET /v1/safety/alerts/{id}`.
    pub id: String,
}

// EventType and EventId implementations for response events

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum WebhookEvent {
    #[serde(rename = "batch.cancelled")]
    BatchCancelled(WebhookBatchCancelled),

    #[serde(rename = "batch.completed")]
    BatchCompleted(WebhookBatchCompleted),

    #[serde(rename = "batch.expired")]
    BatchExpired(WebhookBatchExpired),

    #[serde(rename = "batch.failed")]
    BatchFailed(WebhookBatchFailed),

    #[serde(rename = "eval.run.canceled")]
    EvalRunCanceled(WebhookEvalRunCanceled),

    #[serde(rename = "eval.run.failed")]
    EvalRunFailed(WebhookEvalRunFailed),

    #[serde(rename = "eval.run.succeeded")]
    EvalRunSucceeded(WebhookEvalRunSucceeded),

    #[serde(rename = "fine_tuning.job.cancelled")]
    FineTuningJobCancelled(WebhookFineTuningJobCancelled),

    #[serde(rename = "fine_tuning.job.failed")]
    FineTuningJobFailed(WebhookFineTuningJobFailed),

    #[serde(rename = "fine_tuning.job.succeeded")]
    FineTuningJobSucceeded(WebhookFineTuningJobSucceeded),

    #[serde(rename = "realtime.call.incoming")]
    RealtimeCallIncoming(WebhookRealtimeCallIncoming),

    #[serde(rename = "response.cancelled")]
    ResponseCancelled(WebhookResponseCancelled),

    #[serde(rename = "response.completed")]
    ResponseCompleted(WebhookResponseCompleted),

    #[serde(rename = "response.failed")]
    ResponseFailed(WebhookResponseFailed),

    #[serde(rename = "response.incomplete")]
    ResponseIncomplete(WebhookResponseIncomplete),

    #[serde(rename = "live.call.incoming")]
    WebhookLiveCallIncoming(WebhookLiveCallIncoming),

    #[serde(rename = "safety.alert.created")]
    WebhookSafetyAlertCreated(WebhookSafetyAlertCreated),

    #[serde(rename = "safety.org_alert.created")]
    WebhookSafetyOrgAlertCreated(WebhookSafetyOrgAlertCreated),
}

// Implement EventType trait for all event types in this file
#[cfg(feature = "_api")]
macro_rules! impl_event_type {
    ($($ty:ty => $event_type:expr),* $(,)?) => {
        $(
            impl crate::traits::EventType for $ty {
                fn event_type(&self) -> &'static str {
                    $event_type
                }
            }
        )*
    };
}

#[cfg(feature = "_api")]
macro_rules! impl_event_id {
    ($($ty:ty),* $(,)?) => {
        $(
            impl crate::traits::EventId for $ty {
                fn event_id(&self) -> &str {
                    &self.id
                }
            }
        )*
    };
}
// Use the macro to implement EventType for all webhook event structs
#[cfg(feature = "_api")]
impl_event_type! {
    WebhookLiveCallIncoming => "live.call.incoming",
    WebhookSafetyAlertCreated => "safety.alert.created",
    WebhookSafetyOrgAlertCreated => "safety.org_alert.created",
    WebhookBatchCancelled => "batch.cancelled",
    WebhookBatchCompleted => "batch.completed",
    WebhookBatchExpired => "batch.expired",
    WebhookBatchFailed => "batch.failed",
    WebhookEvalRunCanceled => "eval.run.canceled",
    WebhookEvalRunFailed => "eval.run.failed",
    WebhookEvalRunSucceeded => "eval.run.succeeded",
    WebhookFineTuningJobCancelled => "fine_tuning.job.cancelled",
    WebhookFineTuningJobFailed => "fine_tuning.job.failed",
    WebhookFineTuningJobSucceeded => "fine_tuning.job.succeeded",
    WebhookRealtimeCallIncoming => "realtime.call.incoming",
    WebhookResponseCancelled => "response.cancelled",
    WebhookResponseCompleted => "response.completed",
    WebhookResponseFailed => "response.failed",
    WebhookResponseIncomplete => "response.incomplete",
}

// Use the macro to implement EventId for all webhook event structs
#[cfg(feature = "_api")]
impl_event_id! {
    WebhookLiveCallIncoming,
    WebhookSafetyAlertCreated,
    WebhookSafetyOrgAlertCreated,
    WebhookBatchCancelled,
    WebhookBatchCompleted,
    WebhookBatchExpired,
    WebhookBatchFailed,
    WebhookEvalRunCanceled,
    WebhookEvalRunFailed,
    WebhookEvalRunSucceeded,
    WebhookFineTuningJobCancelled,
    WebhookFineTuningJobFailed,
    WebhookFineTuningJobSucceeded,
    WebhookRealtimeCallIncoming,
    WebhookResponseCancelled,
    WebhookResponseCompleted,
    WebhookResponseFailed,
    WebhookResponseIncomplete,
}

// Trait implementations for WebhookEvent enum
#[cfg(feature = "_api")]
impl crate::traits::EventType for WebhookEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::WebhookLiveCallIncoming(e) => e.event_type(),
            Self::WebhookSafetyAlertCreated(e) => e.event_type(),
            Self::WebhookSafetyOrgAlertCreated(e) => e.event_type(),
            Self::BatchCancelled(e) => e.event_type(),
            Self::BatchCompleted(e) => e.event_type(),
            Self::BatchExpired(e) => e.event_type(),
            Self::BatchFailed(e) => e.event_type(),
            Self::EvalRunCanceled(e) => e.event_type(),
            Self::EvalRunFailed(e) => e.event_type(),
            Self::EvalRunSucceeded(e) => e.event_type(),
            Self::FineTuningJobCancelled(e) => e.event_type(),
            Self::FineTuningJobFailed(e) => e.event_type(),
            Self::FineTuningJobSucceeded(e) => e.event_type(),
            Self::RealtimeCallIncoming(e) => e.event_type(),
            Self::ResponseCancelled(e) => e.event_type(),
            Self::ResponseCompleted(e) => e.event_type(),
            Self::ResponseFailed(e) => e.event_type(),
            Self::ResponseIncomplete(e) => e.event_type(),
        }
    }
}

#[cfg(feature = "_api")]
impl crate::traits::EventId for WebhookEvent {
    fn event_id(&self) -> &str {
        match self {
            Self::WebhookLiveCallIncoming(e) => e.event_id(),
            Self::WebhookSafetyAlertCreated(e) => e.event_id(),
            Self::WebhookSafetyOrgAlertCreated(e) => e.event_id(),
            Self::BatchCancelled(e) => e.event_id(),
            Self::BatchCompleted(e) => e.event_id(),
            Self::BatchExpired(e) => e.event_id(),
            Self::BatchFailed(e) => e.event_id(),
            Self::EvalRunCanceled(e) => e.event_id(),
            Self::EvalRunFailed(e) => e.event_id(),
            Self::EvalRunSucceeded(e) => e.event_id(),
            Self::FineTuningJobCancelled(e) => e.event_id(),
            Self::FineTuningJobFailed(e) => e.event_id(),
            Self::FineTuningJobSucceeded(e) => e.event_id(),
            Self::RealtimeCallIncoming(e) => e.event_id(),
            Self::ResponseCancelled(e) => e.event_id(),
            Self::ResponseCompleted(e) => e.event_id(),
            Self::ResponseFailed(e) => e.event_id(),
            Self::ResponseIncomplete(e) => e.event_id(),
        }
    }
}

impl WebhookEvent {
    /// Get the timestamp when the event was created
    pub fn created_at(&self) -> u64 {
        match self {
            Self::WebhookLiveCallIncoming(e) => e.created_at,
            Self::WebhookSafetyAlertCreated(e) => e.created_at,
            Self::WebhookSafetyOrgAlertCreated(e) => e.created_at,
            Self::BatchCancelled(w) => w.created_at,
            Self::BatchCompleted(w) => w.created_at,
            Self::BatchExpired(w) => w.created_at,
            Self::BatchFailed(w) => w.created_at,
            Self::EvalRunCanceled(w) => w.created_at,
            Self::EvalRunFailed(w) => w.created_at,
            Self::EvalRunSucceeded(w) => w.created_at,
            Self::FineTuningJobCancelled(w) => w.created_at,
            Self::FineTuningJobFailed(w) => w.created_at,
            Self::FineTuningJobSucceeded(w) => w.created_at,
            Self::RealtimeCallIncoming(w) => w.created_at,
            Self::ResponseCancelled(w) => w.created_at,
            Self::ResponseCompleted(w) => w.created_at,
            Self::ResponseFailed(w) => w.created_at,
            Self::ResponseIncomplete(w) => w.created_at,
        }
    }
}
