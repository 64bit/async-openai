use crate::{
    error::OpenAIError,
    traits::AsyncTryFrom,
    types::audio::{
        CreateTranscriptionRequest, CreateTranslationRequest, TranscriptionChunkingStrategy,
    },
    util::create_file_part,
};

impl AsyncTryFrom<CreateTranscriptionRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateTranscriptionRequest) -> Result<Self, Self::Error> {
        let audio_part = create_file_part(request.file.source).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", audio_part)
            .text("model", request.model);

        if let Some(language) = request.language {
            form = form.text("language", language);
        }

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string())
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string())
        }

        if let Some(include) = request.include {
            for inc in include {
                form = form.text("include[]", inc.to_string());
            }
        }

        if let Some(timestamp_granularities) = request.timestamp_granularities {
            for tg in timestamp_granularities {
                form = form.text("timestamp_granularities[]", tg.to_string());
            }
        }

        if let Some(stream) = request.stream {
            form = form.text("stream", stream.to_string());
        }

        if let Some(chunking_strategy) = request.chunking_strategy {
            match chunking_strategy {
                TranscriptionChunkingStrategy::Auto => {
                    form = form.text("chunking_strategy", "auto");
                }
                TranscriptionChunkingStrategy::ServerVad(vad_config) => {
                    form = form.text(
                        "chunking_strategy",
                        serde_json::to_string(&vad_config).unwrap().to_string(),
                    );
                }
            }
        }

        if let Some(known_speaker_names) = request.known_speaker_names {
            for kn in known_speaker_names {
                form = form.text("known_speaker_names[]", kn.to_string());
            }
        }

        if let Some(known_speaker_references) = request.known_speaker_references {
            for kn in known_speaker_references {
                form = form.text("known_speaker_references[]", kn.to_string());
            }
        }

        Ok(form)
    }
}

impl AsyncTryFrom<CreateTranslationRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateTranslationRequest) -> Result<Self, Self::Error> {
        let audio_part = create_file_part(request.file.source).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", audio_part)
            .text("model", request.model);

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string())
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string())
        }
        Ok(form)
    }
}
