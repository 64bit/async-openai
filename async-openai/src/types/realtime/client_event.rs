use serde::{Deserialize, Serialize};

use crate::types::realtime::{RealtimeConversationItem, RealtimeResponseCreateParams, Session};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeClientEventSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    /// This is an arbitrary string that a client may assign. It will be passed
    /// back if there is an error with the event, but the corresponding
    /// `session.updated` event will not include it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Update the Realtime session. Choose either a realtime session or a transcription session.
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Base64-encoded audio bytes. This must be in the format specified by
    /// the `input_audio_format` field in the session configuration.
    pub audio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventInputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealtimeClientEventConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the preceding item after which the new item will be inserted.
    /// If not set, the new item will be appended to the end of the conversation.
    /// If set to `root`, the new item will be added to the beginning of the conversation.
    /// If set to an existing ID, it allows an item to be inserted mid-conversation.
    /// If the ID cannot be found, an error will be returned and the item will not be added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,

    /// A single item within a Realtime conversation.
    pub item: RealtimeConversationItem,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the item to retrieve.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventConversationItemTruncate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the assistant message item to truncate. Only assistant message items can be truncated.
    pub item_id: String,

    /// The index of the content part to truncate. Set this to `0`.
    pub content_index: u32,

    /// Inclusive duration up to which audio is truncated, in milliseconds.
    /// If the audio_end_ms is greater than the actual audio duration, the server will respond with an error.
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventConversationItemDelete {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the item to delete.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventResponseCreate {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// Create a new Realtime response with these parameters
    pub response: Option<RealtimeResponseCreateParams>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventResponseCancel {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// A specific response ID to cancel - if not provided, will cancel an
    /// in-progress response in the default conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// These are events that the OpenAI Realtime WebSocket server will accept from the client.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeClientEvent {
    /// Send this event to update the session's configuration. The client may send this event at any time to update any field
    /// except for `voice` and `model`. `voice` can be updated only if there have been no other audio outputs yet.
    ///
    /// When the server receives a `session.update`, it will respond with a `session.updated` event showing the full, effective
    /// configuration. Only the fields that are present in the `session.update` are updated. To clear a field like `instructions`,
    /// pass an empty string. To clear a field like `tools`, pass an empty array. To clear a field like `turn_detection`, pass `null`.
    #[serde(rename = "session.update")]
    SessionUpdate(RealtimeClientEventSessionUpdate),

    /// Send this event to append audio bytes to the input audio buffer. The audio buffer is temporary storage you can write to and later commit.
    /// A "commit" will create a new user message item in the conversation history from the buffer content and clear the buffer. Input audio
    /// transcription (if enabled) will be generated when the buffer is committed.
    ///
    /// If VAD is enabled the audio buffer is used to detect speech and the server will decide when to commit. When Server VAD is disabled,
    /// you must commit the audio buffer manually. Input audio noise reduction operates on writes to the audio buffer.
    ///
    /// The client may choose how much audio to place in each event up to a maximum of 15 MiB, for example streaming smaller chunks from the
    /// client may allow the VAD to be more responsive. Unlike most other client events, the server will not send a confirmation response to
    /// this event.
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(RealtimeClientEventInputAudioBufferAppend),

    /// Send this event to commit the user input audio buffer, which will create a new user message item in the conversation.
    /// This event will produce an error if the input audio buffer is empty.
    /// When in Server VAD mode, the client does not need to send this event, the server will commit the audio buffer automatically.
    /// Committing the input audio buffer will trigger input audio transcription (if enabled in session configuration), but it will not create a response from the model.
    /// The server will respond with an input_audio_buffer.committed event.
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(RealtimeClientEventInputAudioBufferCommit),

    /// Send this event to clear the audio bytes in the buffer.
    /// The server will respond with an `input_audio_buffer.cleared` event.
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(RealtimeClientEventInputAudioBufferClear),

    /// Add a new Item to the Conversation's context, including messages, function calls, and function call responses.
    /// This event can be used both to populate a "history" of the conversation and to add new items mid-stream,
    /// but has the current limitation that it cannot populate assistant audio messages.
    ///
    /// If successful, the server will respond with a `conversation.item.created` event, otherwise an `error` event will be sent.
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(RealtimeClientEventConversationItemCreate),

    /// Send this event when you want to retrieve the server's representation of a specific item in the conversation history.
    /// This is useful, for example, to inspect user audio after noise cancellation and VAD.
    /// The server will respond with a `conversation.item.retrieved` event, unless the item does not exist in the conversation history,
    /// in which case the server will respond with an error.
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve(RealtimeClientEventConversationItemRetrieve),

    /// Send this event to truncate a previous assistant message's audio. The server will produce audio faster than realtime,
    /// so this event is useful when the user interrupts to truncate audio that has already been sent to the client but not
    /// yet played. This will synchronize the server's understanding of the audio with the client's playback.
    ///
    /// Truncating audio will delete the server-side text transcript to ensure there is not text in the context that hasn't
    /// been heard by the user.
    ///
    /// If successful, the server will respond with a `conversation.item.truncated` event.
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(RealtimeClientEventConversationItemTruncate),

    /// Send this event when you want to remove any item from the conversation history. The server will respond with a
    /// `conversation.item.deleted` event, unless the item does not exist in the conversation history, in which case the
    /// server will respond with an error.
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(RealtimeClientEventConversationItemDelete),

    /// This event instructs the server to create a Response, which means triggering model inference.
    /// When in Server VAD mode, the server will create Responses automatically.
    ///
    /// A Response will include at least one Item, and may have two, in which case the second will be a function call.
    /// These Items will be appended to the conversation history by default.
    ///
    /// The server will respond with a `response.created` event, events for Items and content created, and finally a
    /// `response.done` event to indicate the Response is complete.
    ///
    /// The `response.create` event includes inference configuration like `instructions` and `tools`. If these are set, they will
    /// override the Session's configuration for this Response only.
    ///
    /// Responses can be created out-of-band of the default Conversation, meaning that they can have arbitrary input, and
    /// it's possible to disable writing the output to the Conversation. Only one Response can write to the default
    /// Conversation at a time, but otherwise multiple Responses can be created in parallel. The `metadata` field is a good
    /// way to disambiguate multiple simultaneous Responses.
    ///
    /// Clients can set `conversation` to `none` to create a Response that does not write to the default Conversation.
    /// Arbitrary input can be provided with the `input` field, which is an array accepting raw Items and references to
    /// existing Items.
    #[serde(rename = "response.create")]
    ResponseCreate(RealtimeClientEventResponseCreate),

    /// Send this event to cancel an in-progress response. The server will respond with a `response.done` event
    /// with a status of `response.status=cancelled`. If there is no response to cancel, the server will respond
    /// with an error. It's safe to call `response.cancel` even if no response is in progress, an error will be
    /// returned the session will remain unaffected.
    #[serde(rename = "response.cancel")]
    ResponseCancel(RealtimeClientEventResponseCancel),

    /// **WebRTC Only:** Emit to cut off the current audio response.
    /// This will trigger the server to stop generating audio and emit a `output_audio_buffer.cleared` event.
    /// This event should be preceded by a `response.cancel` client event to stop the generation of the current response.
    /// [Learn more](https://platform.openai.com/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc)
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear(RealtimeClientEventOutputAudioBufferClear),
}

impl From<&RealtimeClientEvent> for String {
    fn from(value: &RealtimeClientEvent) -> Self {
        serde_json::to_string(value).unwrap()
    }
}

macro_rules! event_from {
    ($from_typ:ty, $evt_typ:ty, $variant:ident) => {
        impl From<$from_typ> for $evt_typ {
            fn from(value: $from_typ) -> Self {
                <$evt_typ>::$variant(value)
            }
        }
    };
}

event_from!(
    RealtimeClientEventSessionUpdate,
    RealtimeClientEvent,
    SessionUpdate
);
event_from!(
    RealtimeClientEventInputAudioBufferAppend,
    RealtimeClientEvent,
    InputAudioBufferAppend
);
event_from!(
    RealtimeClientEventInputAudioBufferCommit,
    RealtimeClientEvent,
    InputAudioBufferCommit
);
event_from!(
    RealtimeClientEventInputAudioBufferClear,
    RealtimeClientEvent,
    InputAudioBufferClear
);
event_from!(
    RealtimeClientEventConversationItemCreate,
    RealtimeClientEvent,
    ConversationItemCreate
);
event_from!(
    RealtimeClientEventConversationItemTruncate,
    RealtimeClientEvent,
    ConversationItemTruncate
);
event_from!(
    RealtimeClientEventConversationItemDelete,
    RealtimeClientEvent,
    ConversationItemDelete
);
event_from!(
    RealtimeClientEventConversationItemRetrieve,
    RealtimeClientEvent,
    ConversationItemRetrieve
);
event_from!(
    RealtimeClientEventResponseCreate,
    RealtimeClientEvent,
    ResponseCreate
);
event_from!(
    RealtimeClientEventResponseCancel,
    RealtimeClientEvent,
    ResponseCancel
);
event_from!(
    RealtimeClientEventOutputAudioBufferClear,
    RealtimeClientEvent,
    OutputAudioBufferClear
);

impl From<RealtimeConversationItem> for RealtimeClientEventConversationItemCreate {
    fn from(value: RealtimeConversationItem) -> Self {
        Self {
            event_id: None,
            previous_item_id: None,
            item: value,
        }
    }
}

#[cfg(feature = "_api")]
impl From<RealtimeClientEvent> for tokio_tungstenite::tungstenite::Message {
    fn from(value: RealtimeClientEvent) -> Self {
        tokio_tungstenite::tungstenite::Message::Text(String::from(&value).into())
    }
}

#[cfg(feature = "_api")]
macro_rules! message_from_event {
    ($from_typ:ty, $evt_typ:ty) => {
        impl From<$from_typ> for tokio_tungstenite::tungstenite::Message {
            fn from(value: $from_typ) -> Self {
                Self::from(<$evt_typ>::from(value))
            }
        }
    };
}

#[cfg(feature = "_api")]
message_from_event!(RealtimeClientEventSessionUpdate, RealtimeClientEvent);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventInputAudioBufferAppend,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventInputAudioBufferCommit,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventInputAudioBufferClear,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventConversationItemCreate,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventConversationItemTruncate,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventConversationItemDelete,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventConversationItemRetrieve,
    RealtimeClientEvent
);
#[cfg(feature = "_api")]
message_from_event!(RealtimeClientEventResponseCreate, RealtimeClientEvent);
#[cfg(feature = "_api")]
message_from_event!(RealtimeClientEventResponseCancel, RealtimeClientEvent);
#[cfg(feature = "_api")]
message_from_event!(
    RealtimeClientEventOutputAudioBufferClear,
    RealtimeClientEvent
);

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
impl_event_type! {
    RealtimeClientEventSessionUpdate => "session.update",
    RealtimeClientEventInputAudioBufferAppend => "input_audio_buffer.append",
    RealtimeClientEventInputAudioBufferCommit => "input_audio_buffer.commit",
    RealtimeClientEventInputAudioBufferClear => "input_audio_buffer.clear",
    RealtimeClientEventConversationItemCreate => "conversation.item.create",
    RealtimeClientEventConversationItemRetrieve => "conversation.item.retrieve",
    RealtimeClientEventConversationItemTruncate => "conversation.item.truncate",
    RealtimeClientEventConversationItemDelete => "conversation.item.delete",
    RealtimeClientEventResponseCreate => "response.create",
    RealtimeClientEventResponseCancel => "response.cancel",
    RealtimeClientEventOutputAudioBufferClear => "output_audio_buffer.clear",
}

#[cfg(feature = "_api")]
impl crate::traits::EventType for RealtimeClientEvent {
    fn event_type(&self) -> &'static str {
        match self {
            RealtimeClientEvent::SessionUpdate(e) => e.event_type(),
            RealtimeClientEvent::InputAudioBufferAppend(e) => e.event_type(),
            RealtimeClientEvent::InputAudioBufferCommit(e) => e.event_type(),
            RealtimeClientEvent::InputAudioBufferClear(e) => e.event_type(),
            RealtimeClientEvent::ConversationItemCreate(e) => e.event_type(),
            RealtimeClientEvent::ConversationItemRetrieve(e) => e.event_type(),
            RealtimeClientEvent::ConversationItemTruncate(e) => e.event_type(),
            RealtimeClientEvent::ConversationItemDelete(e) => e.event_type(),
            RealtimeClientEvent::ResponseCreate(e) => e.event_type(),
            RealtimeClientEvent::ResponseCancel(e) => e.event_type(),
            RealtimeClientEvent::OutputAudioBufferClear(e) => e.event_type(),
        }
    }
}
