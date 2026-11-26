import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"

import { useKeepVaultUnlocked } from "./use-keep-vault-unlocked"
import { useSaveRecoveryKey } from "./use-vault"

type RecoveryKeyPanelProps = {
  recoveryKey: string
  confirmLabel: string
  onConfirmed: () => void
}

export function RecoveryKeyPanel({
  recoveryKey,
  confirmLabel,
  onConfirmed,
}: RecoveryKeyPanelProps) {
  const [hasSavedKey, setHasSavedKey] = useState(false)
  const saveKey = useSaveRecoveryKey()
  useKeepVaultUnlocked()

  return (
    <>
      <div className="grid w-full grid-cols-4 gap-2 rounded-xl bg-raised p-4 font-mono text-sm tracking-wide select-text">
        {recoveryKey.split("-").map((group, index) => (
          <span key={index} className="text-center">
            {group}
          </span>
        ))}
      </div>
      <div className="flex w-full flex-col gap-1">
        <button
          type="button"
          disabled={saveKey.isPending}
          onClick={() => saveKey.mutate(recoveryKey)}
          className="h-11 w-full rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover disabled:cursor-default disabled:opacity-40"
        >
          {saveKey.data === true
            ? "Saved. Save another copy"
            : "Save as a file"}
        </button>
        <FormError message={saveKey.error?.message} />
        <p className="px-1 text-xs text-ink-muted">
          The file is plain text. Keep it somewhere private, not in your synced
          folder.
        </p>
      </div>
      <div className="flex w-full items-center gap-3 text-sm">
        <Checkbox
          label="I have saved my recovery key"
          isChecked={hasSavedKey}
          onToggle={() => setHasSavedKey((isSaved) => !isSaved)}
        />
        <span>I have saved my recovery key somewhere safe</span>
      </div>
      <PrimaryButton disabled={!hasSavedKey} onClick={onConfirmed}>
        {confirmLabel}
      </PrimaryButton>
    </>
  )
}
