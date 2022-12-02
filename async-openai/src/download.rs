use std::path::{Path, PathBuf};

use rand::{distributions::Alphanumeric, Rng};
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

pub(crate) async fn save_b64<P: AsRef<Path>>(b64: &str, dir: P) -> Result<(), OpenAIError> {
    let filename: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let filename = format!("{filename}.png");

    let path = PathBuf::from(dir.as_ref()).join(filename);

    tokio::fs::write(
        path,
        base64::decode(b64).map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?,
    )
    .await
    .map_err(|e| OpenAIError::ImageSaveError(e.to_string()))?;

    Ok(())
}
