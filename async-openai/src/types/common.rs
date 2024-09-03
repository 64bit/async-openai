use std::path::PathBuf;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    Path { path: PathBuf },
    Bytes { filename: String, bytes: Bytes },
    VecU8 { filename: String, vec: Vec<u8> },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationRole {
    Owner,
    Reader,
}
