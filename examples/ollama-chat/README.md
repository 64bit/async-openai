## Docker Notes

- Since Docker's build context cannot access parent directorie(s), the [async-openai](https://crates.io/crates/async-openai) crate is pulled from crates.io instead of from the local project parent directory.
  - The alternative is to create the docker compose / docker image files at the project root, but that would make the example no longer self contained.
  - See ["Adding files from the build context from the Docker docs"](https://docs.docker.com/reference/dockerfile/#adding-files-from-the-build-context)
- Since Ollama requires you to pull a model before first use, a custom entrypoint script is used. See [Stack Overflow discussion](https://stackoverflow.com/a/78501628).
  - The model will be cached in the volumes dir.
  - A healthcheck is then used to have the Rust container wait to start until after the Ollama model is loaded.
    - Depending on your network connection, the healthcheck may need to be adjusted to allow more time for the model to download.
- [llama3.2:1b](https://ollama.com/library/llama3.2:1b) is used in the example as it is a smaller model and will download more quickly compared to larger models.
  - A larger model will provide better responses, but be slower to download.
  - Also, using the default CPU inference, smaller models will have better tokens / second performance.
- The GPU mapping is written but commented out. This means it will default to CPU inference which is slower, but should run without any additional setup.
  - If you have a GPU and the proper container support, feel free to uncomment / adapt.

## Ollama OpenAI Compatibility

**NOTE: an api key parameter is used for compatibility with OpenAI's API spec, but it is ignored by Ollama (it can be any value).**

See the [Ollama OpenAI Compatibility docs](https://github.com/ollama/ollama/blob/main/docs/openai.md) for more details on what Ollama supports.

## Response

> Response:
>
> 0: Role: assistant Content: Some("The 2020 World Series was played at Globe Life Field in Arlington, Texas, as part of Major League Baseball's (MLB) move to play its season without spectators due to the COVID-19 pandemic. The Dodgers defeated the Tampa Bay Rays in 6 games.")
