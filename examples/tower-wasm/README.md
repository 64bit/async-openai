# Tower WASM Example

Minimal Dioxus web app using async-openai's Response API through the `middleware` feature.

The default `SimpleRetryPolicy` retries immediately on wasm because wasm has no universal timer runtime. On wasm it retries rate limits only. If you need delayed backoff, compose a tower layer that is compatible with your wasm runtime.

## Running

1. Install Dioxus CLI: `cargo install dioxus-cli`
2. Run: `dx serve -p tower-wasm`
3. Open http://127.0.0.1:8080
4. Enter your OpenAI API key and message

## Bundling For Web

```bash
dx bundle --web -p tower-wasm
```

Output will be in `dist/`.
