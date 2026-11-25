import { useEffect } from "react"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import type { CommandError } from "@/lib/call-command"

import { itemApi } from "./item-api"
import type {
  ItemDetail,
  NewItemInput,
  UpdateItemInput,
  VaultItem,
} from "./item-types"

const itemsQueryKey = ["items"] as const
const copyHintDurationMs = 30_000

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

export function useCopyField(itemId: string, key: string) {
  const mutation = useMutation<void, CommandError, void>({
    mutationFn: () => itemApi.copyField(itemId, key),
    gcTime: 0,
  })
  const { isSuccess, reset } = mutation

  useEffect(() => {
    if (!isSuccess) return
    const timer = setTimeout(reset, copyHintDurationMs)
    return () => clearTimeout(timer)
  }, [isSuccess, reset])

  return mutation
}

export function useUpdateItem(itemId: string) {
  const queryClient = useQueryClient()

  return useMutation<VaultItem, CommandError, UpdateItemInput>({
    mutationFn: (input) => itemApi.update(itemId, input),
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: itemsQueryKey }),
        queryClient.invalidateQueries({ queryKey: ["item", itemId] }),
      ]),
  })
}

export function useDeleteItem(itemId: string) {
  const queryClient = useQueryClient()

  return useMutation<void, CommandError, void>({
    mutationFn: () => itemApi.remove(itemId),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: ["item", itemId] })
      return queryClient.invalidateQueries({ queryKey: itemsQueryKey })
    },
  })
}

type MoveItemsInput = {
  itemIds: string[]
  folderId: string
}

export function useDeleteItems() {
  const queryClient = useQueryClient()

  return useMutation<number, CommandError, string[]>({
    mutationFn: itemApi.removeMany,
    onSuccess: (_count, itemIds) => {
      for (const itemId of itemIds) {
        queryClient.removeQueries({ queryKey: ["item", itemId] })
      }
      return queryClient.invalidateQueries({ queryKey: itemsQueryKey })
    },
  })
}

export function useMoveItems() {
  const queryClient = useQueryClient()

  return useMutation<number, CommandError, MoveItemsInput>({
    mutationFn: ({ itemIds, folderId }) => itemApi.moveMany(itemIds, folderId),
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: itemsQueryKey }),
        queryClient.invalidateQueries({ queryKey: ["item"] }),
      ]),
  })
}
