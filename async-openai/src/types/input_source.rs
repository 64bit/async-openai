#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    #[cfg(not(target_family = "wasm"))]
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

#[cfg(not(target_family = "wasm"))]
impl Default for InputSource {
    fn default() -> Self {
        InputSource::Path {
            path: std::path::PathBuf::new(),
        }
    }
}

#[cfg(target_family = "wasm")]
impl Default for InputSource {
    fn default() -> Self {
        InputSource::Bytes {
            filename: String::new(),
            bytes: bytes::Bytes::new(),
        }
    }
}
