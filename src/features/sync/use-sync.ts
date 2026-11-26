import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { vaultStatusQueryKey } from "@/features/vault/use-vault"
import type { CommandError } from "@/lib/call-command"

import { syncApi, type SyncStatus } from "./sync-api"

export const syncStatusQueryKey = ["vault", "sync"] as const

const statusRefreshMilliseconds = 5000

export function useSyncStatus() {
  return useQuery<SyncStatus, CommandError>({
    queryKey: syncStatusQueryKey,
    queryFn: syncApi.getStatus,
    retry: false,
    refetchInterval: statusRefreshMilliseconds,
  })
}

function useSyncMutation<Result extends SyncStatus | null>(
  action: () => Promise<Result>
) {
  const queryClient = useQueryClient()

  return useMutation<Result, CommandError, void>({
    mutationFn: action,
    onSuccess: (status) => {
      if (status) queryClient.setQueryData(syncStatusQueryKey, status)
      return queryClient.invalidateQueries({
        predicate: (query) => query.queryKey[1] !== "status",
      })
    },
  })
}

export function useChooseSyncFolder() {
  return useSyncMutation(syncApi.chooseFolder)
}

export function useTurnOffSync() {
  return useSyncMutation(syncApi.turnOff)
}

export function useSyncNow() {
  return useSyncMutation(syncApi.syncNow)
}

export function useJoinFromSyncFolder() {
  const queryClient = useQueryClient()

  return useMutation<boolean, CommandError, void>({
    mutationFn: syncApi.joinFromFolder,
    onSuccess: (didJoin) => {
      if (didJoin) {
        return queryClient.invalidateQueries({ queryKey: vaultStatusQueryKey })
      }
    },
  })
}
