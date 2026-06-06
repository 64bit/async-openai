# RateLimitLayer Design

## Overview

A Tower middleware layer that provides client-side RPM (Requests Per Minute) rate limiting using a leaky bucket algorithm (GCRA via the `governor` crate), combined with server-side feedback backpressure using OpenAI's `x-ratelimit-*` response headers.

## Motivation

The current rate limit handling in async-openai relies on exponential backoff after receiving 429 responses. This is reactive - requests are sent first, then retried when rejected. For large batches, this wastes requests and time. A proactive approach using a local rate limiter prevents exceeding limits in the first place, while server-side header feedback adapts to actual tier limits when users configure RPM higher than their tier allows.

## Goals

- Proactive RPM limiting using governor (GCRA / leaky bucket)
- Server-side backpressure via `x-ratelimit-remaining-requests` and `x-ratelimit-reset-requests` headers
- Compatible with WASM targets
- Zero impact on users who don't use it (optional middleware, gated behind `rate-limit` feature)
- Works correctly under tower's clone-based service model

## Non-Goals

- TPM (Tokens Per Minute) limiting - requires extracting token usage from response bodies, separate design needed
- Per-model keyed limits - users can compose at the application layer with one client per model
- Precise governor bucket sync from `remaining-requests` - backpressure is sufficient for v1

## Public API

```rust
use async_openai::middleware::rate_limit::RateLimitLayer;
use async_openai::middleware::{ReqwestService, retry::OpenAIRetryLayer};

let service = tower::ServiceBuilder::new()
    .layer(OpenAIRetryLayer::default())
    .layer(RateLimitLayer::per_minute(60))
    .service(ReqwestService::new(reqwest::Client::new()));

let client = Client::with_config(OpenAIConfig::default())
    .with_http_service(service);
```

Single constructor: `RateLimitLayer::per_minute(rpm: u32)`. Header sync and backpressure are automatic.

## Middleware Stack Position

```
HttpRequestFactory
       |
       v
  OpenAIRetryLayer        <- retries on 429/5xx with exponential backoff
       |
       v
  RateLimitLayer          <- proactive limiting + server-side backpressure
       |
       v
  ReqwestService          <- actual HTTP transport
```

RateLimitLayer sits below OpenAIRetryLayer so that retries also go through rate limiting. Without this ordering, retries could bypass the limiter and exceed the configured RPM.

## Request Flow

1. Request arrives at retry layer, which calls down to rate limit layer
2. `RateLimitService::poll_ready` checks two conditions:
   - Governor local bucket has a token available
   - `ServerBackpressure` reset time has passed (or is not set)
   - If either condition fails, returns `Pending` - request stalls here
3. When `poll_ready` returns `Ready`, `call` forwards the request to the inner service
4. Response future intercepts the response, parses rate limit headers, updates `ServerBackpressure`

## Retry Flow

When a 429 is received, the retry layer backs off and retries. On retry, it calls `poll_ready` on the rate limit layer again. If the local bucket is empty or server backpressure is active, the retry is also throttled. Retries cannot bypass the limiter.

## Internal Components

### RateLimitLayer / RateLimitService

```rust
pub struct RateLimitLayer {
    limiter: RateLimiter<NotKeyed, InMemoryState, QuantaInstant>,
    backpressure: Arc<ServerBackpressure>,
}

pub struct RateLimitService<S> {
    inner: S,
    limiter: RateLimiter<NotKeyed, InMemoryState, QuantaInstant>,
    backpressure: Arc<ServerBackpressure>,
}
```

Governor's `RateLimiter` is `Clone` and internally uses `Arc` for state sharing, so all cloned `RateLimitService` instances share the same bucket.

### ServerBackpressure

```rust
struct ServerBackpressure {
    reset_at: AtomicU64,  // UNIX timestamp in milliseconds, 0 = no limit
}
```

Uses `AtomicU64` instead of `Mutex` because:
- Single writer: response future updating from headers
- Single reader: `poll_ready` checking the timestamp
- No multi-field atomic updates needed
- Reset timestamp is `now + parse(reset_duration_string)`, stored as millis u64

### Header Parsing and State Update

In `call`, the returned future is wrapped to intercept the response:

```rust
fn call(&mut self, req: HttpRequestFactory) -> Self::Future {
    let bp = self.backpressure.clone();
    let inner_future = self.inner.call(req);
    Box::pin(async move {
        let response = inner_future.await?;
        update_backpressure_from_headers(response.headers(), &bp);
        Ok(response)
    })
}
```

### poll_ready Backpressure Logic

```rust
fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
    // 1. Check server backpressure
    let reset_ms = self.backpressure.reset_at.load(Ordering::Relaxed);
    if reset_ms > 0 {
        let now = now_millis();
        if now < reset_ms {
            // Register waker via tokio::time::sleep, return Pending
            return Poll::Pending;
        }
        // Reset time passed, clear
        self.backpressure.reset_at.store(0, Ordering::Relaxed);
    }

    // 2. Check governor local bucket
    match self.limiter.check() {
        Ok(_) => self.inner.poll_ready(cx),
        Err(NotUntil(_)) => Poll::Pending,
    }
}
```

Waker registration: on native targets, `poll_ready` creates a `tokio::time::sleep_until` and polls it to register the waker. On WASM, server-side backpressure is not implemented - `poll_ready` skips the reset time check and always proceeds to the governor check. This is consistent with the retry module's WASM behavior (no delays on WASM). Governor's local bucket still works on WASM via its `wasm` feature, providing basic RPM limiting without server feedback.

## Edge Cases

### Header Parsing Failures

- `x-ratelimit-remaining-requests` missing or unparseable: ignore, rely on local governor
- `x-ratelimit-reset-requests` missing or unparseable: ignore
- `remaining` present but not 0: no backpressure update, debug log only
- Both headers present and `remaining == 0`: compute `now + parse(reset)`, store in `reset_at`

### Reset Header Format

OpenAI returns duration strings like `"6s"`, `"1m30s"`. A simple parser handles `"Ns"`, `"Nm"`, `"NmNs"` formats. Invalid input returns `None` and is ignored.

### Governor Bucket Exhaustion

- `limiter.check()` returns `Err(NotUntil(deadline))`: `poll_ready` returns `Pending` until deadline
- Normal traffic: governor passes through with zero overhead

### Concurrent Clones

- Tower clones services during retry; all clones share the same governor bucket (governor internal Arc) and same `ServerBackpressure` (Arc)
- One clone's response future updating `reset_at` is immediately visible to other clones' `poll_ready`
- No race conditions: `AtomicU64` store/load is atomic

### User RPM Much Higher Than Tier Limit

- Local governor passes requests at configured RPM
- Before server 429, response headers show `remaining` decreasing to 0
- Backpressure engages, `poll_ready` stalls, subsequent requests wait until reset time
- Server feedback automatically takes over

### No Rate Limit Configured

- No `RateLimitLayer` added = no impact on existing behavior
- Zero-intrusive design

### Tracing

- Backpressure engaged (remaining hits 0): `tracing::warn!`
- Backpressure released (reset time passed): `tracing::info!`
- Governor bucket exhausted: `tracing::debug!`
- Header parse failure: `tracing::debug!` (not warn, to avoid noise)

## Feature Gate

New `rate-limit` feature in Cargo.toml:

```toml
rate-limit = ["dep:governor", "middleware"]
```

governor dependency:

```toml
governor = { version = "0.8", optional = true }

[target.wasm32-unknown-unknown.dependencies]
governor = { version = "0.8", features = ["wasm"], optional = true }
```

## Testing

### Unit Tests (no network)

- Mock service returning responses with rate limit headers, verify `ServerBackpressure` state updates correctly
- Verify `poll_ready` returns `Pending` when `remaining=0` and reset not yet reached
- Verify `poll_ready` returns `Ready` after reset time passes
- Verify governor bucket exhaustion returns `Pending`
- Reset header parser: `"6s"` -> 6s, `"1m30s"` -> 90s, invalid -> None

### Integration Tests (no real API)

- Mock service counting requests, verify request count doesn't exceed configured RPM in a time window
- Verify retry + rate limit combination: after 429, retries are also rate limited

## File Structure

```
async-openai/src/middleware/
  mod.rs                  <- update docs, add rate_limit module export
  retry/                  <- existing, no changes
  rate_limit/
    mod.rs                <- RateLimitLayer, RateLimitService, ServerBackpressure
```

New files: `async-openai/src/middleware/rate_limit/mod.rs`

Modified files:
- `async-openai/Cargo.toml` - add `rate-limit` feature and `governor` dependency
- `async-openai/src/middleware/mod.rs` - export `rate_limit` module, update docs
