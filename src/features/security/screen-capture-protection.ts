import { getCurrentWindow } from "@tauri-apps/api/window"

const storageKey = "hades.screenCaptureProtection"

export function readScreenCaptureProtection(): boolean {
  try {
    return localStorage.getItem(storageKey) === "on"
  } catch {
    return false
  }
}

export async function applyScreenCaptureProtection(isEnabled: boolean) {
  await getCurrentWindow().setContentProtected(isEnabled)
  try {
    localStorage.setItem(storageKey, isEnabled ? "on" : "off")
  } catch {
    return
  }
}
