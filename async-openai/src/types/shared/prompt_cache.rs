use serde::{Deserialize, Serialize};

/// Whether prompt-cache breakpoints are created implicitly or explicitly.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PromptCacheMode {
    Implicit,
    Explicit,
}

/// Minimum lifetime for a prompt-cache breakpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PromptCacheTtl {
    #[serde(rename = "30m")]
    Minutes30,
}

/// Options for prompt caching on a request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PromptCacheOptionsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PromptCacheTtl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PromptCacheMode>,
}

/// Prompt-caching options applied to a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PromptCacheOptions {
    pub ttl: PromptCacheTtl,
    pub mode: PromptCacheMode,
}

/// Marks the exact end of a reusable prompt prefix.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct PromptCacheBreakpoint {
    pub mode: PromptCacheBreakpointMode,
}

/// Prompt-cache breakpoint mode.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PromptCacheBreakpointMode {
    Explicit,
}

pub type PromptCacheBreakpointConfig = PromptCacheBreakpoint;
pub type PromptCacheBreakpointParam = PromptCacheBreakpoint;
pub type PromptCacheModeEnum = PromptCacheMode;
pub type PromptCacheTTLEnum = PromptCacheTtl;
