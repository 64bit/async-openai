use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{
    download::{download_url, save_b64},
    error::OpenAIError,
};

use super::{
    AudioInput, AudioResponseFormat, EmbeddingInput, FileInput, ImageData, ImageInput,
    ImageResponse, ImageSize, ModerationInput, Prompt, ResponseFormat, Role, Stop,
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
