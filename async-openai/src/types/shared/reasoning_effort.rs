use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "protocol-schema", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "protocol-schema", schema(as = async_openai::shared::ReasoningEffort))]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    #[default]
    Medium,
    High,
    Xhigh,
    Max,
}
