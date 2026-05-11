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

`async-openai` is an unofficial Rust library for OpenAI, based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi). It implements all APIs from the spec.

<details>
<summary>Feature Flags</summary>

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

</details>


**OpenAI**
  - Requests are retried with exponential backoff when [rate limited](https://platform.openai.com/docs/guides/rate-limits).
  - Ergonomic builder pattern for all request objects.
  - SSE streaming.
  - Granular feature flags to enable any types or apis.
  - WASM.
  - Middleware support with [tower](https://crates.io/crates/tower) ecosystem.

**+ OpenAI compatible providers**
  - Bring your own custom types for Request or Response objects.
  - Customize path, query and headers per request or for all requests.
  - Microsoft Azure OpenAI Service.

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

## OpenAI Compatible Providers

Even though the scope of the crate is official OpenAI APIs, it is very configurable to work with compatible providers.

### Bring Your Own Types

Enable methods whose input and outputs are generics with `byot` feature. It creates a new method with same name and `_byot` suffix. 

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
- When shape of request/response in OpenAI-compatible APIs don't exactly match OpenAI. 
- Extend existing types in this crate with new fields like `extra_body` (with serde flatten)
- To avoid typing verbose types.
- To escape deserialization errors on expected type and actual response mismatch.

`*_byot` methods require same trait bounds as regular methods.

Visit [examples/bring-your-own-type](https://github.com/64bit/async-openai/tree/main/examples/bring-your-own-type)
directory to learn more.

#### References: Borrow Instead of Move

With `byot` use reference to request types

```rust
let response: Response = client
  .responses()
  .create_byot(&request).await?
```

Visit [examples/borrow-instead-of-move](https://github.com/64bit/async-openai/tree/main/examples/borrow-instead-of-move) to learn more.


### Configurable Requests

Configure path, headers, and query parameters for a HTTP request.

#### Request Options
Use `path()`, `.query()`, `.header()`, `.headers()` on the API group. Path overrides the default path but all other methods are additive - adds to existing query or headers.

For demonstration:
```rust
client.
  .chat()
  // override default path
  .path("/v1/messages")
  // query can be a struct or a map too - additive
  .query(&[("limit", "10")])?
  // header for unique id for this API request - additive
  .header("x-request-id", "id123")?
  .list()
  .await?
```

#### Modifying all Requests

Use `Config`, `OpenAIConfig` etc. for configuring url, headers or query parameters globally for all requests.


### Dynamic Dispatch

This allows you to use same code (say a `fn`) to call APIs on different OpenAI-compatible providers.

Create a client with `Box` or `Arc` wrapped configuration.

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

## Rust Types

To only use Rust types from the crate - disable default features and use feature flag `types`. 

There are granular feature flags like `response-types`, `chat-completion-types`, etc.

These granular types are enabled when the corresponding API feature is enabled - for example `responses` will enable `response-types`.

## Webhooks

Support for webhook includes event types, signature verification, and building webhook events from payloads.

## Middleware

Middleware is supported via Tower ecosystem, which can be enabled with `middleware` feature. See [middleware](https://github.com/64bit/async-openai/blob/main/async-openai/MIDDLEWARE.md) for more detail.

## Contributing

🎉 Thank you for taking the time to contribute and improve the project. I'd be happy to have you!

Please see [contributing guide!](https://github.com/64bit/async-openai/blob/main/CONTRIBUTING.md)


## Complimentary Crates
- [async-openai-wasm](https://github.com/ifsheldon/async-openai-wasm) provides WASM support.
- [openai-func-enums](https://github.com/frankfralick/openai-func-enums) macros for working with function/tool calls.

## License

This project is licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
