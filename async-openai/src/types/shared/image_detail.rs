use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "protocol-schema", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "protocol-schema", schema(as = async_openai::shared::ImageDetail))]
pub enum ImageDetail {
    #[default]
    Auto,
    Low,
    High,
    Original,
}
