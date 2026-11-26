const FREE_ATTEMPTS: u32 = 4;
const FIRST_DELAY_MS: i64 = 5_000;
const SECOND_DELAY_MS: i64 = 15_000;
const LONG_DELAY_MS: i64 = 60_000;

#[derive(Debug, Default)]
pub struct UnlockThrottle {
    failures: u32,
    blocked_until_ms: i64,
}

impl UnlockThrottle {
    pub fn check(&self, now_ms: i64) -> Result<(), i64> {
        let remaining_ms = self.blocked_until_ms - now_ms;
        if remaining_ms > 0 {
            return Err(remaining_ms);
        }
        Ok(())
    }

    pub fn record_failure(&mut self, now_ms: i64) {
        self.failures = self.failures.saturating_add(1);
        let delay_ms = match self.failures.saturating_sub(FREE_ATTEMPTS) {
            0 => 0,
            1 => FIRST_DELAY_MS,
            2 => SECOND_DELAY_MS,
            _ => LONG_DELAY_MS,
        };
        self.blocked_until_ms = now_ms + delay_ms;
    }

    pub fn record_success(&mut self) {
        self.failures = 0;
        self.blocked_until_ms = 0;
    }
}
