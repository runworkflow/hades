import { callCommand } from "@/lib/call-command"

export type SyncSummary = {
  recordsPushed: number
  recordsPulled: number
  conflictCopies: number
  settingsPulled: boolean
  settingsConflict: boolean
  skippedFileCount: number
  rejectedRecordCount: number
  missingBlobCount: number
}

export type SyncStatus = {
  folderPath: string | null
  isSyncing: boolean
  lastSyncedAt: number | null
  lastSummary: SyncSummary | null
  errorMessage: string | null
}

export const syncApi = {
  getStatus: () => callCommand<SyncStatus>("get_sync_status"),
  chooseFolder: () => callCommand<SyncStatus | null>("choose_sync_folder"),
  turnOff: () => callCommand<SyncStatus>("turn_off_sync"),
  syncNow: () => callCommand<SyncStatus>("sync_now"),
  joinFromFolder: () => callCommand<boolean>("join_from_sync_folder"),
}

export const vaultSyncedEvent = "vault-synced"
