#[cfg(feature = "audio-types")]
use crate::types::audio::AudioInput;
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
use crate::types::chat::{Prompt, StopConfiguration};
#[cfg(feature = "embedding-types")]
use crate::types::embeddings::EmbeddingInput;
#[cfg(feature = "file-types")]
use crate::types::files::FileInput;
#[cfg(feature = "moderation-types")]
use crate::types::moderations::ModerationInput;
#[cfg(any(feature = "image-types", feature = "video-types"))]
use crate::types::shared::ImageInput;
#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "video-types"
))]
use crate::types::InputSource;

/// for `impl_from!(T, Enum)`, implements
/// - `From<T>`
/// - `From<Vec<T>>`
/// - `From<&Vec<T>>`
/// - `From<[T; N]>`
/// - `From<&[T; N]>`
///
/// for `T: Into<String>` and `Enum` having variants `String(String)` and `StringArray(Vec<String>)`
#[cfg(any(
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types"
))]
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
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(&str, Prompt);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(String, Prompt);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(&String, Prompt);

// From String "family" to StopConfiguration
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(&str, StopConfiguration);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(String, StopConfiguration);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from!(&String, StopConfiguration);

// From String "family" to ModerationInput
#[cfg(feature = "moderation-types")]
impl_from!(&str, ModerationInput);
#[cfg(feature = "moderation-types")]
impl_from!(String, ModerationInput);
#[cfg(feature = "moderation-types")]
impl_from!(&String, ModerationInput);

// From String "family" to EmbeddingInput
#[cfg(feature = "embedding-types")]
impl_from!(&str, EmbeddingInput);
#[cfg(feature = "embedding-types")]
impl_from!(String, EmbeddingInput);
#[cfg(feature = "embedding-types")]
impl_from!(&String, EmbeddingInput);

/// for `impl_default!(Enum)`, implements `Default` for `Enum` as `Enum::String("")` where `Enum` has `String` variant
#[cfg(any(
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types",
    feature = "moderation-types"
))]
macro_rules! impl_default {
    ($for_typ:ty) => {
        impl Default for $for_typ {
            fn default() -> Self {
                Self::String("".into())
            }
        }
    };
}

#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_default!(Prompt);
#[cfg(feature = "moderation-types")]
impl_default!(ModerationInput);
#[cfg(feature = "embedding-types")]
impl_default!(EmbeddingInput);

/// for `impl_input!(Struct)` where
/// ```text
/// Struct {
///     source: InputSource
/// }
/// ```
/// implements methods `from_bytes` and `from_vec_u8`,
/// and `From<P>` for `P: AsRef<Path>`
#[cfg(any(
    feature = "audio-types",
    feature = "file-types",
    feature = "image-types",
    feature = "video-types"
))]
macro_rules! impl_input {
    ($for_typ:ty) => {
        impl $for_typ {
            pub fn from_bytes(filename: String, bytes: bytes::Bytes) -> Self {
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

        impl<P: AsRef<std::path::Path>> From<P> for $for_typ {
            fn from(path: P) -> Self {
                let path_buf = path.as_ref().to_path_buf();
                Self {
                    source: InputSource::Path { path: path_buf },
                }
            }
        }
    };
}

#[cfg(feature = "audio-types")]
impl_input!(AudioInput);
#[cfg(feature = "file-types")]
impl_input!(FileInput);
#[cfg(any(feature = "image-types", feature = "video-types"))]
impl_input!(ImageInput);

#[cfg(any(
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types"
))]
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

#[cfg(feature = "embedding-types")]
impl_from_for_integer_array!(u32, EmbeddingInput);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from_for_integer_array!(u32, Prompt);

#[cfg(any(
    feature = "chat-completion-types",
    feature = "completion-types",
    feature = "embedding-types"
))]
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

#[cfg(feature = "embedding-types")]
impl_from_for_array_of_integer_array!(u32, EmbeddingInput);
#[cfg(any(feature = "chat-completion-types", feature = "completion-types"))]
impl_from_for_array_of_integer_array!(u32, Prompt);
