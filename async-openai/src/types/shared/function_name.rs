use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "protocol-schema", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "protocol-schema", schema(as = async_openai::shared::FunctionName))]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}
