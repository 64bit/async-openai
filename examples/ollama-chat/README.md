## Setup

A docker compose file is provided to run a dockerized version of Ollama and download a default model. You will need the Ollama container to be up and running _before_ you can run the Rust example code.

You can check the container status with `docker ps` or check the container's logs with `docker container logs {CONTAINER NAME} -f`. E.g. `docker container logs ollama -f`.

## Running the Example

```sh
# Bring ollama up with model and wait for it to be healthy.
docker compose up -d

# Once model is downloaded and Ollama is up, run the Rust code.
cargo run
```

## Docker Notes

- Since Ollama requires you to pull a model before first use, a custom entrypoint script is used. See [Stack Overflow discussion](https://stackoverflow.com/a/78501628).
  - The model will be cached in the volumes dir.
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
