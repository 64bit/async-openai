#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PromptCacheModeEnum {
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "explicit")]
    Explicit,
}

/// Options for prompt caching. Supported for `gpt-5.6` and later models. By default, OpenAI
/// automatically chooses one implicit cache breakpoint. You can add explicit breakpoints to content
/// blocks with `prompt_cache_breakpoint`. Each request can write up to four breakpoints. For cache
/// matching, OpenAI considers up to the latest 80 breakpoints in the conversation, without a content-
/// block lookback limit. Set `mode` to `explicit` to disable the implicit breakpoint. The `ttl`
/// defaults to `30m`, which is currently the only supported value. See the [prompt caching
/// guide](/docs/guides/prompt-caching) for current details.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PromptCacheOptionsParam {
    /// The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request.
    /// Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries
    /// for longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PromptCacheTTLEnum>,
    /// Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`.
    /// With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit
    /// breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and
    /// writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request
    /// does not use prompt caching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PromptCacheModeEnum>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PromptCacheTTLEnum {
    #[serde(rename = "30m")]
    Value30m,
}
