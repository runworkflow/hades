import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"

import { describeSyncResult } from "./describe-sync-result"
import type { SyncStatus } from "./sync-api"
import {
  useChooseSyncFolder,
  useSyncNow,
  useSyncStatus,
  useTurnOffSync,
} from "./use-sync"

const secondaryButtonClasses =
  "h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover disabled:cursor-default disabled:opacity-40"

function SetupView({ onChoose, isBusy, errorMessage }: SetupViewProps) {
  return (
    <div className="flex flex-col gap-5">
      <p className="text-ink-muted">
        Keep your vault in step between Macs through a folder in iCloud Drive,
        Dropbox or any shared folder. Everything in it stays encrypted.
      </p>
      <p className="text-sm text-ink-muted">
        Anyone who can open that folder could try to guess your master password
        offline, so use a strong one.
      </p>
      <FormError message={errorMessage} />
      <PrimaryButton onClick={onChoose} disabled={isBusy}>
        Choose folder
      </PrimaryButton>
    </div>
  )
}

type SetupViewProps = {
  onChoose: () => void
  isBusy: boolean
  errorMessage: string | undefined
}

type ActiveViewProps = {
  status: SyncStatus
  errorMessage: string | undefined
  isBusy: boolean
  onSyncNow: () => void
  onChange: () => void
  onTurnOff: () => void
}

function ActiveView({
  status,
  errorMessage,
  isBusy,
  onSyncNow,
  onChange,
  onTurnOff,
}: ActiveViewProps) {
  const lines = status.lastSummary ? describeSyncResult(status.lastSummary) : []

  return (
    <div className="flex flex-col gap-5">
      <div className="rounded-2xl bg-raised p-4">
        <p className="text-sm text-ink-muted">Sync folder</p>
        <p className="break-all">{status.folderPath}</p>
      </div>
      {lines.length > 0 && (
        <div className="flex flex-col gap-1 text-sm text-ink-muted">
          {lines.map((line) => (
            <p key={line}>{line}</p>
          ))}
        </div>
      )}
      <FormError message={errorMessage ?? status.errorMessage ?? undefined} />
      <PrimaryButton onClick={onSyncNow} disabled={isBusy || status.isSyncing}>
        {isBusy || status.isSyncing ? "Syncing" : "Sync now"}
      </PrimaryButton>
      <div className="flex gap-3">
        <button
          type="button"
          onClick={onChange}
          disabled={isBusy}
          className={secondaryButtonClasses}
        >
          Change folder
        </button>
        <button
          type="button"
          onClick={onTurnOff}
          disabled={isBusy}
          className={secondaryButtonClasses}
        >
          Turn off
        </button>
      </div>
    </div>
  )
}

type SyncDialogProps = {
  isOpen: boolean
  onClose: () => void
}

export function SyncDialog({ isOpen, onClose }: SyncDialogProps) {
  const status = useSyncStatus()
  const chooseFolder = useChooseSyncFolder()
  const turnOff = useTurnOffSync()
  const syncNow = useSyncNow()

  const isBusy =
    chooseFolder.isPending || turnOff.isPending || syncNow.isPending
  const errorMessage =
    chooseFolder.error?.message ??
    turnOff.error?.message ??
    syncNow.error?.message

  return (
    <Modal isOpen={isOpen} title="Sync" onClose={onClose}>
      <div className="flex flex-col gap-5">
        {status.data?.folderPath ? (
          <ActiveView
            status={status.data}
            errorMessage={errorMessage}
            isBusy={isBusy}
            onSyncNow={() => syncNow.mutate()}
            onChange={() => chooseFolder.mutate()}
            onTurnOff={() => turnOff.mutate()}
          />
        ) : (
          <SetupView
            onChoose={() => chooseFolder.mutate()}
            isBusy={isBusy}
            errorMessage={errorMessage}
          />
        )}
        <button
          type="button"
          onClick={onClose}
          className="h-11 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
        >
          Close
        </button>
      </div>
    </Modal>
  )
}
