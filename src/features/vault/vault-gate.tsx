import { useState } from "react"

import { PrimaryButton } from "@/components/primary-button"
import { AppShell } from "@/features/shell/app-shell"

import { CreateVaultScreen } from "./create-vault-screen"
import { RecoveryKeyScreen } from "./recovery-key-screen"
import { UnlockScreen } from "./unlock-screen"
import { useVaultStatus } from "./use-vault"
import { VaultScreenLayout } from "./vault-screen-layout"

export function VaultGate() {
  const vaultStatus = useVaultStatus()
  const [recoveryKeyToSave, setRecoveryKeyToSave] = useState<string | null>(
    null
  )

  if (recoveryKeyToSave) {
    return (
      <RecoveryKeyScreen
        recoveryKey={recoveryKeyToSave}
        onConfirmed={() => setRecoveryKeyToSave(null)}
      />
    )
  }

  if (vaultStatus.isPending) {
    return <div className="h-screen bg-sidebar" />
  }

  if (vaultStatus.isError) {
    return (
      <VaultScreenLayout
        title="Could not open the vault"
        description="Something went wrong while starting Hades."
      >
        <PrimaryButton onClick={() => vaultStatus.refetch()}>
          Try again
        </PrimaryButton>
      </VaultScreenLayout>
    )
  }

  if (vaultStatus.data === "not_created") {
    return <CreateVaultScreen onCreated={setRecoveryKeyToSave} />
  }

  if (vaultStatus.data === "locked") {
    return <UnlockScreen />
  }

  return <AppShell />
}
