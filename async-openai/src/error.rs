use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{}: {}", .0.r#type, .0.message)]
    ApiError(ApiError),
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
    #[error("failed to save image: {0}")]
    ImageSaveError(String),
}

/*b"{\n    \"error\": {\n
\"message\": \"You exceeded your current quota, please check your plan and billing details.\",\n
     \"type\": \"insufficient_quota\",\n        \"param\": null,\n        \"code\": null\n    }\n}\n", */
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub r#type: String,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}
