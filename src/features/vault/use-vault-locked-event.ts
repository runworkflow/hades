import { useEffect } from "react"
import { useQueryClient } from "@tanstack/react-query"
import { listen } from "@tauri-apps/api/event"

import { vaultStatusQueryKey } from "./use-vault"
import { vaultLockedEvent } from "./vault-api"

export function useVaultLockedEvent() {
  const queryClient = useQueryClient()

  useEffect(() => {
    const stopListening = listen(vaultLockedEvent, () => {
      queryClient.invalidateQueries({ queryKey: vaultStatusQueryKey })
    })

    return () => {
      stopListening.then((stop) => stop())
    }
  }, [queryClient])
}
