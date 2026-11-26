import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"
import { RecoveryKeyPanel } from "@/features/vault/recovery-key-panel"

import { useCreateNewRecoveryKey } from "./use-security"

type NewRecoveryKeyFormProps = {
  onDone: () => void
}

export function NewRecoveryKeyForm({ onDone }: NewRecoveryKeyFormProps) {
  const [currentPassword, setCurrentPassword] = useState("")
  const createKey = useCreateNewRecoveryKey()

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    createKey.mutate(currentPassword, {
      onSuccess: () => setCurrentPassword(""),
    })
  }

  if (createKey.data) {
    return (
      <div className="flex flex-col gap-4">
        <p className="text-ink-muted">
          This is your new recovery key. It is shown only once, and your old key
          no longer works.
        </p>
        <RecoveryKeyPanel
          recoveryKey={createKey.data.recoveryKey}
          confirmLabel="Done"
          onConfirmed={onDone}
        />
      </div>
    )
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <p className="text-ink-muted">
        Your current recovery key will stop working immediately. Have a safe
        place ready for the new one.
      </p>
      <TextField
        label="Current password"
        type="password"
        autoFocus
        autoComplete="off"
        value={currentPassword}
        onChange={(event) => setCurrentPassword(event.target.value)}
      />
      <FormError message={createKey.error?.message} />
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
            disabled={!currentPassword || createKey.isPending}
          >
            {createKey.isPending ? "Working" : "Make new key"}
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}
