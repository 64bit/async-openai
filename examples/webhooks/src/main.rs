//! OpenAI Webhook Example with Axum
//!
//! This example demonstrates a complete webhook integration:
//! 1. Starts an Axum server with webhook endpoint
//! 2. Automatically sends a background response request
//! 3. Receives and processes webhook events
//!
//! # Prerequisites
//!
//! 1. Start ngrok in a separate terminal:
//!    ```bash
//!    ngrok http 3000
//!    ```
//!
//! 2. Set your environment variables:
//!    ```bash
//!    export OPENAI_API_KEY="your-api-key"
//!    export OPENAI_WEBHOOK_SECRET="whsec_your_secret"
//!    ```
//!
//! 3. Configure the ngrok URL in OpenAI dashboard:
//!    https://platform.openai.com/settings/organization/webhooks
//!
//! 4. Run this example:
//!    ```bash
//!    cargo run --package webhooks
//!    ```
//!
//! The example will automatically send a background response request
//! and you'll see the webhook events being received!

use async_openai::traits::{EventId, EventType};
use async_openai::types::responses::{
    CreateResponseArgs, EasyInputContent, EasyInputMessage, InputItem, InputParam, MessageType,
    Role,
};
use async_openai::types::webhooks::WebhookEvent;
use async_openai::webhooks::Webhooks;
use async_openai::Client;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use std::sync::Arc;
use tracing::{error, info, warn};

/// Application state
#[derive(Clone)]
struct AppState {
    webhook_secret: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 Starting OpenAI Webhook Example");
    info!("");

    // Get webhook secret from environment
    let webhook_secret = std::env::var("OPENAI_WEBHOOK_SECRET").unwrap_or_else(|_| {
        warn!("⚠️  OPENAI_WEBHOOK_SECRET not set, using default test secret");
        warn!("   Set it with: export OPENAI_WEBHOOK_SECRET=\"whsec_your_secret\"");
        "test_secret".to_string()
    });

    let state = AppState {
        webhook_secret: webhook_secret.clone(),
    };

    // Build the router
    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(Arc::new(state));

    // Start the server
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("✅ Webhook server started on http://{}", addr);
    info!("📬 Webhook endpoint: http://{}/webhook", addr);
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📋 Setup Instructions:");
    info!("");
    info!("1. Make sure ngrok is running:");
    info!("   ngrok http 3000");
    info!("");
    info!("2. Configure the ngrok URL in OpenAI dashboard:");
    info!("   https://platform.openai.com/settings/organization/webhooks");
    info!("   Add your ngrok URL + /webhook");
    info!("   Example: https://abc123.ngrok.io/webhook");
    info!("");
    info!("3. Copy the webhook secret from OpenAI dashboard and set it:");
    info!("   export OPENAI_WEBHOOK_SECRET=\"whsec_...\"");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");
    info!("🔄 Sending background response request in 3 seconds...");

    // Spawn a task to send the background response request
    tokio::spawn(async move {
        // Wait for server to be ready
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        info!("");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📤 Sending background response request...");
        info!("");

        match send_background_response().await {
            Ok(response_id) => {
                info!("✅ Background response created successfully!");
                info!("   Response ID: {}", response_id);
                info!("   Waiting for webhook events...");
                info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                info!("");
            }
            Err(e) => {
                error!("❌ Failed to send background response: {}", e);
                error!("   Make sure your OPENAI_API_KEY is valid");
                error!("   and you have access to the Responses API");
            }
        }
    });

    // Start serving
    axum::serve(listener, app).await.unwrap();
}

/// Send a background response request to OpenAI using async-openai client
async fn send_background_response() -> Result<String, Box<dyn std::error::Error>> {
    // Create OpenAI client (will use OPENAI_API_KEY env var)
    let client = Client::new();

    info!("   Model: gpt-4o-mini");
    info!("   Prompt: \"What is the day today?\"");
    info!("   Background: true (to receive webhook events)");
    info!("");

    // Create a background response request
    let request = CreateResponseArgs::default()
        .model("gpt-4o-mini")
        .background(true) // Enable background processing to trigger webhooks
        .input(InputParam::Items(vec![InputItem::EasyMessage(
            EasyInputMessage {
                r#type: MessageType::Message,
                role: Role::User,
                content: EasyInputContent::Text("What is the day today?".to_string()),
            },
        )]))
        .build()?;

    // Send the request
    let response = client.responses().create(request).await?;

    Ok(response.id)
}

/// Handle incoming webhook requests
async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    info!("📥 Received webhook request");

    // Convert body to string
    let body_str = std::str::from_utf8(&body).map_err(|e| {
        error!("❌ Failed to parse body as UTF-8: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Extract signature header
    let signature = headers
        .get("webhook-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("❌ Missing openai-webhook-signature header");
            StatusCode::BAD_REQUEST
        })?;

    // Extract timestamp header
    let timestamp = headers
        .get("webhook-timestamp")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("❌ Missing openai-webhook-timestamp header");
            StatusCode::BAD_REQUEST
        })?;

    // extract webhook-id header
    let webhook_id = headers
        .get("webhook-id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("❌ Missing webhook-id header");
            StatusCode::BAD_REQUEST
        })?;

    info!("✅ Webhook ID: {}", webhook_id);

    // Verify signature and construct event
    let event = Webhooks::build_event(
        body_str,
        signature,
        timestamp,
        webhook_id,
        &state.webhook_secret,
    )
    .map_err(|e| {
        error!("❌ Webhook verification failed: {}", e);
        error!("   Signature: {}", signature);
        error!("   Timestamp: {}", timestamp);
        error!("   Webhook ID: {}", webhook_id);
        StatusCode::BAD_REQUEST
    })?;

    info!("✅ Webhook signature verified");

    // Process the event
    process_webhook_event(event);

    Ok(StatusCode::OK)
}

/// Process webhook events
fn process_webhook_event(event: WebhookEvent) {
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🎉 Processing Webhook Event");
    info!("   Type: {}", event.event_type());
    info!("   ID: {}", event.event_id());
    info!("   Timestamp: {}", event.created_at());
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    match event {
        // Batch events
        WebhookEvent::BatchCancelled(webhook) => {
            info!("📦 Batch Cancelled");
            info!("   Batch ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::BatchCompleted(webhook) => {
            info!("📦 Batch Completed");
            info!("   Batch ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::BatchExpired(webhook) => {
            info!("📦 Batch Expired");
            info!("   Batch ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::BatchFailed(webhook) => {
            info!("📦 Batch Failed");
            info!("   Batch ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }

        // Eval run events
        WebhookEvent::EvalRunCanceled(webhook) => {
            info!("🧪 Eval Run Canceled");
            info!("   Eval Run ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::EvalRunFailed(webhook) => {
            info!("🧪 Eval Run Failed");
            info!("   Eval Run ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::EvalRunSucceeded(webhook) => {
            info!("🧪 Eval Run Succeeded");
            info!("   Eval Run ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }

        // Fine-tuning events
        WebhookEvent::FineTuningJobCancelled(webhook) => {
            info!("🔧 Fine-Tuning Job Cancelled");
            info!("   Job ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::FineTuningJobFailed(webhook) => {
            info!("🔧 Fine-Tuning Job Failed");
            info!("   Job ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }
        WebhookEvent::FineTuningJobSucceeded(webhook) => {
            info!("🔧 Fine-Tuning Job Succeeded");
            info!("   Job ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
        }

        // Realtime events
        WebhookEvent::RealtimeCallIncoming(webhook) => {
            info!("📞 Realtime Call Incoming");
            info!("   Call ID: {}", webhook.data.call_id);
            info!("   SIP Headers:");
            for header in &webhook.data.sip_headers {
                info!("     {}: {}", header.name, header.value);
            }
            info!("   Event created at: {}", webhook.created_at);
        }

        // Response events (for background responses)
        WebhookEvent::ResponseCancelled(webhook) => {
            info!("💬 Response Cancelled");
            info!("   Response ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
            info!("");
            info!("   ℹ️  The background response was cancelled before completion.");
        }
        WebhookEvent::ResponseCompleted(webhook) => {
            info!("💬 Response Completed ✅");
            info!("   Response ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
            info!("");
            info!("   ℹ️  The background response has been completed successfully!");
        }
        WebhookEvent::ResponseFailed(webhook) => {
            info!("💬 Response Failed");
            info!("   Response ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
            info!("");
            info!("   ℹ️  The background response failed during processing.");
        }
        WebhookEvent::ResponseIncomplete(webhook) => {
            info!("💬 Response Incomplete");
            info!("   Response ID: {}", webhook.data.id);
            info!("   Event created at: {}", webhook.created_at);
            info!("");
            info!("   ℹ️  The background response was interrupted.");
        }
    }

    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");
}
