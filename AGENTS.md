## Sync Upstream OpenAPI spec

- Download upstream OpenAI OpenAPI spec from https://raw.githubusercontent.com/openai/openai-openapi/refs/heads/main/openapi.yaml and update the openapi.yaml file in this git repo
- Use git diff to see new changes, sometime these changes can be large.
- Use diff efficiently to update the Rust code - such as api updates, new apis, new types, update existing types, also do same for doc comments.
- use "git diff" and write python code to avoid running out of context to efficiently generate updates to rust code.
- after updates make sure all feature flags and examples compiles
- these changes should be in a new branch to create a pull request
- Do not implement new Beta APIs or their types, endpoint wrappers, or feature flags when syncing the spec. Keep the downloaded upstream spec intact.
- Represent `oneOf` unions with a shared discriminator as internally tagged Rust enums, using the discriminator key (for example, `#[serde(tag = "type")]` with correct rename_all if applicable). Avoid duplicate discriminator fields in variant payloads.
- Use `u64` for new Unix timestamp fields such as `created_at` and `expires_at`, and `Option<u64>` when optional.
- Keep types shared across type modules in the private `types/shared` module and explicitly re-export them from the public modules that use them; do not expose a separate public common-types module.

Deprecation:
- The Assistants API was officially sunset on August 26, 2026, and we removed code from this crate, do not implement them again.
- do not update previously deprecated and legacy apis and types like Realtime Beta and Completions
- When a previously deprecated API no longer exist in spec remove it from the code
- Mark code with deprecated when updates in spec introduces new deprecation