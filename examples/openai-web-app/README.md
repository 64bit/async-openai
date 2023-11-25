# OpenAI Web App

This builds a `dioxus` web App that uses Azure OpenAI Services or OpenAI to generate text.

To run it, you need:
1. Set `USE_AZURE` in `./src/main.rs`
2. Set Azure OpenAI or OpenAI secrets in `./src/main.rs`. Please do NOT take this demo into production without using a secure secret store
3. Install `dioxus-cli` by `cargo install dioxus-cli --locked`.
4. Run `dx serve`

Note: Safari may not work due to CORS issues. Please use Chrome or Edge.