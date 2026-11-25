import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import type { CommandError } from "@/lib/call-command"

import { folderApi } from "./folder-api"
import type { Folder, NewFolderInput } from "./folder-types"

const foldersQueryKey = ["folders"] as const

export function useFolders() {
  return useQuery({
    queryKey: foldersQueryKey,
    queryFn: folderApi.list,
    retry: false,
  })
}

export function useCreateFolder() {
  const queryClient = useQueryClient()

  return useMutation<Folder, CommandError, NewFolderInput>({
    mutationFn: folderApi.create,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: foldersQueryKey }),
  })
}
