use std::fmt::Display;

use crate::types::audio::{
    AudioResponseFormat, TimestampGranularity, TranscriptionInclude, TranslationResponseFormat,
};

impl Display for AudioResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AudioResponseFormat::Json => "json",
                AudioResponseFormat::Srt => "srt",
                AudioResponseFormat::Text => "text",
                AudioResponseFormat::VerboseJson => "verbose_json",
                AudioResponseFormat::Vtt => "vtt",
                AudioResponseFormat::DiarizedJson => "diarized_json",
            }
        )
    }
}

impl Display for TranslationResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TranslationResponseFormat::Json => "json",
                TranslationResponseFormat::Srt => "srt",
                TranslationResponseFormat::Text => "text",
                TranslationResponseFormat::VerboseJson => "verbose_json",
                TranslationResponseFormat::Vtt => "vtt",
            }
        )
    }
}

impl Display for TimestampGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimestampGranularity::Word => "word",
                TimestampGranularity::Segment => "segment",
            }
        )
    }
}

impl Display for TranscriptionInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TranscriptionInclude::Logprobs => "logprobs",
            }
        )
    }
}
