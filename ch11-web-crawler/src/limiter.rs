use governor::{
    DefaultDirectRateLimiter, DefaultKeyedRateLimiter, Quota, RateLimiter,
};
use nonzero_ext::nonzero;

pub struct Limiters {
    pub global: DefaultDirectRateLimiter,
    pub per_host: DefaultKeyedRateLimiter<String>,
}

impl Limiters {
    pub fn new() -> Self {
        Self {
            global: RateLimiter::direct(Quota::per_second(nonzero!(50u32))),
            per_host: RateLimiter::keyed(Quota::per_second(nonzero!(1u32))),
        }
    }

    pub async fn acquire(&self, host: &str) {
        self.global.until_ready().await;
        self.per_host.until_key_ready(&host.to_string()).await;
    }
}

impl Default for Limiters {
    fn default() -> Self {
        Self::new()
    }
}
