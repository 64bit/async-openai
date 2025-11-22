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

#[cfg(feature = "api")]
pub mod config;
pub mod error;
#[cfg(feature = "api")]
pub mod traits;
pub mod types;

#[cfg(feature = "api")]
mod request_options;

#[cfg(feature = "administration")]
mod admin;
#[cfg(feature = "administration")]
mod admin_api_keys;
#[cfg(feature = "assistants")]
mod assistants;
#[cfg(feature = "platform")]
mod audio;
#[cfg(feature = "administration")]
mod audit_logs;
#[cfg(feature = "platform")]
mod batches;
#[cfg(feature = "administration")]
mod certificates;
#[cfg(feature = "chat-completion")]
mod chat;
#[cfg(feature = "chatkit")]
mod chatkit;
#[cfg(feature = "api")]
mod client;
#[cfg(feature = "completions")]
mod completion;
#[cfg(feature = "container")]
mod container_files;
#[cfg(feature = "container")]
mod containers;
#[cfg(feature = "chat-completion")]
mod conversation_items;
#[cfg(feature = "chat-completion")]
mod conversations;
#[cfg(feature = "api")]
mod download;
#[cfg(feature = "platform")]
mod embedding;
#[cfg(feature = "platform")]
mod eval_run_output_items;
#[cfg(feature = "platform")]
mod eval_runs;
#[cfg(feature = "platform")]
mod evals;
#[cfg(feature = "platform")]
mod file;
#[cfg(feature = "platform")]
mod fine_tuning;
#[cfg(feature = "administration")]
mod group_roles;
#[cfg(feature = "administration")]
mod group_users;
#[cfg(feature = "administration")]
mod groups;
#[cfg(feature = "platform")]
mod image;
#[cfg(feature = "api")]
mod impls;
#[cfg(feature = "administration")]
mod invites;
#[cfg(feature = "assistants")]
mod messages;
#[cfg(feature = "platform")]
mod model;
#[cfg(feature = "platform")]
mod moderation;
#[cfg(feature = "administration")]
mod project_api_keys;
#[cfg(feature = "administration")]
mod project_certificates;
#[cfg(feature = "administration")]
mod project_group_roles;
#[cfg(feature = "administration")]
mod project_groups;
#[cfg(feature = "administration")]
mod project_rate_limits;
#[cfg(feature = "administration")]
mod project_roles;
#[cfg(feature = "administration")]
mod project_service_accounts;
#[cfg(feature = "administration")]
mod project_user_roles;
#[cfg(feature = "administration")]
mod project_users;
#[cfg(feature = "administration")]
mod projects;
#[cfg(feature = "realtime")]
mod realtime;
#[cfg(feature = "responses")]
mod responses;
#[cfg(feature = "administration")]
mod roles;
#[cfg(feature = "assistants")]
mod runs;
#[cfg(feature = "platform")]
mod speech;
#[cfg(feature = "assistants")]
mod steps;
#[cfg(feature = "assistants")]
mod threads;
#[cfg(feature = "platform")]
mod transcriptions;
#[cfg(feature = "platform")]
mod translations;
#[cfg(feature = "platform")]
mod uploads;
#[cfg(feature = "administration")]
mod usage;
#[cfg(feature = "administration")]
mod user_roles;
#[cfg(feature = "administration")]
mod users;
#[cfg(feature = "api")]
mod util;
#[cfg(feature = "vector-stores")]
mod vector_store_file_batches;
#[cfg(feature = "vector-stores")]
mod vector_store_files;
#[cfg(feature = "vector-stores")]
mod vector_stores;
#[cfg(feature = "platform")]
mod video;
#[cfg(feature = "webhook")]
pub mod webhooks;

#[cfg(feature = "api")]
pub use client::Client;
#[cfg(feature = "api")]
pub use request_options::RequestOptions;

#[cfg(feature = "administration")]
pub use admin::Admin;
#[cfg(feature = "administration")]
pub use admin_api_keys::AdminAPIKeys;
#[cfg(feature = "assistants")]
pub use assistants::Assistants;
#[cfg(feature = "platform")]
pub use audio::Audio;
#[cfg(feature = "administration")]
pub use audit_logs::AuditLogs;
#[cfg(feature = "platform")]
pub use batches::Batches;
#[cfg(feature = "administration")]
pub use certificates::Certificates;
#[cfg(feature = "chat-completion")]
pub use chat::Chat;
#[cfg(feature = "chatkit")]
pub use chatkit::Chatkit;
#[cfg(feature = "completions")]
pub use completion::Completions;
#[cfg(feature = "container")]
pub use container_files::ContainerFiles;
#[cfg(feature = "container")]
pub use containers::Containers;
#[cfg(feature = "responses")]
pub use conversation_items::ConversationItems;
#[cfg(feature = "responses")]
pub use conversations::Conversations;
#[cfg(feature = "platform")]
pub use embedding::Embeddings;
#[cfg(feature = "platform")]
pub use eval_run_output_items::EvalRunOutputItems;
#[cfg(feature = "platform")]
pub use eval_runs::EvalRuns;
#[cfg(feature = "platform")]
pub use evals::Evals;
#[cfg(feature = "platform")]
pub use file::Files;
#[cfg(feature = "platform")]
pub use fine_tuning::FineTuning;
#[cfg(feature = "administration")]
pub use group_roles::GroupRoles;
#[cfg(feature = "administration")]
pub use group_users::GroupUsers;
#[cfg(feature = "administration")]
pub use groups::Groups;
#[cfg(feature = "platform")]
pub use image::Images;
#[cfg(feature = "administration")]
pub use invites::Invites;
#[cfg(feature = "assistants")]
pub use messages::Messages;
#[cfg(feature = "platform")]
pub use model::Models;
#[cfg(feature = "platform")]
pub use moderation::Moderations;
#[cfg(feature = "administration")]
pub use project_api_keys::ProjectAPIKeys;
#[cfg(feature = "administration")]
pub use project_certificates::ProjectCertificates;
#[cfg(feature = "administration")]
pub use project_group_roles::ProjectGroupRoles;
#[cfg(feature = "administration")]
pub use project_groups::ProjectGroups;
#[cfg(feature = "administration")]
pub use project_rate_limits::ProjectRateLimits;
#[cfg(feature = "administration")]
pub use project_roles::ProjectRoles;
#[cfg(feature = "administration")]
pub use project_service_accounts::ProjectServiceAccounts;
#[cfg(feature = "administration")]
pub use project_user_roles::ProjectUserRoles;
#[cfg(feature = "administration")]
pub use project_users::ProjectUsers;
#[cfg(feature = "administration")]
pub use projects::Projects;
#[cfg(feature = "realtime")]
pub use realtime::Realtime;
#[cfg(feature = "responses")]
pub use responses::Responses;
#[cfg(feature = "administration")]
pub use roles::Roles;
#[cfg(feature = "assistants")]
pub use runs::Runs;
#[cfg(feature = "platform")]
pub use speech::Speech;
#[cfg(feature = "assistants")]
pub use steps::Steps;
#[cfg(feature = "assistants")]
pub use threads::Threads;
#[cfg(feature = "platform")]
pub use transcriptions::Transcriptions;
#[cfg(feature = "platform")]
pub use translations::Translations;
#[cfg(feature = "platform")]
pub use uploads::Uploads;
#[cfg(feature = "administration")]
pub use usage::Usage;
#[cfg(feature = "administration")]
pub use user_roles::UserRoles;
#[cfg(feature = "administration")]
pub use users::Users;
#[cfg(feature = "vector-stores")]
pub use vector_store_file_batches::VectorStoreFileBatches;
#[cfg(feature = "vector-stores")]
pub use vector_store_files::VectorStoreFiles;
#[cfg(feature = "vector-stores")]
pub use vector_stores::VectorStores;
#[cfg(feature = "platform")]
pub use video::Videos;
