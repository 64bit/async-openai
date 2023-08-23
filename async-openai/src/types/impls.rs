use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{
    download::{download_url, save_b64},
    error::OpenAIError,
    util::create_file_part,
};

use super::{
    AudioInput, AudioResponseFormat, ChatCompletionFunctionCall, CreateFileRequest,
    CreateImageEditRequest, CreateImageVariationRequest, CreateTranscriptionRequest,
    CreateTranslationRequest, EmbeddingInput, FileInput, ImageData, ImageInput, ImageResponse,
    ImageSize, ModerationInput, Prompt, ResponseFormat, Role, Stop,
};

macro_rules! impl_from {
    ($from_typ:ty, $to_typ:ty) => {
        impl From<$from_typ> for $to_typ {
            fn from(value: $from_typ) -> Self {
                <$to_typ>::String(value.into())
            }
        }

        impl From<Vec<$from_typ>> for $to_typ {
            fn from(value: Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        impl From<&Vec<$from_typ>> for $to_typ {
            fn from(value: &Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        impl<const N: usize> From<[$from_typ; N]> for $to_typ {
            fn from(value: [$from_typ; N]) -> Self {
                <$to_typ>::StringArray(value.into_iter().map(|v| v.to_string()).collect())
            }
        }

        impl<const N: usize> From<&[$from_typ; N]> for $to_typ {
            fn from(value: &[$from_typ; N]) -> Self {
                <$to_typ>::StringArray(value.into_iter().map(|v| v.to_string()).collect())
            }
        }
    };
}

// From String "family" to Prompt
impl_from!(&str, Prompt);
impl_from!(String, Prompt);
impl_from!(&String, Prompt);

// From String "family" to Stop
impl_from!(&str, Stop);
impl_from!(String, Stop);
impl_from!(&String, Stop);

// From String "family" to ModerationInput
impl_from!(&str, ModerationInput);
impl_from!(String, ModerationInput);
impl_from!(&String, ModerationInput);

// From String "family" to EmbeddingInput
impl_from!(&str, EmbeddingInput);
impl_from!(String, EmbeddingInput);
impl_from!(&String, EmbeddingInput);

macro_rules! file_path_input {
    ($for_typ:ty) => {
        impl $for_typ {
            pub fn new<P: AsRef<Path>>(path: P) -> Self {
                Self {
                    path: PathBuf::from(path.as_ref()),
                }
            }
        }

        impl<P: AsRef<Path>> From<P> for $for_typ {
            fn from(path: P) -> Self {
                Self {
                    path: PathBuf::from(path.as_ref()),
                }
            }
        }
    };
}

file_path_input!(ImageInput);
file_path_input!(FileInput);
file_path_input!(AudioInput);

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageSize::S256x256 => "256x256",
                ImageSize::S512x512 => "512x512",
                ImageSize::S1024x1024 => "1024x1024",
            }
        )
    }
}

impl Display for ResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResponseFormat::Url => "url",
                ResponseFormat::B64Json => "b64_json",
            }
        )
    }
}

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
            }
        )
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::System => "system",
                Role::Assistant => "assistant",
                Role::Function => "function",
            }
        )
    }
}

impl ImageResponse {
    /// Save each image in a dedicated Tokio task and return paths to saved files.
    /// For [ResponseFormat::Url] each file is downloaded in dedicated Tokio task.
    pub async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, OpenAIError> {
        let exists = match Path::try_exists(dir.as_ref()) {
            Ok(exists) => exists,
            Err(e) => return Err(OpenAIError::FileSaveError(e.to_string())),
        };

        if !exists {
            std::fs::create_dir_all(dir.as_ref())
                .map_err(|e| OpenAIError::FileSaveError(e.to_string()))?;
        }

        let mut handles = vec![];
        for id in self.data.clone() {
            let dir_buf = PathBuf::from(dir.as_ref());
            handles.push(tokio::spawn(async move { id.save(dir_buf).await }));
        }

        let results = futures::future::join_all(handles).await;
        let mut errors = vec![];
        let mut paths = vec![];

        for result in results {
            match result {
                Ok(inner) => match inner {
                    Ok(path) => paths.push(path),
                    Err(e) => errors.push(e),
                },
                Err(e) => errors.push(OpenAIError::FileSaveError(e.to_string())),
            }
        }

        if errors.is_empty() {
            Ok(paths)
        } else {
            Err(OpenAIError::FileSaveError(
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("; "),
            ))
        }
    }
}

impl ImageData {
    async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, OpenAIError> {
        match self {
            ImageData::Url(url) => download_url(url, dir).await,
            ImageData::B64Json(b64_json) => save_b64(b64_json, dir).await,
        }
    }
}

impl Default for ModerationInput {
    fn default() -> Self {
        ModerationInput::String("".to_owned())
    }
}

impl Default for EmbeddingInput {
    fn default() -> Self {
        EmbeddingInput::String("".to_owned())
    }
}

macro_rules! impl_from_for_integer_array {
    ($from_typ:ty, $to_typ:ty) => {
        impl<const N: usize> From<[$from_typ; N]> for $to_typ {
            fn from(value: [$from_typ; N]) -> Self {
                Self::IntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<&[$from_typ; N]> for $to_typ {
            fn from(value: &[$from_typ; N]) -> Self {
                Self::IntegerArray(value.to_vec())
            }
        }

        impl From<Vec<$from_typ>> for $to_typ {
            fn from(value: Vec<$from_typ>) -> Self {
                Self::IntegerArray(value)
            }
        }

        impl From<&Vec<$from_typ>> for $to_typ {
            fn from(value: &Vec<$from_typ>) -> Self {
                Self::IntegerArray(value.clone())
            }
        }
    };
}

impl_from_for_integer_array!(u32, EmbeddingInput);
impl_from_for_integer_array!(u16, Prompt);

macro_rules! impl_from_for_array_of_integer_array {
    ($from_typ:ty, $to_typ:ty) => {
        impl From<Vec<Vec<$from_typ>>> for $to_typ {
            fn from(value: Vec<Vec<$from_typ>>) -> Self {
                Self::ArrayOfIntegerArray(value)
            }
        }

        impl From<&Vec<Vec<$from_typ>>> for $to_typ {
            fn from(value: &Vec<Vec<$from_typ>>) -> Self {
                Self::ArrayOfIntegerArray(value.clone())
            }
        }

        impl<const M: usize, const N: usize> From<[[$from_typ; N]; M]> for $to_typ {
            fn from(value: [[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<[&[$from_typ; N]; M]> for $to_typ {
            fn from(value: [&[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<&[[$from_typ; N]; M]> for $to_typ {
            fn from(value: &[[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<&[&[$from_typ; N]; M]> for $to_typ {
            fn from(value: &[&[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<[Vec<$from_typ>; N]> for $to_typ {
            fn from(value: [Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<&[Vec<$from_typ>; N]> for $to_typ {
            fn from(value: &[Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<[&Vec<$from_typ>; N]> for $to_typ {
            fn from(value: [&Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.clone()).collect())
            }
        }

        impl<const N: usize> From<&[&Vec<$from_typ>; N]> for $to_typ {
            fn from(value: &[&Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(
                    value
                        .to_vec()
                        .into_iter()
                        .map(|inner| inner.clone())
                        .collect(),
                )
            }
        }

        impl<const N: usize> From<Vec<[$from_typ; N]>> for $to_typ {
            fn from(value: Vec<[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<&Vec<[$from_typ; N]>> for $to_typ {
            fn from(value: &Vec<[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<Vec<&[$from_typ; N]>> for $to_typ {
            fn from(value: Vec<&[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<&Vec<&[$from_typ; N]>> for $to_typ {
            fn from(value: &Vec<&[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }
    };
}

impl_from_for_array_of_integer_array!(u32, EmbeddingInput);
impl_from_for_array_of_integer_array!(u16, Prompt);

impl From<&str> for ChatCompletionFunctionCall {
    fn from(value: &str) -> Self {
        ChatCompletionFunctionCall::String(value.to_string())
    }
}

impl From<serde_json::Value> for ChatCompletionFunctionCall {
    fn from(value: serde_json::Value) -> Self {
        ChatCompletionFunctionCall::Object(value)
    }
}

// start: types to multipart from

#[async_convert::async_trait]
impl async_convert::TryFrom<CreateTranscriptionRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateTranscriptionRequest) -> Result<Self, Self::Error> {
        let audio_part = create_file_part(&request.file.path).await?;

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

#[async_convert::async_trait]
impl async_convert::TryFrom<CreateTranslationRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateTranslationRequest) -> Result<Self, Self::Error> {
        let audio_part = create_file_part(&request.file.path).await?;

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

#[async_convert::async_trait]
impl async_convert::TryFrom<CreateImageEditRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateImageEditRequest) -> Result<Self, Self::Error> {
        let image_part = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new()
            .part("image", image_part)
            .text("prompt", request.prompt);

        if let Some(mask) = request.mask {
            let mask_part = create_file_part(&mask.path).await?;
            form = form.part("mask", mask_part);
        }

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }
        Ok(form)
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<CreateImageVariationRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateImageVariationRequest) -> Result<Self, Self::Error> {
        let image_part = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image_part);

        if request.n.is_some() {
            form = form.text("n", request.n.unwrap().to_string())
        }

        if request.size.is_some() {
            form = form.text("size", request.size.unwrap().to_string())
        }

        if request.response_format.is_some() {
            form = form.text(
                "response_format",
                request.response_format.unwrap().to_string(),
            )
        }

        if request.user.is_some() {
            form = form.text("user", request.user.unwrap())
        }
        Ok(form)
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<CreateFileRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateFileRequest) -> Result<Self, Self::Error> {
        let file_part = create_file_part(&request.file.path).await?;
        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("purpose", request.purpose);
        Ok(form)
    }
}

// end: types to multipart form
