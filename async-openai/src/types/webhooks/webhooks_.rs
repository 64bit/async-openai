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

    /// Headers from the SIP Invite.
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
            WebhookEvent::BatchCancelled(e) => e.event_type(),
            WebhookEvent::BatchCompleted(e) => e.event_type(),
            WebhookEvent::BatchExpired(e) => e.event_type(),
            WebhookEvent::BatchFailed(e) => e.event_type(),
            WebhookEvent::EvalRunCanceled(e) => e.event_type(),
            WebhookEvent::EvalRunFailed(e) => e.event_type(),
            WebhookEvent::EvalRunSucceeded(e) => e.event_type(),
            WebhookEvent::FineTuningJobCancelled(e) => e.event_type(),
            WebhookEvent::FineTuningJobFailed(e) => e.event_type(),
            WebhookEvent::FineTuningJobSucceeded(e) => e.event_type(),
            WebhookEvent::RealtimeCallIncoming(e) => e.event_type(),
            WebhookEvent::ResponseCancelled(e) => e.event_type(),
            WebhookEvent::ResponseCompleted(e) => e.event_type(),
            WebhookEvent::ResponseFailed(e) => e.event_type(),
            WebhookEvent::ResponseIncomplete(e) => e.event_type(),
        }
    }
}

#[cfg(feature = "_api")]
impl crate::traits::EventId for WebhookEvent {
    fn event_id(&self) -> &str {
        match self {
            WebhookEvent::BatchCancelled(e) => e.event_id(),
            WebhookEvent::BatchCompleted(e) => e.event_id(),
            WebhookEvent::BatchExpired(e) => e.event_id(),
            WebhookEvent::BatchFailed(e) => e.event_id(),
            WebhookEvent::EvalRunCanceled(e) => e.event_id(),
            WebhookEvent::EvalRunFailed(e) => e.event_id(),
            WebhookEvent::EvalRunSucceeded(e) => e.event_id(),
            WebhookEvent::FineTuningJobCancelled(e) => e.event_id(),
            WebhookEvent::FineTuningJobFailed(e) => e.event_id(),
            WebhookEvent::FineTuningJobSucceeded(e) => e.event_id(),
            WebhookEvent::RealtimeCallIncoming(e) => e.event_id(),
            WebhookEvent::ResponseCancelled(e) => e.event_id(),
            WebhookEvent::ResponseCompleted(e) => e.event_id(),
            WebhookEvent::ResponseFailed(e) => e.event_id(),
            WebhookEvent::ResponseIncomplete(e) => e.event_id(),
        }
    }
}

impl WebhookEvent {
    /// Get the timestamp when the event was created
    pub fn created_at(&self) -> u64 {
        match self {
            WebhookEvent::BatchCancelled(w) => w.created_at,
            WebhookEvent::BatchCompleted(w) => w.created_at,
            WebhookEvent::BatchExpired(w) => w.created_at,
            WebhookEvent::BatchFailed(w) => w.created_at,
            WebhookEvent::EvalRunCanceled(w) => w.created_at,
            WebhookEvent::EvalRunFailed(w) => w.created_at,
            WebhookEvent::EvalRunSucceeded(w) => w.created_at,
            WebhookEvent::FineTuningJobCancelled(w) => w.created_at,
            WebhookEvent::FineTuningJobFailed(w) => w.created_at,
            WebhookEvent::FineTuningJobSucceeded(w) => w.created_at,
            WebhookEvent::RealtimeCallIncoming(w) => w.created_at,
            WebhookEvent::ResponseCancelled(w) => w.created_at,
            WebhookEvent::ResponseCompleted(w) => w.created_at,
            WebhookEvent::ResponseFailed(w) => w.created_at,
            WebhookEvent::ResponseIncomplete(w) => w.created_at,
        }
    }
}
