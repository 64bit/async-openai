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
<sub>Logo created by this <a href="https://github.com/64bit/async-openai/tree/main/examples/create-image-b64-json">repo itself</a></sub>
</div>

## Overview

`async-openai` is an unofficial Rust library for OpenAI.

- It's based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi)
- Current features:
  - [x] Assistants (Beta)
  - [x] Audio (Whisper/TTS)
  - [x] Chat
  - [x] Completions (Legacy)
  - [x] Edits (Deprecated)
  - [x] Embeddings
  - [x] Files
  - [x] Fine-Tuning
  - [x] Fine-Tunes (Deprecated)
  - [x] Images
  - [x] Microsoft Azure OpenAI Service
  - [x] Models
  - [x] Moderations
  - [x] WASM support (experimental and only available in [`experiments`](https://github.com/64bit/async-openai/tree/experiments) branch)
- Support SSE streaming on available APIs
- All requests including form submissions (except SSE streaming) are retried with exponential backoff when [rate limited](https://platform.openai.com/docs/guides/rate-limits) by the API server.
- Ergonomic builder pattern for all request objects.

**Note on Azure OpenAI Service (AOS)**:  `async-openai` primarily implements OpenAI spec, and doesn't try to maintain parity with spec of AOS.

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

- Visit [examples](https://github.com/64bit/async-openai/tree/main/examples) directory on how to use `async-openai`.
- Visit [docs.rs/async-openai](https://docs.rs/async-openai) for docs.

### Feature Flags
- `wasm`: Enables support for `wasm32-unknown-unknown` target
  - Disabling tokio support and backoff retries.
  - _Now_ at the cost of disabling all media related functionalities (audio transcription, image generation, etc.).
  - _Help wanted_ to re-enable media related functionalities and backoff retries.
- `backoff`: Enables backoff retries for all requests.
  - Enabled by default.
  - Disabling this feature will disable all retries.
- `tokio`: Enables support for `tokio` runtime.
  - Enabled by default.
  - _Now_ disabling this feature will disable all media related functionalities.

## Image Generation Example

```rust
use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
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
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

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

## Contributing

Thank you for your time to contribute and improve the project, I'd be happy to have you!

A good starting point would be existing [open issues](https://github.com/64bit/async-openai/issues).

## Complimentary Crates
- [openai-func-enums](https://github.com/frankfralick/openai-func-enums) provides procedural macros that make it easier to use this library with OpenAI API's tool calling feature. It also provides derive macros you can add to existing [clap](https://github.com/clap-rs/clap) application subcommands for natural language use of command line tools. It also supports openai's [parallel tool calls](https://platform.openai.com/docs/guides/function-calling/parallel-function-calling) and allows you to choose between running multiple tool calls concurrently or own their own OS threads.


## License

This project is licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
