use crate::{config::Config, Client, RequestOptions, Speech, Transcriptions, Translations};

/// Turn audio into text or text into audio.
/// Related guide: [Speech to text](https://platform.openai.com/docs/guides/speech-to-text)
pub struct Audio<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Audio<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// APIs in Speech group.
    pub fn speech(&self) -> Speech<'_, C> {
        Speech::new(self.client)
    }

    /// APIs in Transcription group.
    pub fn transcription(&self) -> Transcriptions<'_, C> {
        Transcriptions::new(self.client)
    }

    /// APIs in Translation group.
    pub fn translation(&self) -> Translations<'_, C> {
        Translations::new(self.client)
    }
}
