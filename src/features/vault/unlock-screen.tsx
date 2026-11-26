import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { TouchIdUnlockButton } from "@/features/biometrics/touch-id-unlock-button"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import { useUnlockWithPassword, useUnlockWithRecoveryKey } from "./use-vault"
import { VaultScreenLayout } from "./vault-screen-layout"

type UnlockMethod = "password" | "recoveryKey"

export function UnlockScreen() {
  const [method, setMethod] = useState<UnlockMethod>("password")
  const [secret, setSecret] = useState("")
  const unlockWithPassword = useUnlockWithPassword()
  const unlockWithRecoveryKey = useUnlockWithRecoveryKey()

  const isRecoveryKey = method === "recoveryKey"
  const unlock = isRecoveryKey ? unlockWithRecoveryKey : unlockWithPassword

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    unlock.mutate(secret, { onSettled: () => setSecret("") })
  }

  function switchMethod() {
    setMethod(isRecoveryKey ? "password" : "recoveryKey")
    setSecret("")
    unlockWithPassword.reset()
    unlockWithRecoveryKey.reset()
  }

  return (
    <VaultScreenLayout
      title="Unlock Hades"
      description={
        isRecoveryKey
          ? "Enter the recovery key you saved when you created your vault."
          : "Enter your master password to open your vault."
      }
    >
      {!isRecoveryKey && <TouchIdUnlockButton />}
      <form onSubmit={handleSubmit} className="flex w-full flex-col gap-4">
        <TextField
          key={method}
          label={isRecoveryKey ? "Recovery key" : "Master password"}
          type={isRecoveryKey ? "text" : "password"}
          placeholder={isRecoveryKey ? "XXXX-XXXX-XXXX-XXXX" : undefined}
          autoFocus={isRecoveryKey}
          autoComplete="off"
          spellCheck={false}
          value={secret}
          onChange={(event) => setSecret(event.target.value)}
        />
        <FormError message={unlock.error?.message} />
        <PrimaryButton type="submit" disabled={!secret || unlock.isPending}>
          {unlock.isPending ? "Unlocking" : "Unlock"}
        </PrimaryButton>
      </form>
      <button
        type="button"
        onClick={switchMethod}
        className="text-sm text-ink-muted transition-colors hover:text-ink"
      >
        {isRecoveryKey
          ? "Use master password instead"
          : "Use recovery key instead"}
      </button>
    </VaultScreenLayout>
  )
}
