import { useEffect } from "react"

import { vaultApi } from "./vault-api"

const heartbeatIntervalMs = 30_000
const maximumHeartbeats = 30

export function useKeepVaultUnlocked() {
  useEffect(() => {
    let heartbeatCount = 0
    const timer = setInterval(() => {
      heartbeatCount += 1
      if (heartbeatCount > maximumHeartbeats) {
        clearInterval(timer)
        return
      }
      vaultApi.reportActivity().catch(() => undefined)
    }, heartbeatIntervalMs)

    return () => clearInterval(timer)
  }, [])
}
