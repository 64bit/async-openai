# Gemini's OpenAI Compatibility Example

This example demonstrates how to use OpenAI's `async_openai` Rust library with Google's Gemini API. By modifying a few lines of configuration, you can integrate Gemini models while maintaining OpenAI compatibility.

## Features
- **List Available Models**: Fetch a list of supported Gemini models.
- **Retrieve Model Details**: Get detailed information about the `gemini-1.5-flash` model.
- **Chat Completion**: Perform chat completions using Gemini's API.
- **Stream Chat Messages**: Receive streaming responses for chat queries in real-time.
- **Generate Images**: Leverage Gemini's image generation capabilities.
- **Understand Images**: Analyze and extract information from images.
- **Understand Audio**: Process and interpret audio inputs.
- **Structured Output Response**: Generate structured outputs for complex queries.
- **Function Calling**: Invoke functions dynamically based on input prompts.
- **Create Embeddings**: Generate embeddings for text or other data types.
- **Bring Your Own Type (BYOT)**: Use custom Gemini response types defined in `gemini_type.rs`.

## Prerequisites
- Rust installed (`rustc` and `cargo`)
- Set up your Google Gemini API key from [Google AI Studio](https://aistudio.google.com/)
- Create a `.env` file with:
  ```plaintext
  GEMINI_API_KEY=your_api_key_here
  ```
- Install dependencies:
  ```sh
  cargo add async-openai dotenv futures tokio
  ```

## Enabling BYOT Feature
To enable the BYOT (Bring Your Own Type) feature in `async-openai`, modify your `Cargo.toml` as follows:
```toml
async-openai = {version = '{{version}}', features = ["byot"]}
```

## Usage
This example now uses the `byot` (Bring Your Own Type) feature to define custom types for Gemini responses. The Gemini types are defined in `gemini_type.rs`, and methods using these types have the `_byot` suffix.

### Running the Example
To run the example:
```sh
cargo run
```
This will:
1. List available models
2. Retrieve details of `gemini-1.5-flash`
3. Generate chat completion responses
4. Stream chat messages
5. Generate an image
6. Understanding an image
7. Understanding an audio
8. Structured output response
9. Function calling
10. Create Embeddings


## References
- [Google Gemini's OpenAI compatibility](https://ai.google.dev/gemini-api/docs/openai)

