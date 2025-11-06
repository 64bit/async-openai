# Webhooks Example

This example demonstrates how to handle OpenAI webhook events using the `async-openai` library, including signature verification.

## Feature Requirement

This example requires the `webhook` feature to be enabled:

```toml
[dependencies]
async-openai = { version = "*", features = ["webhook"] }
```



## Running the Example

This example automatically:
1. Starts an Axum webhook server
2. Sends a background response request
3. Receives and displays webhook events

### Quick Start (3 Simple Steps!)

**Step 1: Start ngrok** (in a separate terminal)
```bash
ngrok http 3000
```

You'll see output like:
```
Forwarding    https://abc123.ngrok.io -> http://localhost:3000
```

**Step 2: Configure webhook in OpenAI Dashboard**

1. Go to https://platform.openai.com/settings/organization/webhooks
2. Click "Add endpoint"
3. Enter your ngrok URL + `/webhook`:
   ```
   https://abc123.ngrok.io/webhook
   ```
4. Copy the webhook secret (starts with `whsec_`)

**Step 3: Run the example**
```bash
# Set your environment variables
export OPENAI_API_KEY="your-api-key"
export OPENAI_WEBHOOK_SECRET="whsec_your_secret_from_dashboard"

# Run the example
cargo run --package webhooks
```




