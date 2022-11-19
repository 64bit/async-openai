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

// use http_types::{Body, Method, Request, Url};
// use serde::{de::DeserializeOwned, Serialize};

// use crate::{
//     client::{request_strategy::RequestStrategy, BaseClient, Response},
//     config::err,
//     params::AppInfo,
//     Headers,
// };

// static USER_AGENT: &str = concat!("OpenAI/v1 Rust/", env!("CARGO_PKG_VERSION"));

// #[derive(Clone)]
// pub struct Client {
//     client: hyper::Client,
//     api_key: String,
//     headers: Headers,
//     api_base: Url,
//     api_root: String,
// }

// impl Client {
//     /// Create a new account with the given secret key.
//     pub fn new(api_key: impl Into<String>) -> Self {
//         Self::from_url("https://api.openai.com/", api_key)
//     }

//     /// Create a new account pointed at a specific URL. This is useful for testing.
//     pub fn from_url<'a>(url: impl Into<&'a str>, api_key: impl Into<String>) -> Self {
//         Client {
//             client: BaseClient::new(),
//             secret_key: secret_key.into(),
//             headers: Headers {
//                 stripe_version: ApiVersion::V2020_08_27,
//                 user_agent: USER_AGENT.to_string(),
//                 client_id: None,
//                 stripe_account: None,
//             },
//             strategy: RequestStrategy::Once,
//             app_info: None,
//             api_base: Url::parse(url.into()).expect("invalid url"),
//             api_root: "v1".to_string(),
//         }
//     }

//     /// Set the client id for the client.
//     pub fn with_org_id(mut self, id: ApplicationId) -> Self {
//         self.headers.client_id = Some(id);
//         self
//     }

//     /// Make a `GET` http request with just a path
//     pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
//         let url = self.url(path);
//         self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
//     }

//     /// Make a `GET` http request with url query parameters
//     pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
//         &self,
//         path: &str,
//         params: P,
//     ) -> Response<T> {
//         let url = match self.url_with_params(path, params) {
//             Err(e) => return err(e),
//             Ok(ok) => ok,
//         };
//         self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
//     }

//     /// Make a `DELETE` http request with just a path
//     pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
//         let url = self.url(path);
//         self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
//     }

//     /// Make a `DELETE` http request with url query parameters
//     pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
//         &self,
//         path: &str,
//         params: P,
//     ) -> Response<T> {
//         let url = match self.url_with_params(path, params) {
//             Err(e) => return err(e),
//             Ok(ok) => ok,
//         };
//         self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
//     }

//     /// Make a `POST` http request with just a path
//     pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
//         let url = self.url(path);
//         self.client.execute::<T>(self.create_request(Method::Post, url), &self.strategy)
//     }

//     /// Make a `POST` http request with urlencoded body
//     pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
//         &self,
//         path: &str,
//         form: F,
//     ) -> Response<T> {
//         let url = self.url(path);
//         let mut req = self.create_request(Method::Post, url);

//         let mut params_buffer = Vec::new();
//         let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
//         if let Err(qs_ser_err) = serde_path_to_error::serialize(&form, qs_ser) {
//             return err(StripeError::QueryStringSerialize(qs_ser_err));
//         }

//         let body = std::str::from_utf8(params_buffer.as_slice())
//             .expect("Unable to extract string from params_buffer")
//             .to_string();

//         req.set_body(Body::from_string(body));

//         req.insert_header("content-type", "application/x-www-form-urlencoded");
//         self.client.execute::<T>(req, &self.strategy)
//     }

//     fn url(&self, path: &str) -> Url {
//         let mut url = self.api_base.clone();
//         url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
//         url
//     }

//     fn url_with_params<P: Serialize>(&self, path: &str, params: P) -> Result<Url, StripeError> {
//         let mut url = self.url(path);

//         let mut params_buffer = Vec::new();
//         let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
//         serde_path_to_error::serialize(&params, qs_ser).map_err(StripeError::from)?;

//         let params = std::str::from_utf8(params_buffer.as_slice())
//             .expect("Unable to extract string from params_buffer")
//             .to_string();

//         url.set_query(Some(&params));
//         Ok(url)
//     }

//     fn create_request(&self, method: Method, url: Url) -> Request {
//         let mut req = Request::new(method, url);
//         req.insert_header("authorization", &format!("Bearer {}", self.secret_key));

//         for (key, value) in self.headers.to_array().iter().filter_map(|(k, v)| v.map(|v| (*k, v))) {
//             req.insert_header(key, value);
//         }

//         req
//     }
// }
