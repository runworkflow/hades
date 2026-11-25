import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import { PrimaryButton } from "@/components/primary-button"

import { VaultScreenLayout } from "./vault-screen-layout"

type RecoveryKeyScreenProps = {
  recoveryKey: string
  onConfirmed: () => void
}

export function RecoveryKeyScreen({
  recoveryKey,
  onConfirmed,
}: RecoveryKeyScreenProps) {
  const [hasSavedKey, setHasSavedKey] = useState(false)

  return (
    <VaultScreenLayout
      title="Save your recovery key"
      description="This key is shown only once. If you forget your password, it is the only way back into your vault."
    >
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
        Continue
      </PrimaryButton>
    </VaultScreenLayout>
  )
}
