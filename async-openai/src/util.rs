use std::path::Path;

use reqwest::Body;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::error::OpenAIError;

pub(crate) async fn file_stream_body<P: AsRef<Path>>(path: P) -> Result<Body, OpenAIError> {
    let file = tokio::fs::File::open(path.as_ref())
        .await
        .map_err(|e| OpenAIError::FileReadError(e.to_string()))?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    Ok(body)
}

/// Creates the part for the given image file for multipart upload.
pub(crate) async fn create_file_part<P: AsRef<Path>>(
    path: P,
) -> Result<reqwest::multipart::Part, OpenAIError> {
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| {
            OpenAIError::FileReadError(format!(
                "cannot extract file name from {}",
                path.as_ref().display()
            ))
        })?
        .to_str()
        .unwrap()
        .to_string();

    let file_part = reqwest::multipart::Part::stream(file_stream_body(path.as_ref()).await?)
        .file_name(file_name)
        .mime_str("application/octet-stream")
        .unwrap();

    Ok(file_part)
}
