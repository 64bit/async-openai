/// A language detected in transcribed audio.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TranscriptionLanguage {
    /// The code of a language detected in the audio.
    pub code: String,
}
