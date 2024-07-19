//! Client configurations: [OpenAIConfig] for OpenAI, [AzureConfig] for Azure OpenAI Service.
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

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
pub trait Config: Clone {
    fn headers(&self) -> HeaderMap;
    fn url(&self, path: &str) -> String;
    fn query(&self) -> Vec<(&str, &str)>;

    fn api_base(&self) -> &str;

    fn api_key(&self) -> &Secret<String>;
}

/// Configuration for OpenAI API
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct OpenAIConfig {
    api_base: String,
    api_key: Secret<String>,
    #[serde(deserialize_with = "deserialize_header_map")]
    headers: HashMap<String, String>,
    org_id: String,
    project_id: String,
}

fn deserialize_header_map<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let header_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    Ok(header_map)
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_base: OPENAI_API_BASE.to_string(),
            api_key: std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            headers: HashMap::new(),
            org_id: Default::default(),
            project_id: Default::default(),
        }
    }
}

impl OpenAIConfig {
    /// Create client with default [OPENAI_API_BASE] url and default API key from OPENAI_API_KEY env var
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
        self.api_key = Secret::from(api_key.into());
        self
    }

    /// To use a API base url different from default [OPENAI_API_BASE]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    /// Add custom headers to the existing headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
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

        // hack for Assistants APIs
        // Calls to the Assistants API require that you pass a Beta header
        headers.insert(OPENAI_BETA_HEADER, "assistants=v2".parse().unwrap());

        headers.extend(self.headers.iter().map(|(k, v)| {
            (
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            )
        }));

        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &Secret<String> {
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
    api_key: Secret<String>,
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
        self.api_key = Secret::from(api_key.into());
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

        headers.insert(
            "api-key",
            self.api_key.expose_secret().as_str().parse().unwrap(),
        );

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

    fn api_key(&self) -> &Secret<String> {
        &self.api_key
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![("api-version", &self.api_version)]
    }
}
