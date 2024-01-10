# Example: Cloudflare Worker Using `async-openai`

A template for kick starting a Cloudflare worker project using [`workers-rs`](https://github.com/cloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to
Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Setup

1. Move this directory out of `async-openai/examples` and out of `async-openai`
    * Since the `wasm-bindgen` dependency conflicts with `examples/openai-web-app`, we need to move this directory out
      of `async-openai/examples` and out of `async-openai` to avoid conflicts.
2. Run `npm install` to install `wrangler`.
3. Make sure you have `wasm32-unknown-unknown` target installed. You can install it by
   running `rustup target add wasm32-unknown-unknown`.
4. IMPORTANT: replace the values of `AUTH`, `OPENAI_API_KEY` with your own values in [code](./src/lib.rs).
5. Run `npx wrangler dev` to start a local server for testing!

Please do NOT simply deploy this demo to production without adding a secure secret store and appropriate authentication.

## Usage

This worker accepts the following paths:

* GET `/` or `/help` or `readme` to get this README
* POST `/chat`
    * Requires `x-api-key` header with the value of `AUTH` in [code](./src/lib.rs)
    * Requires the payload to be a JSON file like:
      ```json
      {
        "content": "Hello!"
      }
      ```

## More Information

* https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/
* https://docs.rs/worker