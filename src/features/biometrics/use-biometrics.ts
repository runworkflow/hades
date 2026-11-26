import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { vaultStatusQueryKey } from "@/features/vault/use-vault"
import type { CommandError } from "@/lib/call-command"

import { biometricApi, type BiometricStatus } from "./biometric-api"

const biometricStatusQueryKey = ["vault", "biometrics"] as const

export function useBiometricStatus() {
  return useQuery<BiometricStatus, CommandError>({
    queryKey: biometricStatusQueryKey,
    queryFn: biometricApi.getStatus,
    retry: false,
  })
}

export function useEnableBiometrics() {
  const queryClient = useQueryClient()

  return useMutation<boolean, CommandError, string>({
    mutationFn: biometricApi.enable,
    gcTime: 0,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: biometricStatusQueryKey }),
  })
}

export function useDisableBiometrics() {
  const queryClient = useQueryClient()

  return useMutation<void, CommandError, void>({
    mutationFn: biometricApi.disable,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: biometricStatusQueryKey }),
  })
}

export function useUnlockWithBiometrics() {
  const queryClient = useQueryClient()

  return useMutation<boolean, CommandError, void>({
    mutationFn: biometricApi.unlock,
    gcTime: 0,
    onSuccess: (isUnlocked) => {
      if (isUnlocked) {
        return queryClient.invalidateQueries({ queryKey: vaultStatusQueryKey })
      }
    },
  })
}
