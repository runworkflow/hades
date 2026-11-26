import { useEffect } from "react"
import { FingerprintIcon } from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"

import { useBiometricStatus, useUnlockWithBiometrics } from "./use-biometrics"

let hasPromptedSinceLaunch = false

export function TouchIdUnlockButton() {
  const status = useBiometricStatus()
  const unlock = useUnlockWithBiometrics()
  const { mutate: startUnlock } = unlock
  const isReady = Boolean(status.data?.isAvailable && status.data.isEnabled)

  useEffect(() => {
    if (!isReady || hasPromptedSinceLaunch) return
    hasPromptedSinceLaunch = true
    startUnlock()
  }, [isReady, startUnlock])

  if (!isReady) return null

  return (
    <div className="flex w-full flex-col gap-2">
      <button
        type="button"
        disabled={unlock.isPending}
        onClick={() => unlock.mutate()}
        className="flex h-11 w-full items-center justify-center gap-2 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover disabled:cursor-default disabled:opacity-40"
      >
        <FingerprintIcon size={20} />
        {unlock.isPending ? "Waiting for Touch ID" : "Unlock with Touch ID"}
      </button>
      <FormError message={unlock.error?.message} />
    </div>
  )
}
