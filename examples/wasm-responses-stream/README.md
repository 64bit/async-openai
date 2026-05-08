# WASM Response API Streaming Example

Minimal web app using Dioxus and async-openai's streaming Responses API.

## Running

1. Install Dioxus CLI: `cargo install dioxus-cli`
2. Run: `dx serve -p wasm-responses-stream`
3. Open http://127.0.0.1:8080
4. Enter your OpenAI API key and message

## Bundling for web

```bash
dx bundle --web -p wasm-responses-stream
```

Output will be in `dist/`.
