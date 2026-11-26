import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import {
  useBiometricStatus,
  useDisableBiometrics,
  useEnableBiometrics,
} from "./use-biometrics"

type TouchIdFormProps = {
  onDone: () => void
}

const secondaryButtonClasses =
  "h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"

export function TouchIdForm({ onDone }: TouchIdFormProps) {
  const status = useBiometricStatus()
  const enable = useEnableBiometrics()
  const disable = useDisableBiometrics()
  const [currentPassword, setCurrentPassword] = useState("")

  function handleEnable(event: FormEvent) {
    event.preventDefault()
    enable.mutate(currentPassword, {
      onSuccess: (isEnabled) => {
        setCurrentPassword("")
        if (isEnabled) onDone()
      },
    })
  }

  if (!status.data) return null

  if (!status.data.isAvailable) {
    return (
      <div className="flex flex-col gap-5">
        <p className="text-ink-muted">
          Touch ID is not available on this Mac, so it cannot be used to unlock
          Hades.
        </p>
        <PrimaryButton onClick={onDone}>Back</PrimaryButton>
      </div>
    )
  }

  if (status.data.isEnabled) {
    return (
      <div className="flex flex-col gap-5">
        <p className="text-ink-muted">
          Touch ID unlock is on. Your password and recovery key keep working. If
          you add or remove a fingerprint on this Mac, Touch ID unlock stops
          until you turn it on again.
        </p>
        <FormError message={disable.error?.message} />
        <div className="flex gap-3">
          <button
            type="button"
            onClick={onDone}
            className={secondaryButtonClasses}
          >
            Back
          </button>
          <button
            type="button"
            disabled={disable.isPending}
            onClick={() => disable.mutate(undefined, { onSuccess: onDone })}
            className="h-11 flex-1 rounded-full bg-danger font-medium text-white transition-opacity disabled:cursor-default disabled:opacity-40"
          >
            Turn off
          </button>
        </div>
      </div>
    )
  }

  return (
    <form onSubmit={handleEnable} className="flex flex-col gap-4">
      <p className="text-ink-muted">
        After you confirm your password, macOS asks for your fingerprint once.
        Your password and recovery key keep working.
      </p>
      <TextField
        label="Current password"
        type="password"
        autoFocus
        autoComplete="off"
        value={currentPassword}
        onChange={(event) => setCurrentPassword(event.target.value)}
      />
      <FormError message={enable.error?.message} />
      <div className="mt-2 flex gap-3">
        <button
          type="button"
          onClick={onDone}
          className={secondaryButtonClasses}
        >
          Cancel
        </button>
        <div className="flex-1">
          <PrimaryButton
            type="submit"
            disabled={!currentPassword || enable.isPending}
          >
            {enable.isPending ? "Waiting for Touch ID" : "Turn on Touch ID"}
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}
