import type { QueryClient } from "@tanstack/react-query"

const vaultQueryRoot = "vault"

export function clearUnlockedData(queryClient: QueryClient) {
  queryClient.removeQueries({
    predicate: (query) => query.queryKey[0] !== vaultQueryRoot,
  })
}
