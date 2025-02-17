## Prerequisites

- [Ollama](https://github.com/ollama/ollama) should be installed and running.
- Pull a model to use with the library: `ollama pull <model>` e.g. `ollama pull llama3.2`
  - See [Ollama.com model search](https://ollama.com/search) for more information on the models available.
  - **You will need to pass the `<model>` name in the API call.**

## Ollama OpenAI Compatibility

**NOTE: an api key parameter is used for compatibility with OpenAI's API spec, but it is ignored by Ollama (it can be any value).**

See the [Ollama OpenAI Compatibility docs](https://github.com/ollama/ollama/blob/main/docs/openai.md) for more details on what Ollama supports.

## Response

> Response:
>
> 0: Role: assistant Content: Some("The 2020 World Series was held at Globe Life Field in Arlington, Texas, which is home of the Texas Rangers. Due to COVID-19 pandemic protocols and stadium capacity restrictions, the series featured a neutral-site venue rather than the traditional host-field advantage. The Dodgers defeated the Tampa Bay Rays 4 games to 2.")
