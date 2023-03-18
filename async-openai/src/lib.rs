//! Async Rust library for OpenAI REST API based on OpenAPI spec.
//!
//! ## Creating client
//!
//! ```
//! use async_openai::Client;
//!
//! // Create a client with api key from env var OPENAI_API_KEY and default base url.
//! let client = Client::new();
//!
//! // OR use API key from different source
//! let api_key = "sk-..."; // This secret could be from a file, or environment variable.
//! let client = Client::new().with_api_key(api_key);
//!
//! // Use organization other than default when making requests
//! let client = Client::new().with_org_id("the-org");
//!
//! // Use custom reqwest client
//! let http_client = reqwest::ClientBuilder::new().user_agent("async-openai").build().unwrap();
//! let client = Client::new().with_http_client(http_client);
//! ```
//!
//! ## Making requests
//!
//!```
//!# tokio_test::block_on(async {
//!
//! use async_openai::{Client, types::{CreateCompletionRequestArgs}};
//!
//! // Create client
//! let client = Client::new();
//!
//! // Create request using builder pattern
//! // Every request struct has companion builder struct with same name + Args suffix
//! let request = CreateCompletionRequestArgs::default()
//!     .model("text-davinci-003")
//!     .prompt("Tell me the recipe of alfredo pasta")
//!     .max_tokens(40_u16)
//!     .build()
//!     .unwrap();
//!
//! // Call API
//! let response = client
//!     .completions()      // Get the API "group" (completions, images, etc.) from the client
//!     .create(request)    // Make the API call in that "group"
//!     .await
//!     .unwrap();
//!
//! println!("{}", response.choices.first().unwrap().text);
//! # });
//!```
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/64bit/async-openai/tree/main/examples) directory in the repository.
//!
mod audio;
mod chat;
mod client;
mod completion;
mod download;
mod edit;
mod embedding;
pub mod error;
mod file;
mod fine_tune;
mod image;
mod model;
mod moderation;
pub mod types;
mod util;

pub use audio::Audio;
pub use chat::Chat;
pub use client::Client;
pub use client::API_BASE;
pub use client::ORGANIZATION_HEADER;
pub use completion::Completions;
pub use edit::Edits;
pub use embedding::Embeddings;
pub use file::Files;
pub use fine_tune::FineTunes;
pub use image::Images;
pub use model::Models;
pub use moderation::Moderations;
