import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import { useCreateVault } from "./use-vault"
import { VaultScreenLayout } from "./vault-screen-layout"

type CreateVaultScreenProps = {
  onCreated: (recoveryKey: string) => void
}

export function CreateVaultScreen({ onCreated }: CreateVaultScreenProps) {
  const [password, setPassword] = useState("")
  const [confirmation, setConfirmation] = useState("")
  const [hasMismatch, setHasMismatch] = useState(false)
  const createVault = useCreateVault()

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    if (password !== confirmation) {
      setHasMismatch(true)
      return
    }
    setHasMismatch(false)
    createVault.mutate(password, {
      onSuccess: (created) => {
        setPassword("")
        setConfirmation("")
        onCreated(created.recoveryKey)
      },
    })
  }

  const errorMessage = hasMismatch
    ? "The passwords do not match."
    : createVault.error?.message

  return (
    <VaultScreenLayout
      title="Create your vault"
      description="Choose a master password of at least 12 characters. It cannot be reset, so make it one you will remember."
    >
      <form onSubmit={handleSubmit} className="flex w-full flex-col gap-4">
        <TextField
          label="Master password"
          type="password"
          autoFocus
          autoComplete="off"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        <TextField
          label="Confirm password"
          type="password"
          autoComplete="off"
          value={confirmation}
          onChange={(event) => setConfirmation(event.target.value)}
        />
        <FormError message={errorMessage} />
        <PrimaryButton
          type="submit"
          disabled={!password || !confirmation || createVault.isPending}
        >
          {createVault.isPending ? "Creating vault" : "Create vault"}
        </PrimaryButton>
      </form>
    </VaultScreenLayout>
  )
}
