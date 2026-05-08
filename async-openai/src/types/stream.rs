#[cfg(not(target_family = "wasm"))]
pub type StreamResponse<T> =
    std::pin::Pin<Box<dyn futures::Stream<Item = Result<T, crate::error::OpenAIError>> + Send>>;

#[cfg(target_family = "wasm")]
pub type StreamResponse<T> =
    std::pin::Pin<Box<dyn futures::Stream<Item = Result<T, crate::error::OpenAIError>>>>;
