import { useEffect } from "react"

import {
  applyScreenCaptureProtection,
  readScreenCaptureProtection,
} from "./screen-capture-protection"

export function useRestoreScreenCaptureProtection() {
  useEffect(() => {
    if (readScreenCaptureProtection()) {
      applyScreenCaptureProtection(true).catch(() => undefined)
    }
  }, [])
}
