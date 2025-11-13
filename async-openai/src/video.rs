use crate::{
    config::Config,
    error::OpenAIError,
    types::videos::{
        CreateVideoRequest, ListVideosResponse, RemixVideoRequest, VideoJob, VideoJobMetadata,
        VideoVariant,
    },
    Client,
};
use bytes::Bytes;
use serde::Serialize;

/// Video generation with Sora
/// Related guide: [Video generation](https://platform.openai.com/docs/guides/video-generation)
pub struct Videos<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Videos<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Create a video
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(&self, request: CreateVideoRequest) -> Result<VideoJob, OpenAIError> {
        self.client.post_form("/videos", request).await
    }

    /// Create a video remix
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn remix(
        &self,
        video_id: &str,
        request: RemixVideoRequest,
    ) -> Result<VideoJob, OpenAIError> {
        self.client
            .post(&format!("/videos/{video_id}/remix"), request)
            .await
    }

    /// Retrieves a video by its ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, video_id: &str) -> Result<VideoJob, OpenAIError> {
        self.client.get(&format!("/videos/{}", video_id)).await
    }

    /// Delete a Video
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, video_id: &str) -> Result<VideoJobMetadata, OpenAIError> {
        self.client.delete(&format!("/videos/{}", video_id)).await
    }

    /// List Videos
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ListVideosResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/videos", &query).await
    }

    /// Download video content
    pub async fn download_content(
        &self,
        video_id: &str,
        variant: VideoVariant,
    ) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw_with_query(
                &format!("/videos/{video_id}/content"),
                &[("variant", variant)],
            )
            .await?;
        Ok(bytes)
    }
}
