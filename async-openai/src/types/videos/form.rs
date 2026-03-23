use crate::{
    error::OpenAIError,
    traits::AsyncTryFrom,
    types::videos::{
        CreateVideoCharacterRequest, CreateVideoEditRequest, CreateVideoExtendRequest,
        CreateVideoRequest, VideoEditInput,
    },
    util::create_file_part,
};

impl AsyncTryFrom<CreateVideoRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("model", request.model);

        form = form.text("prompt", request.prompt);

        if let Some(size) = request.size {
            form = form.text("size", size.to_string());
        }

        if let Some(seconds) = request.seconds {
            form = form.text("seconds", seconds.to_string());
        }

        if let Some(input_reference) = request.input_reference {
            let image_part = create_file_part(input_reference.source).await?;
            form = form.part("input_reference", image_part);
        }

        if let Some(character_ids) = request.character_ids {
            for character_id in character_ids {
                form = form.text("character_ids[]", character_id);
            }
        }

        Ok(form)
    }
}

impl AsyncTryFrom<CreateVideoCharacterRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoCharacterRequest) -> Result<Self, Self::Error> {
        let video_part = create_file_part(request.video).await?;
        let form = reqwest::multipart::Form::new()
            .part("video", video_part)
            .text("name", request.name);
        Ok(form)
    }
}

impl AsyncTryFrom<CreateVideoEditRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoEditRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("prompt", request.prompt);

        match request.video {
            VideoEditInput::Reference(reference) => {
                // Send as JSON string for the video reference
                form = form.text(
                    "video",
                    serde_json::to_string(&reference).map_err(|e| {
                        OpenAIError::InvalidArgument(format!(
                            "Failed to serialize video reference: {}",
                            e
                        ))
                    })?,
                );
            }
            VideoEditInput::Input(source) => {
                let video_part = create_file_part(source).await?;
                form = form.part("video", video_part);
            }
        }

        Ok(form)
    }
}

impl AsyncTryFrom<CreateVideoExtendRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateVideoExtendRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new().text("prompt", request.prompt);

        match request.video {
            VideoEditInput::Reference(reference) => {
                form = form.text(
                    "video",
                    serde_json::to_string(&reference).map_err(|e| {
                        OpenAIError::InvalidArgument(format!(
                            "Failed to serialize video reference: {}",
                            e
                        ))
                    })?,
                );
            }
            VideoEditInput::Input(source) => {
                let video_part = create_file_part(source).await?;
                form = form.part("video", video_part);
            }
        }

        if let Some(seconds) = request.seconds {
            form = form.text("seconds", seconds.to_string());
        }

        Ok(form)
    }
}
