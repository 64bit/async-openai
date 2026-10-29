use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

macro_rules! json_resource {
    ($($name:ident),+ $(,)?) => {$(
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

// Live session configuration is extensible across WebRTC, SIP, tools, skills,
// and sideband transports. Preserve unknown fields while exposing distinct
// endpoint request and response types.
json_resource!(
    LiveSessionCreateRequest,
    LiveSessionCreateResponse,
    LiveCreateResponse,
    LiveCallAcceptRequest,
    LiveForkRequest,
    LiveCallReferRequest,
    LiveCallRejectRequest,
);
