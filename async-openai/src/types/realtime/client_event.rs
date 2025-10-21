use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use super::{
    item::Item,
    session_resource::{
        AudioFormat, MaxResponseOutputTokens, Modality, RealtimeVoice, SessionResource, ToolChoice,
        ToolDefinition,
    },
};

/// Configuration for a response in the OpenAI Realtime API.
/// This is used in the `response.create` event.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseConfig {
    /// Controls which conversation the response is added to. Currently supports "auto" and "none",
    /// with "auto" as the default value. The "auto" value means that the contents of the response
    /// will be added to the default conversation. Set this to "none" to create an out-of-band response
    /// which will not add items to default conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,

    /// Input items to include in the prompt for the model. Using this field creates a new context
    /// for this Response instead of using the default conversation. An empty array [] will clear
    /// the context for this Response. Note that this can include references to items from the default conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<Item>>,

    /// The default system instructions (i.e. system message) prepended to model calls.
    /// This field allows the client to guide the model on desired responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response, inclusive of tool calls.
    /// Provide an integer between 1 and 4096 to limit output tokens, or "inf" for the maximum available tokens for a given model.
    /// Defaults to "inf".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<MaxResponseOutputTokens>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// The set of modalities the model can respond with. To disable audio, set this to ["text"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,

    /// The format of output audio. Options are "pcm16", "g711_ulaw", or "g711_alaw".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<AudioFormat>,

    /// Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// How the model chooses tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Tools (functions) available to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// The voice the model uses to respond. Cannot be changed once the model has responded with audio at least once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<RealtimeVoice>,

    /// The speed of the model's spoken response. 1.0 is the default speed. 0.25 is the minimum speed. 1.5 is the maximum speed.
    /// This value can only be changed in between model turns, not while a response is in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionUpdateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Session configuration to update.
    pub session: SessionResource,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferAppendEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Base64-encoded audio bytes.
    pub audio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferCommitEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputAudioBufferClearEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OutputAudioBufferClearEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationItemCreateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the preceding item after which the new item will be inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,

    /// The item to add to the conversation.
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemTruncateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the assistant message item to truncate.
    pub item_id: String,

    /// The index of the content part to truncate.
    pub content_index: u32,

    /// Inclusive duration up to which audio is truncated, in milliseconds.
    pub audio_end_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemDeleteEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the item to delete.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversationItemRetrieveEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the item to retrieve.
    pub item_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCreateEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// Configuration for the response.
    pub response: Option<ResponseConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseCancelEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// A specific response ID to cancel - if not provided, will cancel an in-progress response in the default conversation.
    pub response_id: String,
}

/// These are events that the OpenAI Realtime WebSocket server will accept from the client.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientEvent {
    /// Send this event to update the session’s default configuration.
    #[serde(rename = "session.update")]
    SessionUpdate(SessionUpdateEvent),

    /// Send this event to append audio bytes to the input audio buffer.
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(InputAudioBufferAppendEvent),

    /// Send this event to commit audio bytes to a user message.
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(InputAudioBufferCommitEvent),

    /// Send this event to clear the audio bytes in the buffer.
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(InputAudioBufferClearEvent),

    /// WebRTC Only: Send this event to cut off the current audio response.
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear(OutputAudioBufferClearEvent),

    /// Send this event when adding an item to the conversation.
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(ConversationItemCreateEvent),

    /// Send this event when you want to truncate a previous assistant message’s audio.
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(ConversationItemTruncateEvent),

    /// Send this event when you want to remove any item from the conversation history.
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(ConversationItemDeleteEvent),

    /// Send this event when you want to retrieve the server's representation of a specific item in the conversation history.
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve(ConversationItemRetrieveEvent),

    /// Send this event to trigger a response generation.
    #[serde(rename = "response.create")]
    ResponseCreate(ResponseCreateEvent),

    /// Send this event to cancel an in-progress response.
    #[serde(rename = "response.cancel")]
    ResponseCancel(ResponseCancelEvent),
}

impl From<&ClientEvent> for String {
    fn from(value: &ClientEvent) -> Self {
        serde_json::to_string(value).unwrap()
    }
}

impl From<ClientEvent> for Message {
    fn from(value: ClientEvent) -> Self {
        Message::Text(String::from(&value).into())
    }
}

macro_rules! message_from_event {
    ($from_typ:ty, $evt_typ:ty) => {
        impl From<$from_typ> for Message {
            fn from(value: $from_typ) -> Self {
                Self::from(<$evt_typ>::from(value))
            }
        }
    };
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

event_from!(SessionUpdateEvent, ClientEvent, SessionUpdate);
event_from!(
    InputAudioBufferAppendEvent,
    ClientEvent,
    InputAudioBufferAppend
);
event_from!(
    InputAudioBufferCommitEvent,
    ClientEvent,
    InputAudioBufferCommit
);
event_from!(
    InputAudioBufferClearEvent,
    ClientEvent,
    InputAudioBufferClear
);
event_from!(
    OutputAudioBufferClearEvent,
    ClientEvent,
    OutputAudioBufferClear
);
event_from!(
    ConversationItemCreateEvent,
    ClientEvent,
    ConversationItemCreate
);
event_from!(
    ConversationItemTruncateEvent,
    ClientEvent,
    ConversationItemTruncate
);
event_from!(
    ConversationItemDeleteEvent,
    ClientEvent,
    ConversationItemDelete
);
event_from!(ResponseCreateEvent, ClientEvent, ResponseCreate);
event_from!(ResponseCancelEvent, ClientEvent, ResponseCancel);
event_from!(
    ConversationItemRetrieveEvent,
    ClientEvent,
    ConversationItemRetrieve
);

message_from_event!(SessionUpdateEvent, ClientEvent);
message_from_event!(InputAudioBufferAppendEvent, ClientEvent);
message_from_event!(InputAudioBufferCommitEvent, ClientEvent);
message_from_event!(InputAudioBufferClearEvent, ClientEvent);
message_from_event!(OutputAudioBufferClearEvent, ClientEvent);
message_from_event!(ConversationItemCreateEvent, ClientEvent);
message_from_event!(ConversationItemTruncateEvent, ClientEvent);
message_from_event!(ConversationItemDeleteEvent, ClientEvent);
message_from_event!(ConversationItemRetrieveEvent, ClientEvent);
message_from_event!(ResponseCreateEvent, ClientEvent);
message_from_event!(ResponseCancelEvent, ClientEvent);

impl From<Item> for ConversationItemCreateEvent {
    fn from(value: Item) -> Self {
        Self {
            event_id: None,
            previous_item_id: None,
            item: value,
        }
    }
}
