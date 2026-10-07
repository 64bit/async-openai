use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "openapi", schema(as = async_openai::FunctionName))]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}
