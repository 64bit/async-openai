//! Async Rust library for OpenAI REST API based on OpenAPI spec.
//!
//! ## Creating client
//!
//! ```
//! use async_openai as openai;
//!
//! // Create a client with api key from env var OPENAI_API_KEY and default base url.
//! let client = openai::Client::new();
//!
//! // OR use API key from different source
//! let api_key = "sk-..."; // This could be from a file, hard coding secret is not a best practice.
//! let client = openai::Client::new().with_api_key(api_key);
//! ```
//!
//! ## Making requests
//!
//!```
//!# tokio_test::block_on(async {
//! use async_openai as openai;
//! use openai::{Client, Completion, types::{CreateCompletionRequest}};
//!
//! // Create client
//! let client = Client::new();
//! // Create request
//! let request = CreateCompletionRequest {
//!     model: "text-davinci-003".to_owned(),
//!     prompt: Some("Tell me a joke about the universe".to_owned()),
//!     ..Default::default()
//! };
//! // Call API
//! let response = Completion::create(&client, request).await.unwrap();
//!
//! println!("{}", response.choices.first().unwrap().text);
//! # });
//!```
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/64bit/async-openai/tree/main/examples) directory in the repository.
//!
mod client;
mod completion;
mod download;
mod edit;
pub mod error;
mod file;
mod fine_tune;
mod image;
mod model;
mod moderation;
pub mod types;
mod util;

pub use client::Client;
pub use client::API_BASE;
pub use completion::Completion;
pub use edit::Edit;
pub use file::File;
pub use fine_tune::FineTunes;
pub use image::Image;
pub use model::Models;
pub use moderation::Moderation;
