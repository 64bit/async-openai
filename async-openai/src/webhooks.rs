use crate::error::WebhookError;
use crate::types::webhooks::WebhookEvent;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Webhook utilities for verifying and constructing webhook events
/// https://platform.openai.com/docs/guides/webhooks
pub struct Webhooks;

impl Webhooks {
    pub fn build_event(
        body: &str,
        signature: &str,
        timestamp: &str,
        webhook_id: &str,
        secret: &str,
    ) -> Result<WebhookEvent, WebhookError> {
        // Verify the signature
        Self::verify_signature(body, signature, timestamp, webhook_id, secret)?;

        // Deserialize the event
        let event: WebhookEvent = serde_json::from_str(body)?;

        Ok(event)
    }

    pub fn verify_signature(
        body: &str,
        signature: &str,
        timestamp: &str,
        webhook_id: &str,
        secret: &str,
    ) -> Result<(), WebhookError> {
        // Construct the signed payload: webhook_id.timestamp.body
        let signed_payload = format!("{}.{}.{}", webhook_id, timestamp, body);

        // Remove "whsec_" prefix from secret if present
        let secret_key = secret.strip_prefix("whsec_").unwrap_or(secret);

        // Decode the secret from base64 (Standard Webhooks uses base64-encoded secrets)
        let secret_bytes = BASE64
            .decode(secret_key)
            .map_err(|_| WebhookError::InvalidSignature)?;

        // Compute HMAC-SHA256
        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .map_err(|_| WebhookError::InvalidSignature)?;
        mac.update(signed_payload.as_bytes());

        // Get the expected signature in base64
        let expected_signature = BASE64.encode(mac.finalize().into_bytes());

        // Parse the signature header (format: "v1,signature" or just "signature")
        // Standard Webhooks uses versioned signatures
        let signature_to_verify = if signature.contains(',') {
            // Extract signature parts (e.g., "v1,signature1 v1,signature2")
            signature
                .split_whitespace()
                .filter_map(|sig| {
                    let parts: Vec<&str> = sig.split(',').collect();
                    if parts.len() == 2 && parts[0] == "v1" {
                        Some(parts[1])
                    } else {
                        None
                    }
                })
                .collect::<Vec<&str>>()
        } else {
            vec![signature]
        };

        // Try to match any of the provided signatures
        for sig in signature_to_verify {
            if constant_time_eq(sig.as_bytes(), expected_signature.as_bytes()) {
                return Ok(());
            }
        }

        Err(WebhookError::InvalidSignature)
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (a_byte, b_byte) in a.iter().zip(b.iter()) {
        result |= a_byte ^ b_byte;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
        assert!(!constant_time_eq(b"hello", b"helloo"));
    }

    #[test]
    fn test_verify_signature_invalid() {
        let body = r#"{"test":"data"}"#;
        let signature = "invalid_signature";
        let timestamp = "1234567890";
        let webhook_id = "webhook_test";
        let secret = "test_secret";

        let result = Webhooks::verify_signature(body, signature, timestamp, webhook_id, secret);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WebhookError::InvalidSignature
        ));
    }

    #[test]
    fn test_verify_signature_valid() {
        let body = r#"{"test":"data"}"#;
        let timestamp = "1234567890";
        let webhook_id = "webhook_test";
        // Base64-encoded secret (Standard Webhooks format)
        let secret = BASE64.encode(b"test_secret");

        // Compute the expected signature
        let signed_payload = format!("{}.{}.{}", webhook_id, timestamp, body);
        let secret_bytes = BASE64.decode(&secret).unwrap();
        let mut mac = HmacSha256::new_from_slice(&secret_bytes).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = BASE64.encode(mac.finalize().into_bytes());

        let result = Webhooks::verify_signature(body, &signature, timestamp, webhook_id, &secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_with_prefix() {
        let body = r#"{"test":"data"}"#;
        let timestamp = "1234567890";
        let webhook_id = "webhook_test";
        let secret = BASE64.encode(b"test_secret");
        let prefixed_secret = format!("whsec_{}", secret);

        // Compute signature
        let signed_payload = format!("{}.{}.{}", webhook_id, timestamp, body);
        let secret_bytes = BASE64.decode(&secret).unwrap();
        let mut mac = HmacSha256::new_from_slice(&secret_bytes).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = BASE64.encode(mac.finalize().into_bytes());

        // Verify using prefixed secret
        let result =
            Webhooks::verify_signature(body, &signature, timestamp, webhook_id, &prefixed_secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_with_version() {
        let body = r#"{"test":"data"}"#;
        let timestamp = "1234567890";
        let webhook_id = "webhook_test";
        let secret = BASE64.encode(b"test_secret");

        // Compute signature
        let signed_payload = format!("{}.{}.{}", webhook_id, timestamp, body);
        let secret_bytes = BASE64.decode(&secret).unwrap();
        let mut mac = HmacSha256::new_from_slice(&secret_bytes).unwrap();
        mac.update(signed_payload.as_bytes());
        let sig_b64 = BASE64.encode(mac.finalize().into_bytes());

        // Standard Webhooks format with version prefix
        let signature = format!("v1,{}", sig_b64);

        let result = Webhooks::verify_signature(body, &signature, timestamp, webhook_id, &secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_construct_event_invalid_json() {
        let body = r#"{"invalid json"#;
        let timestamp = "1234567890";
        let webhook_id = "webhook_test";
        let secret = BASE64.encode(b"test_secret");

        // Compute valid signature for invalid JSON
        let signed_payload = format!("{}.{}.{}", webhook_id, timestamp, body);
        let secret_bytes = BASE64.decode(&secret).unwrap();
        let mut mac = HmacSha256::new_from_slice(&secret_bytes).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = BASE64.encode(mac.finalize().into_bytes());

        let result = Webhooks::construct_event(body, &signature, timestamp, webhook_id, &secret);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            WebhookError::Deserialization(_)
        ));
    }
}
