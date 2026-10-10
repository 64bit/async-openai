use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

macro_rules! json_resource {
    ($($(#[$meta:meta])* $name:ident),+ $(,)?) => {$(
        $(#[$meta])*
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        #[serde(transparent)]
        pub struct $name(pub serde_json::Value);

        impl From<serde_json::Value> for $name {
            fn from(value: serde_json::Value) -> Self { Self(value) }
        }

        impl Deref for $name {
            type Target = serde_json::Value;
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    )+};
}

// The Agents API has an intentionally extensible item/tool/event model. These
// transparent resources preserve all fields while giving every endpoint a
// distinct public request or response type.
json_resource!(
    /// Parameters for creating an agent.
    CreateAgentParams,
    /// Parameters for updating an agent.
    UpdateAgentParams,
    AgentResource,
    AgentListResource,
    DeletedAgentResource,
    CreateEnvironmentTemplateParams,
    UpdateEnvironmentTemplateParams,
    EnvironmentTemplateResource,
    EnvironmentTemplateListResource,
    DeletedEnvironmentTemplateResource,
    PublicEnvironmentResource,
    HostedEnvironmentFileParam,
    EnvironmentFileResource,
    EnvironmentFileListResource,
    CreateAgentSessionParams,
    UpdateAgentSessionParams,
    CreateSessionEventsParams,
    SessionResource,
    SessionListResource,
    DeletedSessionResource,
    SessionEvent,
    SessionArtifactResource,
    SessionArtifactListResource,
    DeletedSessionArtifactResource,
    SessionItemListResource,
    SessionTraceListResource,
    SessionTurnListResource,
    TurnResource,
    SubagentResource,
    SubagentListResource,
);

#[cfg(feature = "_api")]
pub type SessionEventStream = crate::types::stream::StreamResponse<SessionEvent>;
