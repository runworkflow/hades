import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import { PrimaryButton } from "@/components/primary-button"

import { useKeepVaultUnlocked } from "./use-keep-vault-unlocked"

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
