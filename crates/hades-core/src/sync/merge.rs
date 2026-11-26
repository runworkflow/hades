use std::cmp::Ordering;

use crate::store::{Envelope, SyncBaseline};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictWinner {
    Local,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeDecision {
    AlreadyInSync,
    PushLocal,
    TakeRemote,
    Conflict {
        winner: ConflictWinner,
        keep_loser_copy: bool,
    },
}

pub(super) fn has_same_content(first: &Envelope, second: &Envelope) -> bool {
    first.version == second.version
        && first.is_deleted == second.is_deleted
        && first.wrapped_key == second.wrapped_key
        && first.payload == second.payload
}

fn compare_strength(local: &Envelope, remote: &Envelope) -> Ordering {
    local
        .version
        .cmp(&remote.version)
        .then(local.modified_at.cmp(&remote.modified_at))
        .then_with(|| local.payload.ciphertext.cmp(&remote.payload.ciphertext))
}

fn resolve_conflict(local: &Envelope, remote: &Envelope) -> MergeDecision {
    let (winner, loser) = if compare_strength(local, remote) == Ordering::Greater {
        (ConflictWinner::Local, remote)
    } else {
        (ConflictWinner::Remote, local)
    };
    MergeDecision::Conflict {
        winner,
        keep_loser_copy: !loser.is_deleted,
    }
}

pub fn decide_merge(
    local: Option<&Envelope>,
    remote: Option<&Envelope>,
    baseline: Option<&SyncBaseline>,
) -> MergeDecision {
    let (local, remote) = match (local, remote) {
        (None, None) => return MergeDecision::AlreadyInSync,
        (Some(_), None) => return MergeDecision::PushLocal,
        (None, Some(_)) => return MergeDecision::TakeRemote,
        (Some(local), Some(remote)) => (local, remote),
    };
    if has_same_content(local, remote) {
        return MergeDecision::AlreadyInSync;
    }

    let has_local_changes = baseline.is_none_or(|known| local.payload.nonce != known.payload_nonce);
    let has_remote_changes = baseline.is_none_or(|known| {
        remote.payload.nonce != known.payload_nonce && remote.version >= known.version
    });
    match (has_local_changes, has_remote_changes) {
        (true, true) => resolve_conflict(local, remote),
        (true, false) => MergeDecision::PushLocal,
        (false, true) => MergeDecision::TakeRemote,
        (false, false) => MergeDecision::AlreadyInSync,
    }
}
