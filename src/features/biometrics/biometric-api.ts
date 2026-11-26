import { callCommand } from "@/lib/call-command"

export type BiometricStatus = {
  isAvailable: boolean
  isEnabled: boolean
}

export const biometricApi = {
  getStatus: () => callCommand<BiometricStatus>("get_biometric_status"),
  enable: (currentPassword: string) =>
    callCommand<boolean>("enable_biometric_unlock", { currentPassword }),
  disable: () => callCommand<void>("disable_biometric_unlock"),
  unlock: () => callCommand<boolean>("unlock_with_biometrics"),
}
