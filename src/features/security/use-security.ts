import { useMutation } from "@tanstack/react-query"

import type { CommandError } from "@/lib/call-command"

import { securityApi, type NewRecoveryKey } from "./security-api"

type ChangePasswordInput = {
  currentPassword: string
  newPassword: string
}

export function useChangePassword() {
  return useMutation<void, CommandError, ChangePasswordInput>({
    mutationFn: ({ currentPassword, newPassword }) =>
      securityApi.changePassword(currentPassword, newPassword),
    gcTime: 0,
  })
}

export function useCreateNewRecoveryKey() {
  return useMutation<NewRecoveryKey, CommandError, string>({
    mutationFn: securityApi.createNewRecoveryKey,
    gcTime: 0,
  })
}
