use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use acvc_domain::Clock;

#[derive(Debug, Default)]
pub struct SystemClock {
    nonce: AtomicU64,
}

impl Clock for SystemClock {
    fn now_millis(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }

    fn nonce(&self) -> u64 {
        let tick = self.nonce.fetch_add(1, Ordering::Relaxed);
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0);
        time.saturating_mul(1_000).saturating_add(tick % 1_000)
    }
}
