# WASM Response API Example

Minimal web app using Dioxus and async-openai's Response API.


## Running

1. Install Dioxus CLI: `cargo install dioxus-cli`
2. Run: `dx serve -p wasm-responses`
3. Open http://127.0.0.1:8080
4. Enter your OpenAI API key and message

## Bundling for web

```bash
dx bundle --web -p wasm-responses
```

Output will be in `dist/`


