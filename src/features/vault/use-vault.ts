import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import type { CommandError } from "@/lib/call-command"

import { vaultApi } from "./vault-api"

export const vaultStatusQueryKey = ["vault", "status"] as const

export function useVaultStatus() {
  return useQuery({
    queryKey: vaultStatusQueryKey,
    queryFn: vaultApi.getStatus,
    retry: false,
    staleTime: Infinity,
    refetchOnWindowFocus: "always",
  })
}

function useVaultMutation<Input, Output>(
  action: (input: Input) => Promise<Output>
) {
  const queryClient = useQueryClient()

  return useMutation<Output, CommandError, Input>({
    mutationFn: action,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: vaultStatusQueryKey }),
  })
}

export function useCreateVault() {
  return useVaultMutation(vaultApi.create)
}

export function useUnlockWithPassword() {
  return useVaultMutation(vaultApi.unlockWithPassword)
}

export function useUnlockWithRecoveryKey() {
  return useVaultMutation(vaultApi.unlockWithRecoveryKey)
}

export function useLockVault() {
  return useVaultMutation(vaultApi.lock)
}
