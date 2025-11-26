//! Client configurations: [OpenAIConfig] for OpenAI, [AzureConfig] for Azure OpenAI Service.
use reqwest::header::{HeaderMap, AUTHORIZATION};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::error::OpenAIError;

/// Default v1 API base url
pub const OPENAI_API_BASE: &str = "https://api.openai.com/v1";
/// Organization header
pub const OPENAI_ORGANIZATION_HEADER: &str = "OpenAI-Organization";
/// Project header
pub const OPENAI_PROJECT_HEADER: &str = "OpenAI-Project";

/// Calls to the Assistants API require that you pass a Beta header
pub const OPENAI_BETA_HEADER: &str = "OpenAI-Beta";

/// [crate::Client] relies on this for every API call on OpenAI
/// or Azure OpenAI service
pub trait Config: Send + Sync {
    fn headers(&self) -> HeaderMap;
    fn url(&self, path: &str) -> String;
    fn query(&self) -> Vec<(&str, &str)>;

    fn api_base(&self) -> &str;

    fn api_key(&self) -> &SecretString;
}

/// Macro to implement Config trait for pointer types with dyn objects
macro_rules! impl_config_for_ptr {
    ($t:ty) => {
        impl Config for $t {
            fn headers(&self) -> HeaderMap {
                self.as_ref().headers()
            }
            fn url(&self, path: &str) -> String {
                self.as_ref().url(path)
            }
            fn query(&self) -> Vec<(&str, &str)> {
                self.as_ref().query()
            }
            fn api_base(&self) -> &str {
                self.as_ref().api_base()
            }
            fn api_key(&self) -> &SecretString {
                self.as_ref().api_key()
            }
        }
    };
}

impl_config_for_ptr!(Box<dyn Config>);
impl_config_for_ptr!(std::sync::Arc<dyn Config>);

/// Configuration for OpenAI API
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct OpenAIConfig {
    api_base: String,
    api_key: SecretString,
    org_id: String,
    project_id: String,
    #[serde(skip)]
    custom_headers: HeaderMap,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_base: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| OPENAI_API_BASE.to_string()),
            api_key: std::env::var("OPENAI_API_KEY")
                .or_else(|_| {
                    std::env::var("OPENAI_ADMIN_KEY").map(|admin_key| {
                        tracing::warn!("Using OPENAI_ADMIN_KEY, OPENAI_API_KEY not set");
                        admin_key
                    })
                })
                .unwrap_or_default()
                .into(),
            org_id: std::env::var("OPENAI_ORG_ID").unwrap_or_default(),
            project_id: std::env::var("OPENAI_PROJECT_ID").unwrap_or_default(),
            custom_headers: HeaderMap::new(),
        }
    }
}

impl OpenAIConfig {
    /// Create client with default [OPENAI_API_BASE] url (can also be changed with OPENAI_BASE_URL env var) and default API key from OPENAI_API_KEY env var
    pub fn new() -> Self {
        Default::default()
    }

    /// To use a different organization id other than default
    pub fn with_org_id<S: Into<String>>(mut self, org_id: S) -> Self {
        self.org_id = org_id.into();
        self
    }

    /// Non default project id
    pub fn with_project_id<S: Into<String>>(mut self, project_id: S) -> Self {
        self.project_id = project_id.into();
        self
    }

    /// To use a different API key different from default OPENAI_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = SecretString::from(api_key.into());
        self
    }

    /// To use a API base url different from default [OPENAI_API_BASE]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    /// Add a custom header that will be included in all requests.
    /// Headers are merged with existing headers, with custom headers taking precedence.
    pub fn with_header<K, V>(mut self, key: K, value: V) -> Result<Self, OpenAIError>
    where
        K: reqwest::header::IntoHeaderName,
        V: TryInto<reqwest::header::HeaderValue>,
        V::Error: Into<reqwest::header::InvalidHeaderValue>,
    {
        let header_value = value.try_into().map_err(|e| {
            OpenAIError::InvalidArgument(format!("Invalid header value: {}", e.into()))
        })?;
        self.custom_headers.insert(key, header_value);
        Ok(self)
    }

    pub fn org_id(&self) -> &str {
        &self.org_id
    }
}

impl Config for OpenAIConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if !self.org_id.is_empty() {
            headers.insert(
                OPENAI_ORGANIZATION_HEADER,
                self.org_id.as_str().parse().unwrap(),
            );
        }

        if !self.project_id.is_empty() {
            headers.insert(
                OPENAI_PROJECT_HEADER,
                self.project_id.as_str().parse().unwrap(),
            );
        }

        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key.expose_secret())
                .as_str()
                .parse()
                .unwrap(),
        );

        // Merge custom headers, with custom headers taking precedence
        for (key, value) in self.custom_headers.iter() {
            headers.insert(key, value.clone());
        }

        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &SecretString {
        &self.api_key
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }
}

/// Configuration for Azure OpenAI Service
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct AzureConfig {
    api_version: String,
    deployment_id: String,
    api_base: String,
    api_key: SecretString,
}

impl Default for AzureConfig {
    fn default() -> Self {
        Self {
            api_base: Default::default(),
            api_key: std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            deployment_id: Default::default(),
            api_version: Default::default(),
        }
    }
}

impl AzureConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_api_version<S: Into<String>>(mut self, api_version: S) -> Self {
        self.api_version = api_version.into();
        self
    }

    pub fn with_deployment_id<S: Into<String>>(mut self, deployment_id: S) -> Self {
        self.deployment_id = deployment_id.into();
        self
    }

    /// To use a different API key different from default OPENAI_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = SecretString::from(api_key.into());
        self
    }

    /// API base url in form of <https://your-resource-name.openai.azure.com>
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }
}

impl Config for AzureConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert("api-key", self.api_key.expose_secret().parse().unwrap());

        headers
    }

    fn url(&self, path: &str) -> String {
        format!(
            "{}/openai/deployments/{}{}",
            self.api_base, self.deployment_id, path
        )
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &SecretString {
        &self.api_key
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![("api-version", &self.api_version)]
    }
}

#[cfg(all(test, feature = "chat-completion"))]
mod test {
    use super::*;
    use crate::types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, CreateChatCompletionRequest,
    };
    use crate::Client;
    use std::sync::Arc;
    #[test]
    fn test_client_creation() {
        unsafe { std::env::set_var("OPENAI_API_KEY", "test") }
        let openai_config = OpenAIConfig::default();
        let config = Box::new(openai_config.clone()) as Box<dyn Config>;
        let client = Client::with_config(config);
        assert!(client.config().url("").ends_with("/v1"));

        let config = Arc::new(openai_config) as Arc<dyn Config>;
        let client = Client::with_config(config);
        assert!(client.config().url("").ends_with("/v1"));
        let cloned_client = client.clone();
        assert!(cloned_client.config().url("").ends_with("/v1"));
    }

    async fn dynamic_dispatch_compiles(client: &Client<Box<dyn Config>>) {
        let _ = client.chat().create(CreateChatCompletionRequest {
            model: "gpt-4o".to_string(),
            messages: vec![ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessage {
                    content: "Hello, world!".into(),
                    ..Default::default()
                },
            )],
            ..Default::default()
        });
    }

    #[tokio::test]
    async fn test_dynamic_dispatch() {
        let openai_config = OpenAIConfig::default();
        let azure_config = AzureConfig::default();

        let azure_client = Client::with_config(Box::new(azure_config.clone()) as Box<dyn Config>);
        let oai_client = Client::with_config(Box::new(openai_config.clone()) as Box<dyn Config>);

        let _ = dynamic_dispatch_compiles(&azure_client).await;
        let _ = dynamic_dispatch_compiles(&oai_client).await;

        let _ = tokio::spawn(async move { dynamic_dispatch_compiles(&azure_client).await });
        let _ = tokio::spawn(async move { dynamic_dispatch_compiles(&oai_client).await });
    }
}
