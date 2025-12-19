use reqwest::header::HeaderMap;

use crate::{error::OpenAIError, RequestOptions};
use serde::Serialize;

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

/// Trait for types that can build RequestOptions through fluent API
pub trait RequestOptionsBuilder: Sized {
    /// Get mutable reference to RequestOptions (for building)
    fn options_mut(&mut self) -> &mut RequestOptions;

    /// Get reference to RequestOptions
    fn options(&self) -> &RequestOptions;

    /// Add headers to RequestOptions
    fn headers(mut self, headers: HeaderMap) -> Self {
        self.options_mut().with_headers(headers);
        self
    }

    /// Add a single header to RequestOptions
    fn header<K, V>(mut self, key: K, value: V) -> Result<Self, OpenAIError>
    where
        K: reqwest::header::IntoHeaderName,
        V: TryInto<reqwest::header::HeaderValue>,
        V::Error: Into<reqwest::header::InvalidHeaderValue>,
    {
        self.options_mut().with_header(key, value)?;
        Ok(self)
    }

    /// Add query parameters to RequestOptions
    fn query<Q: Serialize + ?Sized>(mut self, query: &Q) -> Result<Self, OpenAIError> {
        self.options_mut().with_query(query)?;
        Ok(self)
    }

    /// Add a path to RequestOptions
    fn path<P: Into<String>>(mut self, path: P) -> Result<Self, OpenAIError> {
        self.options_mut().with_path(path.into().as_str())?;
        Ok(self)
    }
}
