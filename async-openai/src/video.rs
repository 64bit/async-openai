use crate::{
    config::Config,
    error::OpenAIError,
    types::videos::{
        CreateVideoCharacterRequest, CreateVideoEditRequest, CreateVideoExtendRequest,
        CreateVideoRequest, DeletedVideoResource, RemixVideoRequest, VideoCharacterResource,
        VideoListResource, VideoResource,
    },
    Client, RequestOptions,
};
use bytes::Bytes;

/// Video generation with Sora
/// Related guide: [Video generation](https://platform.openai.com/docs/guides/video-generation)
pub struct Videos<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Videos<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Create a video
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(&self, request: CreateVideoRequest) -> Result<VideoResource, OpenAIError> {
        self.client
            .post_form("/videos", request, &self.request_options)
            .await
    }

    /// Create a video remix
    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn remix(
        &self,
        video_id: &str,
        request: RemixVideoRequest,
    ) -> Result<VideoResource, OpenAIError> {
        self.client
            .post(
                &format!("/videos/{video_id}/remix"),
                request,
                &self.request_options,
            )
            .await
    }

    /// Create a character from an uploaded video.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create_character(
        &self,
        request: CreateVideoCharacterRequest,
    ) -> Result<VideoCharacterResource, OpenAIError> {
        self.client
            .post_form("/videos/characters", request, &self.request_options)
            .await
    }

    /// Fetch a character by its ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn get_character(
        &self,
        character_id: &str,
    ) -> Result<VideoCharacterResource, OpenAIError> {
        self.client
            .get(
                &format!("/videos/characters/{character_id}"),
                &self.request_options,
            )
            .await
    }

    /// Create a new video generation job by editing a source video.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn edit(
        &self,
        request: CreateVideoEditRequest,
    ) -> Result<VideoResource, OpenAIError> {
        self.client
            .post_form("/videos/edits", request, &self.request_options)
            .await
    }

    /// Create an extension of a completed video.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn extend(
        &self,
        request: CreateVideoExtendRequest,
    ) -> Result<VideoResource, OpenAIError> {
        self.client
            .post_form("/videos/extensions", request, &self.request_options)
            .await
    }

    /// Retrieves a video by its ID.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, video_id: &str) -> Result<VideoResource, OpenAIError> {
        self.client
            .get(&format!("/videos/{}", video_id), &self.request_options)
            .await
    }

    /// Delete a Video
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, video_id: &str) -> Result<DeletedVideoResource, OpenAIError> {
        self.client
            .delete(&format!("/videos/{}", video_id), &self.request_options)
            .await
    }

    /// List Videos
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<VideoListResource, OpenAIError> {
        self.client.get("/videos", &self.request_options).await
    }

    /// Download video content.
    /// Variant can be provided as query parameter
    pub async fn download_content(&self, video_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                &format!("/videos/{video_id}/content"),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}
