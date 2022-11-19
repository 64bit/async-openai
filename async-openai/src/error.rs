use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{}: {}", .0.error.r#type, .0.error.message)]
    ApiError(ApiError),
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
}

/*b"{\n    \"error\": {\n
\"message\": \"You exceeded your current quota, please check your plan and billing details.\",\n
     \"type\": \"insufficient_quota\",\n        \"param\": null,\n        \"code\": null\n    }\n}\n", */
#[derive(Debug, Deserialize)]
pub struct ApiError {
    error: ErrorResponse,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    message: String,
    r#type: String,
    param: Option<serde_json::Value>,
    code: Option<serde_json::Value>,
}
