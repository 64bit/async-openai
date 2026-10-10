use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "openapi", schema(as = async_openai::ImageDetail))]
pub enum ImageDetail {
    #[default]
    Auto,
    Low,
    High,
    Original,
}
