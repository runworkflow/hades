import { callCommand } from "@/lib/call-command"

export type NewRecoveryKey = {
  recoveryKey: string
}

export const securityApi = {
  changePassword: (currentPassword: string, newPassword: string) =>
    callCommand<void>("change_password", { currentPassword, newPassword }),
  createNewRecoveryKey: (currentPassword: string) =>
    callCommand<NewRecoveryKey>("create_new_recovery_key", { currentPassword }),
}
