const FIVE_MINUTES_MS: i64 = 5 * 60 * 1000;
const THIRTY_SECONDS_MS: i64 = 30 * 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockReason {
    Idle,
    Sleep,
    ScreenLock,
    UserSwitch,
}

impl LockReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Sleep => "sleep",
            Self::ScreenLock => "screen_lock",
            Self::UserSwitch => "user_switch",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutoLockPolicy {
    pub idle_timeout_ms: i64,
    pub max_check_gap_ms: i64,
}

impl Default for AutoLockPolicy {
    fn default() -> Self {
        Self {
            idle_timeout_ms: FIVE_MINUTES_MS,
            max_check_gap_ms: THIRTY_SECONDS_MS,
        }
    }
}

#[derive(Debug)]
pub struct AutoLockMonitor {
    policy: AutoLockPolicy,
    last_activity_ms: i64,
    last_check_ms: i64,
}

impl AutoLockMonitor {
    pub fn new(policy: AutoLockPolicy, now_ms: i64) -> Self {
        Self {
            policy,
            last_activity_ms: now_ms,
            last_check_ms: now_ms,
        }
    }

    pub fn record_activity(&mut self, now_ms: i64) {
        self.last_activity_ms = now_ms;
    }

    pub fn check(&mut self, now_ms: i64) -> Option<LockReason> {
        let gap_since_last_check = now_ms - self.last_check_ms;
        self.last_check_ms = now_ms;

        if gap_since_last_check > self.policy.max_check_gap_ms {
            return Some(LockReason::Sleep);
        }
        if now_ms - self.last_activity_ms >= self.policy.idle_timeout_ms {
            return Some(LockReason::Idle);
        }
        None
    }
}
