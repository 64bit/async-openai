use std::path::Path;

use reqwest::Body;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::error::OpenAIError;
use crate::types::InputSource;

pub(crate) async fn file_stream_body(source: InputSource) -> Result<Body, OpenAIError> {
    let body = match source {
        InputSource::Path { path } => {
            let file = File::open(path)
                .await
                .map_err(|e| OpenAIError::FileReadError(e.to_string()))?;
            let stream = FramedRead::new(file, BytesCodec::new());
            Body::wrap_stream(stream)
        }
        _ => {
            return Err(OpenAIError::FileReadError(
                "Cannot create stream from non-file source".to_string(),
            ))
        }
    };
    Ok(body)
}

/// Creates the part for the given file for multipart upload.
pub(crate) async fn create_file_part(
    source: InputSource,
) -> Result<reqwest::multipart::Part, OpenAIError> {
    let (stream, file_name) = match source {
        InputSource::Path { path } => {
            let file_name = path
                .file_name()
                .ok_or_else(|| {
                    OpenAIError::FileReadError(format!(
                        "cannot extract file name from {}",
                        path.display()
                    ))
                })?
                .to_str()
                .unwrap()
                .to_string();

            (
                file_stream_body(InputSource::Path { path }).await?,
                file_name,
            )
        }
        InputSource::Bytes { filename, bytes } => (Body::from(bytes), filename),
        InputSource::VecU8 { filename, vec } => (Body::from(vec), filename),
    };

    let file_part = reqwest::multipart::Part::stream(stream)
        .file_name(file_name)
        .mime_str("application/octet-stream")
        .unwrap();

    Ok(file_part)
}

pub(crate) fn create_all_dir<P: AsRef<Path>>(dir: P) -> Result<(), OpenAIError> {
    let exists = match Path::try_exists(dir.as_ref()) {
        Ok(exists) => exists,
        Err(e) => return Err(OpenAIError::FileSaveError(e.to_string())),
    };

    if !exists {
        std::fs::create_dir_all(dir.as_ref())
            .map_err(|e| OpenAIError::FileSaveError(e.to_string()))?;
    }

    Ok(())
}

/// Formatter for serializing JSON with non-ASCII characters escaped.
pub(crate) struct EscapeNonAscii;

impl serde_json::ser::Formatter for EscapeNonAscii {
    fn write_string_fragment<W: ?Sized + std::io::Write>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> std::io::Result<()> {
        for ch in fragment.chars() {
            if ch.is_ascii() {
                writer.write_all(ch.encode_utf8(&mut [0; 4]).as_bytes())?;
            } else {
                let mut buf = [0; 2];
                let escape = ch.encode_utf16(&mut buf);
                if escape.len() == 1 {
                    write!(writer, "\\u{:04x}", escape[0])?;
                } else {
                    write!(writer, "\\u{:04x}\\u{:04x}", escape[0], escape[1])?;
                }
            }
        }
        Ok(())
    }
}

/// Serialize the given value to JSON with non-ASCII characters escaped.
pub(crate) fn escape_non_ascii_json<T: serde::Serialize>(
    value: &T,
) -> Result<Vec<u8>, OpenAIError> {
    let mut writer = Vec::with_capacity(128);
    let formatter = EscapeNonAscii;
    let mut ser = serde_json::Serializer::with_formatter(&mut writer, formatter);
    value
        .serialize(&mut ser)
        .map_err(|err| OpenAIError::InvalidArgument(format!("{err:#}")))?;
    Ok(writer)
}
