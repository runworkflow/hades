use std::collections::BTreeMap;

use crate::folders::make_folder_conflict_copy;
use crate::items::make_item_conflict_copy;
use crate::records::{is_deleted_payload, open_record};
use crate::store::{Envelope, EnvelopeKind, Store, SyncBaseline};
use crate::vault::UnlockedVault;

use super::merge::{ConflictWinner, MergeDecision, decide_merge, has_same_content};
use super::shared_folder::SharedFolder;
use super::sync_error::SyncResult;
use super::sync_report::SyncReport;

struct RecordSync<'a, S: Store> {
    store: &'a S,
    folder: &'a SharedFolder,
    vault: Option<&'a UnlockedVault>,
    now_ms: i64,
}

fn is_trustworthy(vault: &UnlockedVault, local: Option<&Envelope>, remote: &Envelope) -> bool {
    let kind_matches = local.is_none_or(|existing| existing.kind == remote.kind);
    kind_matches
        && open_record(vault.vault_key(), remote)
            .is_ok_and(|plaintext| remote.is_deleted == is_deleted_payload(&plaintext))
}

impl<S: Store> RecordSync<'_, S> {
    fn push(&self, local: &Envelope, report: &mut SyncReport) -> SyncResult<()> {
        self.folder.write_record(local)?;
        self.store
            .put_sync_baseline(&local.id, &SyncBaseline::of(local))?;
        report.records_pushed += 1;
        Ok(())
    }

    fn pull(&self, remote: &Envelope, report: &mut SyncReport) -> SyncResult<()> {
        self.store.replace_envelope(remote)?;
        self.store
            .put_sync_baseline(&remote.id, &SyncBaseline::of(remote))?;
        report.records_pulled += 1;
        Ok(())
    }

    fn keep_copy_of(
        &self,
        vault: &UnlockedVault,
        loser: &Envelope,
        report: &mut SyncReport,
    ) -> SyncResult<()> {
        let copy = match loser.kind {
            EnvelopeKind::Item => make_item_conflict_copy(vault, loser, self.now_ms)?,
            EnvelopeKind::Folder => make_folder_conflict_copy(vault, loser, self.now_ms)?,
        };
        self.store.put_envelope(&copy)?;
        self.folder.write_record(&copy)?;
        self.store
            .put_sync_baseline(&copy.id, &SyncBaseline::of(&copy))?;
        report.conflict_copies += 1;
        Ok(())
    }

    fn settle(
        &self,
        local: Option<Envelope>,
        remote: Option<Envelope>,
        report: &mut SyncReport,
    ) -> SyncResult<()> {
        let Some(id) = local.as_ref().or(remote.as_ref()).map(|e| e.id.clone()) else {
            return Ok(());
        };
        let remote = match (self.vault, remote) {
            (Some(vault), Some(candidate))
                if !is_trustworthy(vault, local.as_ref(), &candidate) =>
            {
                report.rejected_record_ids.push(id.clone());
                None
            }
            (_, candidate) => candidate,
        };
        let baseline = self.store.get_sync_baseline(&id)?;

        match (
            decide_merge(local.as_ref(), remote.as_ref(), baseline.as_ref()),
            local,
            remote,
        ) {
            (MergeDecision::AlreadyInSync, Some(local), Some(remote))
                if has_same_content(&local, &remote) =>
            {
                self.store
                    .put_sync_baseline(&id, &SyncBaseline::of(&local))?;
            }
            (MergeDecision::PushLocal, Some(local), _) => self.push(&local, report)?,
            (MergeDecision::TakeRemote, _, Some(remote)) if self.vault.is_some() => {
                self.pull(&remote, report)?;
            }
            (
                MergeDecision::Conflict {
                    winner,
                    keep_loser_copy,
                },
                Some(local),
                Some(remote),
            ) => {
                let Some(vault) = self.vault else {
                    return Ok(());
                };
                let (winning, losing) = match winner {
                    ConflictWinner::Local => (&local, &remote),
                    ConflictWinner::Remote => (&remote, &local),
                };
                if keep_loser_copy {
                    self.keep_copy_of(vault, losing, report)?;
                }
                match winner {
                    ConflictWinner::Local => self.push(winning, report)?,
                    ConflictWinner::Remote => self.pull(winning, report)?,
                }
            }
            _ => {}
        }
        Ok(())
    }
}

pub(super) fn sync_records<S: Store>(
    store: &S,
    folder: &SharedFolder,
    vault: Option<&UnlockedVault>,
    now_ms: i64,
    report: &mut SyncReport,
) -> SyncResult<()> {
    let scan = folder.read_records()?;
    report.skipped_files = scan.skipped_files;
    let mut remote_by_id: BTreeMap<String, Envelope> = scan
        .envelopes
        .into_iter()
        .map(|envelope| (envelope.id.clone(), envelope))
        .collect();

    let engine = RecordSync {
        store,
        folder,
        vault,
        now_ms,
    };
    for local in store.list_all_envelopes()? {
        let remote = remote_by_id.remove(&local.id);
        engine.settle(Some(local), remote, report)?;
    }
    for remote in remote_by_id.into_values() {
        engine.settle(None, Some(remote), report)?;
    }
    Ok(())
}
