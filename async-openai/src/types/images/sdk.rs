use crate::{
    download::{download_url, save_b64},
    error::OpenAIError,
    types::images::{Image, ImagesResponse},
    util::create_all_dir,
};
use std::path::{Path, PathBuf};

impl ImagesResponse {
    /// Save each image in a dedicated Tokio task and return paths to saved files.
    /// For `ResponseFormat::Url`` each file is downloaded in dedicated Tokio task.
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

impl Image {
    async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, OpenAIError> {
        match self {
            Image::Url { url, .. } => download_url(url, dir).await,
            Image::B64Json { b64_json, .. } => save_b64(b64_json, dir).await,
        }
    }
}
