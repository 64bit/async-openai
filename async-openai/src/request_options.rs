use reqwest::header::HeaderMap;
use serde::Serialize;
use url::Url;

use crate::{config::OPENAI_API_BASE, error::OpenAIError};

#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    query: Option<Vec<(String, String)>>,
    headers: Option<HeaderMap>,
    path: Option<String>,
}

impl RequestOptions {
    pub(crate) fn new() -> Self {
        Self {
            query: None,
            headers: None,
            path: None,
        }
    }

    pub(crate) fn with_path(&mut self, path: &str) -> Result<(), OpenAIError> {
        if path.is_empty() {
            return Err(OpenAIError::InvalidArgument(
                "Path cannot be empty".to_string(),
            ));
        }
        self.path = Some(path.to_string());
        Ok(())
    }

    pub(crate) fn with_headers(&mut self, headers: HeaderMap) {
        // merge with existing headers or update with new headers
        if let Some(existing_headers) = &mut self.headers {
            existing_headers.extend(headers);
        } else {
            self.headers = Some(headers);
        }
    }

    pub(crate) fn with_header<K, V>(&mut self, key: K, value: V) -> Result<(), OpenAIError>
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
            headers.insert(key, value);
            self.headers = Some(headers);
        }
        Ok(())
    }

    pub(crate) fn with_query<Q: Serialize + ?Sized>(
        &mut self,
        query: &Q,
    ) -> Result<(), OpenAIError> {
        // Use serde_urlencoded::Serializer directly to handle any serializable type
        // similar to how reqwest does it. We create a temporary URL to use query_pairs_mut()
        // which allows us to handle any serializable type, not just top-level maps/structs.
        let mut url = Url::parse(OPENAI_API_BASE)
            .map_err(|e| OpenAIError::InvalidArgument(format!("Failed to create URL: {}", e)))?;

        {
            let mut pairs = url.query_pairs_mut();
            let serializer = serde_urlencoded::Serializer::new(&mut pairs);

            query
                .serialize(serializer)
                .map_err(|e| OpenAIError::InvalidArgument(format!("Invalid query: {}", e)))?;
        }

        // Extract query pairs from the URL and append to our vec
        let query = self.query.get_or_insert_with(Vec::new);
        for (key, value) in url.query_pairs() {
            query.push((key.to_string(), value.to_string()));
        }

        Ok(())
    }

    pub(crate) fn query(&self) -> &[(String, String)] {
        self.query.as_deref().unwrap_or(&[])
    }

    pub(crate) fn headers(&self) -> Option<&HeaderMap> {
        self.headers.as_ref()
    }

    pub(crate) fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}
