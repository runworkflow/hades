import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import type { CommandError } from "@/lib/call-command"

import { itemApi } from "./item-api"
import type { ItemDetail, NewItemInput, VaultItem } from "./item-types"

const itemsQueryKey = ["items"] as const

export function useItems() {
  return useQuery<VaultItem[], CommandError>({
    queryKey: itemsQueryKey,
    queryFn: itemApi.list,
    retry: false,
  })
}

export function useItemDetail(itemId: string) {
  return useQuery<ItemDetail, CommandError>({
    queryKey: ["item", itemId],
    queryFn: () => itemApi.get(itemId),
    retry: false,
  })
}

export function useCreateItem() {
  const queryClient = useQueryClient()

  return useMutation<VaultItem, CommandError, NewItemInput>({
    mutationFn: itemApi.create,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: itemsQueryKey }),
  })
}

export function useRevealField(itemId: string, key: string) {
  return useMutation<string, CommandError, void>({
    mutationFn: () => itemApi.revealField(itemId, key),
    gcTime: 0,
  })
}
