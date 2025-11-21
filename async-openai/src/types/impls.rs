use std::path::{Path, PathBuf};

use crate::{
    download::{download_url, save_b64},
    error::OpenAIError,
    types::{
        audio::{AudioInput, CreateSpeechResponse},
        chat::{Prompt, StopConfiguration},
        embeddings::EmbeddingInput,
        files::FileInput,
        images::{Image, ImageInput, ImagesResponse},
        moderations::ModerationInput,
        InputSource,
    },
    util::create_all_dir,
};

use bytes::Bytes;

/// for `impl_from!(T, Enum)`, implements
/// - `From<T>`
/// - `From<Vec<T>>`
/// - `From<&Vec<T>>`
/// - `From<[T; N]>`
/// - `From<&[T; N]>`
///
/// for `T: Into<String>` and `Enum` having variants `String(String)` and `StringArray(Vec<String>)`
macro_rules! impl_from {
    ($from_typ:ty, $to_typ:ty) => {
        // From<T> -> String variant
        impl From<$from_typ> for $to_typ {
            fn from(value: $from_typ) -> Self {
                <$to_typ>::String(value.into())
            }
        }

        // From<Vec<T>> -> StringArray variant
        impl From<Vec<$from_typ>> for $to_typ {
            fn from(value: Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        // From<&Vec<T>> -> StringArray variant
        impl From<&Vec<$from_typ>> for $to_typ {
            fn from(value: &Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        // From<[T; N]> -> StringArray variant
        impl<const N: usize> From<[$from_typ; N]> for $to_typ {
            fn from(value: [$from_typ; N]) -> Self {
                <$to_typ>::StringArray(value.into_iter().map(|v| v.to_string()).collect())
            }
        }

        // From<&[T; N]> -> StringArray variatn
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

// From String "family" to StopConfiguration
impl_from!(&str, StopConfiguration);
impl_from!(String, StopConfiguration);
impl_from!(&String, StopConfiguration);

// From String "family" to ModerationInput
impl_from!(&str, ModerationInput);
impl_from!(String, ModerationInput);
impl_from!(&String, ModerationInput);

// From String "family" to EmbeddingInput
impl_from!(&str, EmbeddingInput);
impl_from!(String, EmbeddingInput);
impl_from!(&String, EmbeddingInput);

/// for `impl_default!(Enum)`, implements `Default` for `Enum` as `Enum::String("")` where `Enum` has `String` variant
macro_rules! impl_default {
    ($for_typ:ty) => {
        impl Default for $for_typ {
            fn default() -> Self {
                Self::String("".into())
            }
        }
    };
}

impl_default!(Prompt);
impl_default!(ModerationInput);
impl_default!(EmbeddingInput);

impl Default for InputSource {
    fn default() -> Self {
        InputSource::Path {
            path: PathBuf::new(),
        }
    }
}

/// for `impl_input!(Struct)` where
/// ```text
/// Struct {
///     source: InputSource
/// }
/// ```
/// implements methods `from_bytes` and `from_vec_u8`,
/// and `From<P>` for `P: AsRef<Path>`
macro_rules! impl_input {
    ($for_typ:ty) => {
        impl $for_typ {
            pub fn from_bytes(filename: String, bytes: Bytes) -> Self {
                Self {
                    source: InputSource::Bytes { filename, bytes },
                }
            }

            pub fn from_vec_u8(filename: String, vec: Vec<u8>) -> Self {
                Self {
                    source: InputSource::VecU8 { filename, vec },
                }
            }
        }

        impl<P: AsRef<Path>> From<P> for $for_typ {
            fn from(path: P) -> Self {
                let path_buf = path.as_ref().to_path_buf();
                Self {
                    source: InputSource::Path { path: path_buf },
                }
            }
        }
    };
}

impl_input!(AudioInput);
impl_input!(FileInput);
impl_input!(ImageInput);

impl ImagesResponse {
    /// Save each image in a dedicated Tokio task and return paths to saved files.
    /// For [ResponseFormat::Url] each file is downloaded in dedicated Tokio task.
    pub async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, OpenAIError> {
        create_all_dir(dir.as_ref())?;

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

impl CreateSpeechResponse {
    pub async fn save<P: AsRef<Path>>(&self, file_path: P) -> Result<(), OpenAIError> {
        let dir = file_path.as_ref().parent();

        if let Some(dir) = dir {
            create_all_dir(dir)?;
        }

        tokio::fs::write(file_path, &self.bytes)
            .await
            .map_err(|e| OpenAIError::FileSaveError(e.to_string()))?;

        Ok(())
    }
}

impl Image {
    async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, OpenAIError> {
        match self {
            Image::Url { url, .. } => download_url(url, dir).await,
            Image::B64Json { b64_json, .. } => save_b64(b64_json, dir).await,
        }
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
impl_from_for_integer_array!(u32, Prompt);

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
impl_from_for_array_of_integer_array!(u32, Prompt);
