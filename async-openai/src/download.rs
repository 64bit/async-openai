use std::path::{Path, PathBuf};

use reqwest::Url;

use crate::error::OpenAIError;

fn create_paths<P: AsRef<Path>>(url: &Url, base_dir: P) -> (PathBuf, PathBuf) {
    let mut dir = PathBuf::from(base_dir.as_ref());
    let mut path = dir.clone();
    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>());
    if let Some(segments) = segments {
        for (idx, segment) in segments.iter().enumerate() {
            if idx != segments.len() - 1 {
                dir.push(segment);
            }
            path.push(segment);
        }
    }

    (dir, path)
}

pub(crate) async fn download_url<P: AsRef<Path>>(url: &str, dir: P) -> Result<(), OpenAIError> {
    let parsed_url = Url::parse(url).map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?;
    let response = reqwest::get(url)
        .await
        .map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(OpenAIError::ImageSaveError(format!(
            "couldn't download file (status: {})",
            response.status()
        )));
    }

    let (dir, file_path) = create_paths(&parsed_url, dir);

    tokio::fs::create_dir_all(dir)
        .await
        .map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?;

    tokio::fs::write(
        file_path,
        response
            .bytes()
            .await
            .map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?,
    )
    .await
    .map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?;

    Ok(())
}
