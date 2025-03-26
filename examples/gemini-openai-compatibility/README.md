# Gemini's OpenAI Compatibility Example

This example demonstrates how to use OpenAI's `async_openai` Rust library with Google's Gemini API. By modifying a few lines of configuration, you can integrate Gemini models while maintaining OpenAI compatibility.

## Features
- **List Available Models**: Fetch a list of supported Gemini models.
- **Retrieve Model Details**: Get information about a specific model.
- **Chat Completion**: Perform standard chat completions.
- **Stream Chat Responses**: Receive streaming responses for chat queries.
- **Generate Images**: Use Gemini's image generation API.

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


## Running the Example
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

## References
- [Google Gemini's OpenAI compatibility](https://ai.google.dev/gemini-api/docs/openai)


