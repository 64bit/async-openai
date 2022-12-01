<h1 align="center"> async-openai </h1>
<p align="center"> Async Rust library for OpenAI </p>

## Overview

`async-openai` is an unofficial Rust library for OpenAI REST API.

- It's based on [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi)
- Currently supported APIs:
  - [x] Completions
  - [x] Edit
  - [ ] Embeddings
  - [ ] Fine-Tunning
  - [x] Image (Generation/Edit/Variation)
  - [x] Moderation


## Usage

The library reads API key from the environment variable `OPENAI_API_KEY`. API key is available from [API Keys page on OpenAI account](https://beta.openai.com/account/api-keys)

```bash
export OPENAI_API_KEY='sk-...'
```

The [examples](./examples/) directory contains various examples on how to use `async-openai`. For library documentation visit [docs.rs/async-openai](https://docs.rs/async-openai)

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

## License

