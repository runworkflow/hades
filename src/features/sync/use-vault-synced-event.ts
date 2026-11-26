import { useEffect } from "react"
import { useQueryClient } from "@tanstack/react-query"
import { listen } from "@tauri-apps/api/event"

import { vaultSyncedEvent } from "./sync-api"

const vaultQueryRoot = "vault"

export function useVaultSyncedEvent() {
  const queryClient = useQueryClient()

  useEffect(() => {
    const stopListening = listen(vaultSyncedEvent, () => {
      queryClient.invalidateQueries({
        predicate: (query) => query.queryKey[0] !== vaultQueryRoot,
      })
      queryClient.invalidateQueries({ queryKey: ["vault", "biometrics"] })
    })

    return () => {
      stopListening.then((stop) => stop())
    }
  }, [queryClient])
}
