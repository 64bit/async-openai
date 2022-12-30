use super::{EmbeddingInput, Prompt, Stop};

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

// From String "family" to EmbeddingInput
impl_from!(&str, EmbeddingInput);
impl_from!(String, EmbeddingInput);
impl_from!(&String, EmbeddingInput);
