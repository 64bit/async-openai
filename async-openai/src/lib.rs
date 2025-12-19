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
//! use async_openai::{Client, types::responses::{CreateResponseArgs}};
//!
//! // Create client
//! let client = Client::new();
//!
//! // Create request using builder pattern
//! // Every request struct has companion builder struct with same name + Args suffix
//! let request = CreateResponseArgs::default()
//!     .model("gpt-5-mini")
//!     .input("tell me the recipe of pav bhaji")
//!     .max_output_tokens(512u32)
//!     .build()?;
//!
//! // Call API
//! let response = client
//!     .responses()      // Get the API "group" (responses, images, etc.) from the client
//!     .create(request)  // Make the API call in that "group"
//!     .await?;
//!
//! println!("{:?}", response.output_text());
//! # Ok::<(), Box<dyn std::error::Error>>(())
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
//!        .await?;
//!
//!  if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
//!     println!("{}", content);
//!  }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//!```
//!
//! **References: Borrow Instead of Move**
//!
//! With `byot` use reference to request types
//!
//! ```
//! # #[cfg(feature = "byot")]
//! # tokio_test::block_on(async {
//! # use async_openai::{Client, types::responses::{CreateResponse, Response}};
//! # let client = Client::new();
//! # let request = CreateResponse::default();
//! let response: Response = client
//!   .responses()
//!   .create_byot(&request).await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Rust Types
//!
//! To only use Rust types from the crate - use feature flag `types`.
//!
//! There are granular feature flags like `response-types`, `chat-completion-types`, etc.
//!
//! These granular types are enabled when the corresponding API feature is enabled - for example `response` will enable `response-types`.
//!
//! ## Configurable Requests
//!
//! **Individual Request**
//!
//! Certain individual APIs that need additional query or header parameters - these can be provided by chaining `.query()`, `.header()`, `.headers()` on the API group.
//!
//! For example:
//! ```
//! # tokio_test::block_on(async {
//! # use async_openai::Client;
//! # use async_openai::traits::RequestOptionsBuilder;
//! # let client = Client::new();
//! client
//!   .chat()
//!   // query can be a struct or a map too.
//!   .query(&[("limit", "10")])?
//!   // header for demo
//!   .header("key", "value")?
//!   .list()
//!   .await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! **All Requests**
//!
//! Use `Config`, `OpenAIConfig` etc. for configuring url, headers or query parameters globally for all requests.
//!
//! ## OpenAI-compatible Providers
//!
//! Even though the scope of the crate is official OpenAI APIs, it is very configurable to work with compatible providers.
//!
//! **Configurable Path**
//!
//! In addition to `.query()`, `.header()`, `.headers()` a path for individual request can be changed by using `.path()`, method on the API group.
//!
//! For example:
//! ```
//! # tokio_test::block_on(async {
//! # use async_openai::{Client, types::chat::CreateChatCompletionRequestArgs};
//! # use async_openai::traits::RequestOptionsBuilder;
//! # let client = Client::new();
//! # let request = CreateChatCompletionRequestArgs::default()
//! #     .model("gpt-4")
//! #     .messages([])
//! #     .build()
//! #     .unwrap();
//! client
//!   .chat()
//!   .path("/v1/messages")?
//!   .create(request)
//!   .await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! **Dynamic Dispatch**
//!
//! This allows you to use same code (say a `fn`) to call APIs on different OpenAI-compatible providers.
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
//! // create client
//! let client: Client<Box<dyn Config>> = Client::with_config(config);
//!
//! // A function can now accept a `&Client<Box<dyn Config>>` parameter
//! // which can invoke any openai compatible api
//! fn chat_completion(client: &Client<Box<dyn Config>>) {
//!     todo!()
//! }
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

#[cfg(all(feature = "_api", feature = "byot"))]
pub(crate) use async_openai_macros::byot;

#[cfg(all(feature = "_api", not(feature = "byot")))]
pub(crate) use async_openai_macros::byot_passthrough as byot;

// #[cfg(all(not(feature = "_api"), not(feature = "byot")))]
// #[macro_export]
// macro_rules! byot {
//     ($($tt:tt)*) => {
//         $($tt)*
//     };
// }

#[cfg(feature = "administration")]
mod admin;
#[cfg(feature = "assistant")]
mod assistants;
#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "batch")]
mod batches;
#[cfg(feature = "chat-completion")]
mod chat;
#[cfg(feature = "chatkit")]
mod chatkit;
#[cfg(feature = "_api")]
mod client;
#[cfg(feature = "completions")]
mod completion;
#[cfg(feature = "_api")]
pub mod config;
#[cfg(feature = "container")]
mod containers;
#[cfg(feature = "image")]
mod download;
#[cfg(feature = "embedding")]
mod embedding;
pub mod error;
#[cfg(feature = "evals")]
mod evals;
#[cfg(feature = "file")]
mod file;
#[cfg(feature = "finetuning")]
mod fine_tuning;
#[cfg(feature = "image")]
mod image;
#[cfg(feature = "_api")]
mod impls;
#[cfg(feature = "model")]
mod model;
#[cfg(feature = "moderation")]
mod moderation;
#[cfg(feature = "realtime")]
mod realtime;
#[cfg(feature = "_api")]
mod request_options;
#[cfg(feature = "responses")]
mod responses;
#[cfg(feature = "_api")]
pub mod traits;
pub mod types;
#[cfg(feature = "upload")]
mod uploads;
#[cfg(any(
    feature = "audio",
    feature = "file",
    feature = "upload",
    feature = "image",
    feature = "video",
    feature = "container"
))]
mod util;
#[cfg(feature = "vectorstore")]
mod vectorstores;
#[cfg(feature = "video")]
mod video;
#[cfg(feature = "webhook")]
pub mod webhooks;

// admin::* would be good - however its expanded here so that docs.rs shows the feature flags
#[cfg(feature = "administration")]
pub use admin::{
    Admin, AdminAPIKeys, AuditLogs, Certificates, GroupRoles, GroupUsers, Groups, Invites,
    ProjectAPIKeys, ProjectCertificates, ProjectGroupRoles, ProjectGroups, ProjectRateLimits,
    ProjectRoles, ProjectServiceAccounts, ProjectUserRoles, ProjectUsers, Projects, Roles, Usage,
    UserRoles, Users,
};
#[cfg(feature = "assistant")]
pub use assistants::{Assistants, Messages, Runs, Steps, Threads};
#[cfg(feature = "audio")]
pub use audio::{Audio, Speech, Transcriptions, Translations};
#[cfg(feature = "batch")]
pub use batches::Batches;
#[cfg(feature = "chat-completion")]
pub use chat::Chat;
#[cfg(feature = "chatkit")]
pub use chatkit::Chatkit;
#[cfg(feature = "_api")]
pub use client::Client;
#[cfg(feature = "completions")]
pub use completion::Completions;
#[cfg(feature = "container")]
pub use containers::{ContainerFiles, Containers};
#[cfg(feature = "embedding")]
pub use embedding::Embeddings;
#[cfg(feature = "evals")]
pub use evals::{EvalRunOutputItems, EvalRuns, Evals};
#[cfg(feature = "file")]
pub use file::Files;
#[cfg(feature = "finetuning")]
pub use fine_tuning::FineTuning;
#[cfg(feature = "image")]
pub use image::Images;
#[cfg(feature = "model")]
pub use model::Models;
#[cfg(feature = "moderation")]
pub use moderation::Moderations;
#[cfg(feature = "realtime")]
pub use realtime::Realtime;
#[cfg(feature = "_api")]
pub use request_options::RequestOptions;
#[cfg(feature = "responses")]
pub use responses::{ConversationItems, Conversations, Responses};
#[cfg(feature = "upload")]
pub use uploads::Uploads;
#[cfg(feature = "vectorstore")]
pub use vectorstores::{VectorStoreFileBatches, VectorStoreFiles, VectorStores};
#[cfg(feature = "video")]
pub use video::Videos;
