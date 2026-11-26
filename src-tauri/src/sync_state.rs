use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};

use hades_core::SyncReport;

const CHANGE_DELAY_MS: i64 = 5_000;
const PERIODIC_DELAY_MS: i64 = 300_000;
const RETRY_DELAY_MS: i64 = 30_000;
const MINIMUM_GAP_MS: i64 = 5_000;

#[derive(Clone, Default)]
pub struct SyncSummary {
    pub records_pushed: usize,
    pub records_pulled: usize,
    pub conflict_copies: usize,
    pub settings_pulled: bool,
    pub settings_conflict: bool,
    pub skipped_file_count: usize,
    pub rejected_record_count: usize,
    pub missing_blob_count: usize,
}

impl SyncSummary {
    pub fn from_report(report: &SyncReport) -> Self {
        Self {
            records_pushed: report.records_pushed,
            records_pulled: report.records_pulled,
            conflict_copies: report.conflict_copies,
            settings_pulled: report.settings_pulled,
            settings_conflict: report.settings_conflict,
            skipped_file_count: report.skipped_files.len(),
            rejected_record_count: report.rejected_record_ids.len(),
            missing_blob_count: report.blobs_missing,
        }
    }

    pub fn has_local_changes(&self) -> bool {
        self.records_pulled > 0 || self.conflict_copies > 0 || self.settings_pulled
    }
}

#[derive(Clone, Default)]
pub struct SyncSnapshot {
    pub folder: Option<PathBuf>,
    pub is_running: bool,
    pub last_success_ms: Option<i64>,
    pub last_summary: Option<SyncSummary>,
    pub last_error: Option<String>,
}

#[derive(Default)]
struct Inner {
    snapshot: SyncSnapshot,
    is_requested: bool,
    changed_at_ms: Option<i64>,
    last_attempt_ms: i64,
}

impl Inner {
    fn is_due(&self, now_ms: i64) -> bool {
        if now_ms - self.last_attempt_ms < MINIMUM_GAP_MS {
            return false;
        }
        let is_changed = self
            .changed_at_ms
            .is_some_and(|changed| now_ms - changed >= CHANGE_DELAY_MS);
        let is_stale = self
            .snapshot
            .last_success_ms
            .is_some_and(|success| now_ms - success >= PERIODIC_DELAY_MS);
        let is_retry =
            self.snapshot.last_error.is_some() && now_ms - self.last_attempt_ms >= RETRY_DELAY_MS;
        self.is_requested || is_changed || is_stale || is_retry
    }
}

#[derive(Default)]
pub struct SyncState {
    inner: Mutex<Inner>,
}

impl SyncState {
    pub fn with_folder(folder: Option<PathBuf>) -> Self {
        let mut inner = Inner::default();
        inner.snapshot.folder = folder;
        Self {
            inner: Mutex::new(inner),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn set_folder(&self, folder: Option<PathBuf>) {
        let mut inner = self.lock();
        let is_running = inner.snapshot.is_running;
        *inner = Inner::default();
        inner.snapshot.folder = folder;
        inner.snapshot.is_running = is_running;
        inner.is_requested = inner.snapshot.folder.is_some();
    }

    pub fn snapshot(&self) -> SyncSnapshot {
        self.lock().snapshot.clone()
    }

    pub fn request(&self) {
        let mut inner = self.lock();
        inner.is_requested = inner.snapshot.folder.is_some();
    }

    pub fn note_change(&self, now_ms: i64) {
        let mut inner = self.lock();
        if inner.snapshot.folder.is_some() {
            inner.changed_at_ms = Some(now_ms);
        }
    }

    pub fn begin(&self, now_ms: i64, is_forced: bool) -> Option<PathBuf> {
        let mut inner = self.lock();
        let folder = inner.snapshot.folder.clone()?;
        if inner.snapshot.is_running || (!is_forced && !inner.is_due(now_ms)) {
            return None;
        }
        inner.snapshot.is_running = true;
        inner.is_requested = false;
        inner.changed_at_ms = None;
        inner.last_attempt_ms = now_ms;
        Some(folder)
    }

    pub fn finish(&self, now_ms: i64, outcome: Result<SyncSummary, String>) {
        let mut inner = self.lock();
        inner.snapshot.is_running = false;
        match outcome {
            Ok(summary) => {
                inner.snapshot.last_success_ms = Some(now_ms);
                inner.snapshot.last_summary = Some(summary);
                inner.snapshot.last_error = None;
            }
            Err(message) => inner.snapshot.last_error = Some(message),
        }
    }
}
