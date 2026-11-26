import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import { useChangePassword } from "./use-security"

type ChangePasswordFormProps = {
  onDone: () => void
}

export function ChangePasswordForm({ onDone }: ChangePasswordFormProps) {
  const [currentPassword, setCurrentPassword] = useState("")
  const [newPassword, setNewPassword] = useState("")
  const [confirmation, setConfirmation] = useState("")
  const [hasMismatch, setHasMismatch] = useState(false)
  const changePassword = useChangePassword()

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    if (newPassword !== confirmation) {
      setHasMismatch(true)
      return
    }
    setHasMismatch(false)
    changePassword.mutate(
      { currentPassword, newPassword },
      {
        onSuccess: () => {
          setCurrentPassword("")
          setNewPassword("")
          setConfirmation("")
        },
      }
    )
  }

  if (changePassword.isSuccess) {
    return (
      <div className="flex flex-col gap-5">
        <p className="text-ink-muted">
          Your master password was changed. Use the new password the next time
          you unlock.
        </p>
        <PrimaryButton onClick={onDone}>Done</PrimaryButton>
      </div>
    )
  }

  const errorMessage = hasMismatch
    ? "The new passwords do not match."
    : changePassword.error?.message

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <TextField
        label="Current password"
        type="password"
        autoFocus
        autoComplete="off"
        value={currentPassword}
        onChange={(event) => setCurrentPassword(event.target.value)}
      />
      <TextField
        label="New password (at least 12 characters)"
        type="password"
        autoComplete="off"
        value={newPassword}
        onChange={(event) => setNewPassword(event.target.value)}
      />
      <TextField
        label="Confirm new password"
        type="password"
        autoComplete="off"
        value={confirmation}
        onChange={(event) => setConfirmation(event.target.value)}
      />
      <FormError message={errorMessage} />
      <div className="mt-2 flex gap-3">
        <button
          type="button"
          onClick={onDone}
          className="h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
        >
          Cancel
        </button>
        <div className="flex-1">
          <PrimaryButton
            type="submit"
            disabled={
              !currentPassword ||
              !newPassword ||
              !confirmation ||
              changePassword.isPending
            }
          >
            {changePassword.isPending ? "Changing" : "Change password"}
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}
