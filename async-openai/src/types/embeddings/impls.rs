use base64::engine::{general_purpose, Engine};

use crate::types::embeddings::Base64EmbeddingVector;

impl From<Base64EmbeddingVector> for Vec<f32> {
    fn from(value: Base64EmbeddingVector) -> Self {
        let bytes = general_purpose::STANDARD
            .decode(value.0)
            .expect("openai base64 encoding to be valid");
        let chunks = bytes.chunks_exact(4);
        chunks
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }
}
