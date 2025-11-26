#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    Path {
        path: std::path::PathBuf,
    },
    Bytes {
        filename: String,
        bytes: bytes::Bytes,
    },
    VecU8 {
        filename: String,
        vec: Vec<u8>,
    },
}

impl Default for InputSource {
    fn default() -> Self {
        InputSource::Path {
            path: std::path::PathBuf::new(),
        }
    }
}
