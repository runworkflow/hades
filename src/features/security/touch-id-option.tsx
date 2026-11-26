import { FingerprintIcon } from "@solar-icons/react/linear"

import { useBiometricStatus } from "@/features/biometrics/use-biometrics"

import { SecurityOption } from "./security-option"

type TouchIdOptionProps = {
  onOpen: () => void
}

export function TouchIdOption({ onOpen }: TouchIdOptionProps) {
  const status = useBiometricStatus()

  return (
    <SecurityOption
      icon={<FingerprintIcon size={22} />}
      title="Touch ID unlock"
      description="Open the vault with your fingerprint."
      trailing={<TouchIdState status={status.data} />}
      hasChevron
      onClick={onOpen}
    />
  )
}

type TouchIdStateProps = {
  status: { isAvailable: boolean; isEnabled: boolean } | undefined
}

function TouchIdState({ status }: TouchIdStateProps) {
  if (!status) return null
  if (!status.isAvailable) {
    return <span className="text-sm text-ink-muted">Not available</span>
  }
  if (status.isEnabled) {
    return <span className="text-sm font-medium text-success">On</span>
  }
  return <span className="text-sm text-ink-muted">Off</span>
}
