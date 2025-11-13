//! Rust library for OpenAI
//!
//! ## Creating client
//!
//! ```
//! use async_openai::{Client, config::OpenAIConfig};
//!
//! // Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
//! let client = Client::new();
//!
//! // Above is shortcut for
//! let config = OpenAIConfig::default();
//! let client = Client::with_config(config);
//!
//! // OR use API key from different source and a non default organization
//! let api_key = "sk-..."; // This secret could be from a file, or environment variable.
//! let config = OpenAIConfig::new()
//!     .with_api_key(api_key)
//!     .with_org_id("the-continental");
//!
//! let client = Client::with_config(config);
//!
//! // Use custom reqwest client
//! let http_client = reqwest::ClientBuilder::new().user_agent("async-openai").build().unwrap();
//! let client = Client::new().with_http_client(http_client);
//! ```
//!
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
//!     .model("gpt-3.5-turbo-instruct")
//!     .prompt("Tell me the recipe of alfredo pasta")
//!     .max_tokens(40_u32)
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
//! ## Bring Your Own Types
//!
//! To use custom types for inputs and outputs, enable `byot` feature which provides additional generic methods with same name and `_byot` suffix.
//! This feature is available on methods whose return type is not `Bytes`
//!
//!```
//!# #[cfg(feature = "byot")]
//!# tokio_test::block_on(async {
//! use async_openai::Client;
//! use serde_json::{Value, json};
//!
//! let client = Client::new();
//!
//! let response: Value = client
//!        .chat()
//!        .create_byot(json!({
//!            "messages": [
//!                {
//!                    "role": "developer",
//!                    "content": "You are a helpful assistant"
//!                },
//!                {
//!                    "role": "user",
//!                    "content": "What do you think about life?"
//!                }
//!            ],
//!            "model": "gpt-4o",
//!            "store": false
//!        }))
//!        .await
//!        .unwrap();
//!
//!  if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
//!     println!("{}", content);
//!  }
//! # });
//!```
//!
//! ## Dynamic Dispatch for OpenAI-compatible Providers
//!
//! For any struct that implements `Config` trait, wrap it in a smart pointer and cast the pointer to `dyn Config`
//! trait object, then create a client with `Box` or `Arc` wrapped configuration.
//!
//! For example:
//! ```
//! use async_openai::{Client, config::{Config, OpenAIConfig}};
//!
//! // Use `Box` or `std::sync::Arc` to wrap the config
//! let config = Box::new(OpenAIConfig::default()) as Box<dyn Config>;
//! // A function can now accept a `&Client<Box<dyn Config>>` parameter
//! // which can invoke any openai compatible api
//! let client: Client<Box<dyn Config>> = Client::with_config(config);
//! ```
//!
//! ## Microsoft Azure
//!
//! ```
//! use async_openai::{Client, config::AzureConfig};
//!
//! let config = AzureConfig::new()
//!     .with_api_base("https://my-resource-name.openai.azure.com")
//!     .with_api_version("2023-03-15-preview")
//!     .with_deployment_id("deployment-id")
//!     .with_api_key("...");
//!
//! let client = Client::with_config(config);
//!
//! // Note that `async-openai` only implements OpenAI spec
//! // and doesn't maintain parity with the spec of Azure OpenAI service.
//!
//! ```
//!
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/64bit/async-openai/tree/main/examples) directory in the repository.
//!
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "byot")]
pub(crate) use async_openai_macros::byot;

#[cfg(not(feature = "byot"))]
pub(crate) use async_openai_macros::byot_passthrough as byot;

mod admin;
mod admin_api_keys;
mod assistants;
mod audio;
mod audit_logs;
mod batches;
mod certificates;
mod chat;
mod chatkit;
mod client;
mod completion;
pub mod config;
mod container_files;
mod containers;
mod conversation_items;
mod conversations;
mod download;
mod embedding;
pub mod error;
mod eval_run_output_items;
mod eval_runs;
mod evals;
mod file;
mod fine_tuning;
mod image;
mod invites;
mod messages;
mod model;
mod moderation;
mod project_api_keys;
mod project_certificates;
mod project_rate_limits;
mod project_service_accounts;
mod project_users;
mod projects;
#[cfg(feature = "realtime")]
mod realtime;
mod responses;
mod runs;
mod speech;
mod steps;
mod threads;
pub mod traits;
mod transcriptions;
mod translations;
pub mod types;
mod uploads;
mod usage;
mod users;
mod util;
mod vector_store_file_batches;
mod vector_store_files;
mod vector_stores;
mod video;
#[cfg(feature = "webhook")]
pub mod webhooks;

pub use admin::Admin;
pub use admin_api_keys::AdminAPIKeys;
pub use assistants::Assistants;
pub use audio::Audio;
pub use audit_logs::AuditLogs;
pub use batches::Batches;
pub use certificates::Certificates;
pub use chat::Chat;
pub use chatkit::Chatkit;
pub use client::Client;
pub use completion::Completions;
pub use container_files::ContainerFiles;
pub use containers::Containers;
pub use conversation_items::ConversationItems;
pub use conversations::Conversations;
pub use embedding::Embeddings;
pub use eval_run_output_items::EvalRunOutputItems;
pub use eval_runs::EvalRuns;
pub use evals::Evals;
pub use file::Files;
pub use fine_tuning::FineTuning;
pub use image::Images;
pub use invites::Invites;
pub use messages::Messages;
pub use model::Models;
pub use moderation::Moderations;
pub use project_api_keys::ProjectAPIKeys;
pub use project_certificates::ProjectCertificates;
pub use project_rate_limits::ProjectRateLimits;
pub use project_service_accounts::ProjectServiceAccounts;
pub use project_users::ProjectUsers;
pub use projects::Projects;
#[cfg(feature = "realtime")]
pub use realtime::Realtime;
pub use responses::Responses;
pub use runs::Runs;
pub use speech::Speech;
pub use steps::Steps;
pub use threads::Threads;
pub use transcriptions::Transcriptions;
pub use translations::Translations;
pub use uploads::Uploads;
pub use usage::Usage;
pub use users::Users;
pub use vector_store_file_batches::VectorStoreFileBatches;
pub use vector_store_files::VectorStoreFiles;
pub use vector_stores::VectorStores;
pub use video::Videos;
