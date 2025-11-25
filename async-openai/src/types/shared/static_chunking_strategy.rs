use serde::{Deserialize, Serialize};

/// Static Chunking Strategy
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Default)]
pub struct StaticChunkingStrategy {
    /// The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`.
    pub max_chunk_size_tokens: u16,
    /// The number of tokens that overlap between chunks. The default value is `400`.
    ///
    /// Note that the overlap must not exceed half of `max_chunk_size_tokens`.
    pub chunk_overlap_tokens: u16,
}
