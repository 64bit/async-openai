## Sync Upstream OpenAPI spec

- Download upstream OpenAI OpenAPI spec from https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml and update the openapi.documented.yml file in this git repo

- Use git diff to see new changes, sometime these changes can be large. Use diff efficiently to update the Rust code - such as api updates, new apis, new types, update existing types, also do same for doc comments

- after updates make sure all feature flags and examples compiles