use std::path::PathBuf;

use bytes::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    Path { path: PathBuf },
    Bytes { filename: String, bytes: Bytes },
    VecU8 { filename: String, vec: Vec<u8> },
}
