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

## Overview

`async-openai` is an unofficial Rust library for OpenAI REST API.

- It's based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi)
- Current features:
  - [ ] Microsoft Azure Endpoints / AD Authentication
  - [x] Completions
  - [x] Edit
  - [ ] Embeddings
  - [ ] Fine-Tunning
  - [x] Image (Generation/Edit/Variation)
  - [x] Moderation


## Usage

The library reads [API key](https://beta.openai.com/account/api-keys) from the environment variable `OPENAI_API_KEY`.

```bash
export OPENAI_API_KEY='sk-...'
```

- Visit [examples](https://github.com/64bit/async-openai/tree/main/examples) directory on how to use `async-openai`.
- Visit [docs.rs/async-openai](https://docs.rs/async-openai) for docs.

## Image Generation Example

```rust
use std::error::Error;

use async_openai as openai;
use openai::{
    types::{CreateImageRequest, ImageSize, ResponseFormat},
    Client, Image,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();

    let request = CreateImageRequest {
        prompt: "cats on sofa and carpet in living room".to_owned(),
        n: Some(2),
        response_format: Some(ResponseFormat::Url),
        size: Some(ImageSize::S256x256),
        user: Some("async-openai".to_owned()),
    };

    let response = Image::create(&client, request).await?;

    // download and save images to ./data directory
    // (creates directory when it doesn't exist)
    response.save("./data").await?;

    Ok(())
}
```

<div align="center">
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-1.png" />
  <img width="315" src="https://raw.githubusercontent.com/64bit/async-openai/assets/create-image/img-2.png" />
  <br/>
  <sub>Scaled up for README, actual size 256x256</sub>
</div>


## License

This project is licensed under [MIT license](https://github.com/64bit/async-openai/blob/main/LICENSE).
