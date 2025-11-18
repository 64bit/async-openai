use reqwest::header::HeaderMap;
use serde::Serialize;

use crate::error::OpenAIError;

#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    query: Option<String>,
    headers: Option<HeaderMap>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self {
            query: None,
            headers: None,
        }
    }

    pub fn with_headers(&mut self, headers: HeaderMap) {
        // merge with existing headers or update with new headers
        if let Some(existing_headers) = &mut self.headers {
            existing_headers.extend(headers.into_iter());
        } else {
            self.headers = Some(headers);
        }
    }

    pub fn with_header<K, V>(&mut self, key: K, value: V) -> Result<(), OpenAIError>
    where
        K: reqwest::header::IntoHeaderName,
        V: TryInto<reqwest::header::HeaderValue>,
        V::Error: Into<reqwest::header::InvalidHeaderValue>,
    {
        let value = value.try_into().map_err(|e| {
            OpenAIError::InvalidArgument(format!("Invalid header value: {}", e.into()))
        })?;
        if let Some(headers) = &mut self.headers {
            headers.insert(key, value);
        } else {
            let mut headers = HeaderMap::new();
            headers.insert(key, value.into());
            self.headers = Some(headers);
        }
        Ok(())
    }

    pub fn with_query<Q: Serialize + ?Sized>(&mut self, query: &Q) -> Result<(), OpenAIError> {
        let new_query = serde_urlencoded::to_string(query)
            .map_err(|e| OpenAIError::InvalidArgument(format!("Invalid query: {}", e)))?;
        if let Some(existing_query) = &self.query {
            self.query = Some(format!("{}&{}", existing_query, new_query));
        } else {
            self.query = Some(new_query);
        }
        Ok(())
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    pub fn headers(&self) -> Option<&HeaderMap> {
        self.headers.as_ref()
    }
}
