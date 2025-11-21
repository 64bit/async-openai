use crate::{error::OpenAIError, types::audio::CreateSpeechResponse, util::create_all_dir};
use std::path::Path;

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
