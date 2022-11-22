use reqwest::header::HeaderMap;

#[derive(Debug, Default)]
pub struct Client {
    api_key: String,
    api_base: String,
    org_id: String,
    //headers: reqwest::header::HeaderMap,
}

const API_BASE: &str = "https://api.openai.com/v1";

impl Client {
    pub fn new() -> Self {
        Self {
            api_base: API_BASE.to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or("".to_string()),
            ..Default::default()
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn with_org_id(mut self, org_id: String) -> Self {
        self.org_id = org_id;
        self
    }

    pub fn with_api_base(mut self, api_base: String) -> Self {
        self.api_base = api_base;
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
