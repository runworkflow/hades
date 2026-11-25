import { useEffect } from "react"

import { vaultApi } from "./vault-api"

const reportIntervalMs = 10_000
const activityEvents = ["pointerdown", "keydown", "wheel", "mousemove"] as const

export function useActivityReporter() {
  useEffect(() => {
    let lastReportedAt = 0

    function handleActivity() {
      const now = Date.now()
      if (now - lastReportedAt < reportIntervalMs) return
      lastReportedAt = now
      vaultApi.reportActivity().catch(() => undefined)
    }

    for (const eventName of activityEvents) {
      window.addEventListener(eventName, handleActivity, { passive: true })
    }
    return () => {
      for (const eventName of activityEvents) {
        window.removeEventListener(eventName, handleActivity)
      }
    }
  }, [])
}
