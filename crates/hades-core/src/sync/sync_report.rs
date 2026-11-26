#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SyncReport {
    pub records_pushed: usize,
    pub records_pulled: usize,
    pub conflict_copies: usize,
    pub settings_pushed: bool,
    pub settings_pulled: bool,
    pub settings_conflict: bool,
    pub blobs_pushed: usize,
    pub blobs_pulled: usize,
    pub blobs_missing: usize,
    pub skipped_files: Vec<String>,
    pub rejected_record_ids: Vec<String>,
}
