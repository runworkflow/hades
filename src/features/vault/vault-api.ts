import { callCommand } from "@/lib/call-command"

export type VaultStatus = "not_created" | "locked" | "unlocked"

export type CreatedVault = {
  recoveryKey: string
}

export const vaultApi = {
  getStatus: () => callCommand<VaultStatus>("get_vault_status"),
  create: (password: string) =>
    callCommand<CreatedVault>("create_vault", { password }),
  unlockWithPassword: (password: string) =>
    callCommand<void>("unlock_with_password", { password }),
  unlockWithRecoveryKey: (recoveryKey: string) =>
    callCommand<void>("unlock_with_recovery_key", { recoveryKey }),
  saveRecoveryKey: (recoveryKey: string) =>
    callCommand<boolean>("save_recovery_key", { recoveryKey }),
  lock: () => callCommand<void>("lock_vault"),
  reportActivity: () => callCommand<void>("report_activity"),
}

export const vaultLockedEvent = "vault-locked"
