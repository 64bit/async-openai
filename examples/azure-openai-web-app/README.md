# Azure OpenAI Web App

This builds a `dioxus` web App that uses Azure OpenAI Services to generate text.

To run it, you need:
1. Set Azure OpenAI secrets in `./src/main.rs`. Please do NOT take this demo into production without using a secure secret store
2. Install `dioxus-cli` by `cargo install dioxus-cli --locked`.
3. Run `dx serve`

Note: Safari may not work due to CORS issues. Please use Chrome or Edge.