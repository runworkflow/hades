import { RecoveryKeyPanel } from "./recovery-key-panel"
import { VaultScreenLayout } from "./vault-screen-layout"

type RecoveryKeyScreenProps = {
  recoveryKey: string
  onConfirmed: () => void
}

export function RecoveryKeyScreen({
  recoveryKey,
  onConfirmed,
}: RecoveryKeyScreenProps) {
  return (
    <VaultScreenLayout
      title="Save your recovery key"
      description="This key is shown only once. If you forget your password, it is the only way back into your vault."
    >
      <RecoveryKeyPanel
        recoveryKey={recoveryKey}
        confirmLabel="Continue"
        onConfirmed={onConfirmed}
      />
    </VaultScreenLayout>
  )
}
