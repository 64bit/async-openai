<div align="center">
  <a href="https://docs.rs/async-openai">
  <img width="50px" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image-b64-json/img-1.png" />
  </a>
</div>
<h1 align="center"> async-openai </h1>
<p align="center"> Async Rust library for OpenAI </p>
<div align="center">
    <a href="https://crates.io/crates/async-openai">
    <img src="https://img.shields.io/crates/v/async-openai.svg" />
    </a>
    <a href="https://docs.rs/async-openai">
    <img src="https://docs.rs/async-openai/badge.svg" />
    </a>
</div>
<div align="center">
<sub>Logo created by this <a href="https://github.com/64bit/async-openai/tree/main/examples/image-generate-b64-json">repo itself</a></sub>
</div>

## Overview

`async-openai` is an unofficial Rust library for OpenAI, based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi). It implements all APIs from the spec:

| What | APIs | Crate Feature Flags |
|---|---|---|
| **Responses API** | Responses, Conversations, Streaming events | `responses` |
| **Webhooks** | Webhook Events | `webhook` |
| **Platform APIs** | Audio, Audio Streaming, Videos, Images, Image Streaming, Embeddings, Evals, Fine-tuning, Graders, Batch, Files, Uploads, Models, Moderations | `audio`, `video`, `image`, `embedding`, `evals`, `finetuning`, `grader`, `batch`, `file`, `upload`, `model`, `moderation` |
| **Vector stores** | Vector stores, Vector store files, Vector store file batches | `vectorstore` |
| **ChatKit** <sub>(Beta)</sub> | ChatKit | `chatkit` |
| **Containers** | Containers, Container Files | `container` |
| **Skills** | Skills | `skill` |
| **Realtime** | Realtime Calls, Client secrets, Client events, Server events | `realtime` |
| **Chat Completions** | Chat Completions, Streaming | `chat-completion` |
| **Assistants** <sub>(Beta)</sub> | Assistants, Threads, Messages, Runs, Run steps, Streaming | `assistant` |
| **Administration** | Admin API Keys, Invites, Users, Groups, Roles, Role assignments, Projects, Project users, Project groups, Project service accounts, Project API keys, Project rate limits, Audit logs, Usage, Certificates | `administration` |
| **Legacy** | Completions | `completions` |

Features that makes `async-openai` unique:
- Bring your own custom types for Request or Response objects.
- SSE streaming on available APIs.
- Customize path, query and headers per request; customize path and headers globally (for all requests).
- Requests (except SSE streaming) including form submissions are retried with exponential backoff when [rate limited](https://platform.openai.com/docs/guides/rate-limits).
- Ergonomic builder pattern for all request objects.
- Granular feature flags to enable any types or apis: good for faster compilation and crate reuse.
- Microsoft Azure OpenAI Service (only for APIs matching OpenAI spec).
- WASM (doesn't include streaming and retry support yet)

## Usage

The library reads [API key](https://platform.openai.com/account/api-keys) from the environment variable `OPENAI_API_KEY`.

```bash
# On macOS/Linux
export OPENAI_API_KEY='sk-...'
```

```powershell
# On Windows Powershell
$Env:OPENAI_API_KEY='sk-...'
```

Other official environment variables supported are: `OPENAI_ADMIN_KEY`, `OPENAI_BASE_URL`, `OPENAI_ORG_ID`, `OPENAI_PROJECT_ID`

- Visit [examples](https://github.com/64bit/async-openai/tree/main/examples) directory on how to use `async-openai`.
- Visit [docs.rs/async-openai](https://docs.rs/async-openai) for docs.


## Image Generation Example

```rust
use async_openai::{
    types::images::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt("cats on sofa and carpet in living room")
        .n(2)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().generate(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
```

<div align="center">
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-1.png" />
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-2.png" />
  <br/>
  <sub>Scaled up for README, actual size 256x256</sub>
</div>

## Webhooks

Support for webhook includes event types, signature verification, and building webhook events from payloads.

## Bring Your Own Types

Enable methods whose input and outputs are generics with `byot` feature. It creates a new method with same name and `_byot` suffix. 

`byot` requires trait bounds: 
- a request type (`fn` input parameter) needs to implement `serde::Serialize` or `std::fmt::Display` trait
- a response type (`fn` ouput parameter) needs to implement `serde::de::DeserializeOwned` trait.

For example, to use `serde_json::Value` as request and response type:
```rust
let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "developer",
                    "content": "You are a helpful assistant"
                },
                {
                    "role": "user",
                    "content": "What do you think about life?"
                }
            ],
            "model": "gpt-4o",
            "store": false
        }))
        .await?;
```

This can be useful in many scenarios:
- To use this library with other OpenAI compatible APIs whose types don't exactly match OpenAI. 
- Extend existing types in this crate with new fields with `serde` (for example with `#[serde(flatten)]`).
- To avoid verbose types.
- To escape deserialization errors.

Visit [examples/bring-your-own-type](https://github.com/64bit/async-openai/tree/main/examples/bring-your-own-type)
directory to learn more.

### References: Borrow Instead of Move

With `byot` use reference to request types

```rust
let response: Response = client
  .responses()
  .create_byot(&request).await?
```

Visit [examples/borrow-instead-of-move](https://github.com/64bit/async-openai/tree/main/examples/borrow-instead-of-move) to learn more.


## Rust Types

To only use Rust types from the crate - disable default features and use feature flag `types`. 

There are granular feature flags like `response-types`, `chat-completion-types`, etc.

These granular types are enabled when the corresponding API feature is enabled - for example `responses` will enable `response-types`.

## Configurable Requests

### Individual Request
Certain individual APIs that need additional query or header parameters - these can be provided by chaining `.query()`, `.header()`, `.headers()` on the API group. 

For example:
```rust
client.
  .chat()
  // query can be a struct or a map too.
  .query(&[("limit", "10")])?
  // header for demo
  .header("key", "value")?
  .list()
  .await?
```

### All Requests

Use `Config`, `OpenAIConfig` etc. for configuring url, headers or query parameters globally for all requests.

## OpenAI-compatible Providers

Even though the scope of the crate is official OpenAI APIs, it is very configurable to work with compatible providers.

### Configurable Path

In addition to  `.query()`, `.header()`, `.headers()` a path for individual request can be changed by using `.path()`,  method on the API group.

For example:

```rust
client
  .chat()
  .path("/v1/messages")?
  .create(request)
  .await?
```

### Dynamic Dispatch

This allows you to use same code (say a `fn`) to call APIs on different OpenAI-compatible providers.

For any struct that implements `Config` trait, wrap it in a smart pointer and cast the pointer to `dyn Config`
trait object, then create a client with `Box` or `Arc` wrapped configuration.

For example:

```rust
use async_openai::{Client, config::{Config, OpenAIConfig}};

// Use `Box` or `std::sync::Arc` to wrap the config
let config = Box::new(OpenAIConfig::default()) as Box<dyn Config>;
// create client
let client: Client<Box<dyn Config>> = Client::with_config(config);

// A function can now accept a `&Client<Box<dyn Config>>` parameter
// which can invoke any openai compatible api
fn chat_completion(client: &Client<Box<dyn Config>>) { 
    todo!() 
}
```

## Middleware

Enable the `middleware` feature when you want `async-openai` to participate in the tower ecosystem.
This is an advanced integration point, so the API groups stay focused on request construction and
response parsing while transport policy lives in your tower stack.

The transport boundary is:

```text
Your code
  |
  v
API group methods  (client.chat(), client.models(), ...)
  |
  v
Request factory  (rebuilds the request when retry needs it)
  |
  v
Your tower layers
  |
  v
ReqwestService or your own transport service
  |
  v
reqwest::Response
  |
  v
async-openai response parsing / SSE parsing
```

That boundary was chosen for two reasons:

1. API group methods should stay focused on OpenAI request construction. They should not know about
   retries, rate limits, or service composition.
2. Retry-safe execution needs a replayable request boundary. If the request is built too early,
   multipart/file bodies and other non-cloneable payloads cannot be retried safely. If the response
   is parsed too early, middleware can no longer operate on the transport in a generic way.

### Using the Default reqwest Transport

If you want tower layers but still want the default reqwest transport, start with `ReqwestService`
and compose your layers around it.

```rust
use async_openai::{
    config::OpenAIConfig,
    Client,
    middleware::ReqwestService,
    retry::SimpleRetryPolicy,
};
use tower::{util::BoxCloneSyncService, ServiceBuilder};

let base = ReqwestService::new(reqwest::Client::new());

let service = ServiceBuilder::new()
    .concurrency_limit(8)
    .retry(SimpleRetryPolicy::default())
    .service(base);

let service = BoxCloneSyncService::new(service);

let client = Client::with_config(
    OpenAIConfig::new()
        .with_api_base("https://api.openai.com/v1")
        .with_api_key("sk-..."),
)
.with_http_service(service);
```

If you need request interception or a test double, replace `ReqwestService` with your own
`tower::Service<HttpRequestFactory, Response = reqwest::Response>`.

### Using `service_fn`

If you want full control over request handling, you can build a service from scratch with
`tower::service_fn`.

```rust
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    Client,
    middleware::HttpRequestFactory,
};
use http::Response as HttpResponse;
use tower::{service_fn, util::BoxCloneSyncService, ServiceBuilder};

let service = ServiceBuilder::new()
    .concurrency_limit(8)
    .service(service_fn(move |factory: HttpRequestFactory| async move {
        let request = factory.build().await?;

        // Inspect the request, route it somewhere else, or return a synthetic
        // response for tests.

        let response = HttpResponse::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(reqwest::Body::from(r#"{"object":"list","data":[]}"#))
            .unwrap();

        Ok::<reqwest::Response, OpenAIError>(response.into())
    }));

let service = BoxCloneSyncService::new(service);

let client = Client::with_config(
    OpenAIConfig::new()
        .with_api_base("https://api.openai.com/v1")
        .with_api_key("sk-..."),
)
.with_http_service(service);
```

### What Lives Where

```text
async-openai
  - API group methods
  - request construction
  - OpenAI error decoding
  - SSE parsing

middleware / tower
  - retry
  - timeout
  - concurrency limit
  - rate limit
  - tracing / metrics
  - request interception
```

The public retry policy, `SimpleRetryPolicy`, is exposed so you can place it anywhere in your tower
stack. It retries rate limits (`429`), server errors (`5xx`), and native connect errors; it
intentionally does not retry timeouts because many API calls are non-idempotent `POST` requests.
Use `async_openai::retry::should_retry` if you want to reuse the same classification in a
custom tower retry policy.

## Contributing

🎉 Thank you for taking the time to contribute and improve the project. I'd be happy to have you!

Please see [contributing guide!](https://github.com/64bit/async-openai/blob/main/CONTRIBUTING.md)


## Complimentary Crates
- [async-openai-wasm](https://github.com/ifsheldon/async-openai-wasm) provides WASM support.
- [openai-func-enums](https://github.com/frankfralick/openai-func-enums) macros for working with function/tool calls.

## License

This project is licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
