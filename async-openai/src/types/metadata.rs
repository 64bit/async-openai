/// Set of 16 key-value pairs that can be attached to an object.
/// This can be useful for storing additional information about the
/// object in a structured format, and querying for objects via API
/// or the dashboard. Keys are strings with a maximum length of 64
/// characters. Values are strings with a maximum length of 512
/// characters.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
#[serde(transparent)]
pub struct Metadata(serde_json::Value);

impl From<serde_json::Value> for Metadata {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
