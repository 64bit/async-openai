pub trait AsyncTryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(value: T) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send;
}

/// Trait for events to get their event type string.
pub trait EventType {
    /// Returns the event type string (e.g., "batch.cancelled")
    fn event_type(&self) -> &'static str;
}

/// Trait for events to get their event ID.
pub trait EventId {
    /// Returns the event ID
    fn event_id(&self) -> &str;
}
