use crate::{
    error::OpenAIError,
    types::{CreateModerationRequest, CreateModerationResponse},
    Client,
};

pub struct Moderation;

impl Moderation {
    pub async fn create(
        client: &Client,
        request: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, OpenAIError> {
        client.post("/moderations", request).await
    }
}
