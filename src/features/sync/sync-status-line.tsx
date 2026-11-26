import { formatRelativeTime } from "@/lib/format-relative-time"

import type { SyncStatus } from "./sync-api"

type SyncLook = {
  label: string
  dotClass: string
}

function lookOf(status: SyncStatus | undefined): SyncLook {
  if (!status?.folderPath) {
    return { label: "Sync off", dotClass: "bg-ink-muted" }
  }
  if (status.isSyncing) return { label: "Syncing", dotClass: "bg-accent" }
  if (status.errorMessage) {
    return { label: "Sync problem", dotClass: "bg-danger" }
  }
  if (status.lastSyncedAt === null) {
    return { label: "Waiting to sync", dotClass: "bg-ink-muted" }
  }
  return {
    label: `Synced ${formatRelativeTime(status.lastSyncedAt)}`,
    dotClass: "bg-success",
  }
}

type SyncStatusLineProps = {
  status: SyncStatus | undefined
  onOpen: () => void
}

export function SyncStatusLine({ status, onOpen }: SyncStatusLineProps) {
  const look = lookOf(status)

  return (
    <button
      type="button"
      onClick={onOpen}
      className="flex items-center gap-2 rounded-xl px-3.5 py-1 text-left text-sm text-ink-muted transition-colors hover:text-ink"
    >
      <span className={`size-2 rounded-full ${look.dotClass}`} />
      {look.label}
    </button>
  )
}
