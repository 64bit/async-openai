use std::pin::Pin;

use futures::Stream;
use serde::{Deserialize};

use crate::error::{ApiError, OpenAIError};

use super::{
    MessageDeltaObject, MessageObject, RunObject, RunStepDeltaObject, RunStepObject, ThreadObject,
};

/// Represents an event emitted when streaming a Run.
///
/// Each event in a server-sent events stream has an `event` and `data` property:
///
/// ```
/// event: thread.created
/// data: {"id": "thread_123", "object": "thread", ...}
/// ```
///
/// We emit events whenever a new object is created, transitions to a new state, or is being
/// streamed in parts (deltas). For example, we emit `thread.run.created` when a new run
/// is created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses
/// to create a message during a run, we emit a `thread.message.created event`, a
/// `thread.message.in_progress` event, many `thread.message.delta` events, and finally a
/// `thread.message.completed` event.
///
/// We may add additional events over time, so we recommend handling unknown events gracefully
/// in your code. See the [Assistants API quickstart](https://platform.openai.com/docs/assistants/overview) to learn how to
/// integrate the Assistants API with streaming.

#[derive(Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum AssistantStreamEvent {
    /// Occurs when a new [thread](https://platform.openai.com/docs/api-reference/threads/object) is created.
    #[serde(rename = "thread.created")]
    TreadCreated(ThreadObject),
    /// Occurs when a new [run](https://platform.openai.com/docs/api-reference/runs/object) is created.
    #[serde(rename = "thread.run.created")]
    ThreadRunCreated(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) moves to a `queued` status.
    #[serde(rename = "thread.run.queued")]
    ThreadRunQueued(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) moves to an `in_progress` status.
    #[serde(rename = "thread.run.in_progress")]
    ThreadRunInProgress(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) moves to a `requires_action` status.
    #[serde(rename = "thread.run.requires_action")]
    ThreadRunRequiresAction(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) is completed.
    #[serde(rename = "thread.run.completed")]
    ThreadRunCompleted(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) ends with status `incomplete`.
    #[serde(rename = "thread.run.incomplete")]
    ThreadRunIncomplete(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) fails.
    #[serde(rename = "thread.run.failed")]
    ThreadRunFailed(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) moves to a `cancelling` status.
    #[serde(rename = "thread.run.cancelling")]
    ThreadRunCancelling(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) is cancelled.
    #[serde(rename = "thread.run.cancelled")]
    ThreadRunCancelled(RunObject),
    /// Occurs when a [run](https://platform.openai.com/docs/api-reference/runs/object) expires.
    #[serde(rename = "thread.run.expired")]
    ThreadRunExpired(RunObject),
    /// Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) is created.
    #[serde(rename = "thread.run.step.created")]
    ThreadRunStepCreated(RunStepObject),
    /// Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) moves to an `in_progress` state.
    #[serde(rename = "thread.run.step.in_progress")]
    ThreadRunStepInProgress(RunStepObject),
    /// Occurs when parts of a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) are being streamed.
    #[serde(rename = "thread.run.step.delta")]
    ThreadRunStepDelta(RunStepDeltaObject),
    ///  Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) is completed.
    #[serde(rename = "thread.run.step.completed")]
    ThreadRunStepCompleted(RunStepObject),
    /// Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) fails.
    #[serde(rename = "thread.run.step.failed")]
    ThreadRunStepFailed(RunStepObject),
    /// Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) is cancelled.
    #[serde(rename = "thread.run.step.cancelled")]
    ThreadRunStepCancelled(RunStepObject),
    /// Occurs when a [run step](https://platform.openai.com/docs/api-reference/runs/step-object) expires.
    #[serde(rename = "thread.run.step.expired")]
    ThreadRunStepExpired(RunStepObject),
    /// Occurs when a [message](https://platform.openai.com/docs/api-reference/messages/object) is created.
    #[serde(rename = "thread.message.created")]
    ThreadMessageCreated(MessageObject),
    /// Occurs when a [message](https://platform.openai.com/docs/api-reference/messages/object) moves to an `in_progress` state.
    #[serde(rename = "thread.message.in_progress")]
    ThreadMessageInProgress(MessageObject),
    /// Occurs when parts of a [Message](https://platform.openai.com/docs/api-reference/messages/object) are being streamed.
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta(MessageDeltaObject),
    /// Occurs when a [message](https://platform.openai.com/docs/api-reference/messages/object) is completed.
    #[serde(rename = "thread.message.completed")]
    ThreadMessageCompleted(MessageObject),
    /// Occurs when a [message](https://platform.openai.com/docs/api-reference/messages/object) ends before it is completed.
    #[serde(rename = "thread.message.incomplete")]
    ThreadMessageIncomplete(MessageObject),
    /// Occurs when an [error](https://platform.openai.com/docs/guides/error-codes/api-errors) occurs. This can happen due to an internal server error or a timeout.
    #[serde(rename = "error")]
    ErrorEvent(ApiError),
    /// Occurs when a stream ends.
    #[serde(rename = "done")]
    Done(String),
}

pub type AssistantEventStream =
    Pin<Box<dyn Stream<Item = Result<AssistantStreamEvent, OpenAIError>> + Send>>;
