use std::num::NonZeroU32;

use async_openai::middleware::rate_limit::RateLimitLayer;
use governor::Quota;

#[test]
fn rate_limit_layer_accepts_governor_quota() {
    let quota = Quota::per_second(NonZeroU32::new(1).unwrap());

    let _layer = RateLimitLayer::new(quota);
}

#[test]
fn rate_limit_layer_has_interval_convenience_constructors() {
    let _per_second = RateLimitLayer::per_second(NonZeroU32::new(1).unwrap());
    let _per_minute = RateLimitLayer::per_minute(NonZeroU32::new(60).unwrap());
    let _per_hour = RateLimitLayer::per_hour(NonZeroU32::new(3_600).unwrap());
}
