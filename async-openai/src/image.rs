use crate::{
    config::Config,
    error::OpenAIError,
    types::images::{
        CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImageEditStream,
        ImageGenStream, ImagesResponse,
    },
    Client, RequestOptions,
};

/// Given a prompt and/or an input image, the model will generate a new image.
///
/// Related guide: [Image generation](https://platform.openai.com/docs/guides/images)
pub struct Images<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Images<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Creates an image given a prompt.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn generate(
        &self,
        request: CreateImageRequest,
    ) -> Result<ImagesResponse, OpenAIError> {
        self.client
            .post("/images/generations", request, &self.request_options)
            .await
    }

    /// Creates an image given a prompt.
    #[crate::byot(
        T0 = serde::Serialize,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static"
    )]
    #[allow(unused_mut)]
    pub async fn generate_stream(
        &self,
        mut request: CreateImageRequest,
    ) -> Result<ImageGenStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if request.stream.is_some() && !request.stream.unwrap() {
                return Err(OpenAIError::InvalidArgument(
                    "When stream is false, use Image::generate".into(),
                ));
            }

            request.stream = Some(true);
        }

        Ok(self
            .client
            .post_stream("/images/generations", request, &self.request_options)
            .await)
    }

    /// Creates an edited or extended image given one or more source images and a prompt.
    /// This endpoint only supports gpt-image-1 and dall-e-2.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn edit(
        &self,
        request: CreateImageEditRequest,
    ) -> Result<ImagesResponse, OpenAIError> {
        self.client
            .post_form("/images/edits", request, &self.request_options)
            .await
    }

    /// Creates an edited or extended image given one or more source images and a prompt.
    /// This endpoint only supports gpt-image-1 and dall-e-2.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        stream = "true",
        where_clause = "R: std::marker::Send + 'static, reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>"
    )]
    #[allow(unused_mut)]
    pub async fn edit_stream(
        &self,
        mut request: CreateImageEditRequest,
    ) -> Result<ImageEditStream, OpenAIError> {
        #[cfg(not(feature = "byot"))]
        {
            if let Some(stream) = request.stream {
                if !stream {
                    return Err(OpenAIError::InvalidArgument(
                        "When stream is false, use Image::edit".into(),
                    ));
                }
            }
            request.stream = Some(true);
        }
        self.client
            .post_form_stream("/images/edits", request, &self.request_options)
            .await
    }

    /// Creates a variation of a given image. This endpoint only supports dall-e-2.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create_variation(
        &self,
        request: CreateImageVariationRequest,
    ) -> Result<ImagesResponse, OpenAIError> {
        self.client
            .post_form("/images/variations", request, &self.request_options)
            .await
    }
}
