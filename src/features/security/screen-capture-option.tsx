import { useState } from "react"
import { EyeClosedIcon } from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"
import { SwitchIndicator } from "@/components/switch-indicator"

import {
  applyScreenCaptureProtection,
  readScreenCaptureProtection,
} from "./screen-capture-protection"
import { SecurityOption } from "./security-option"

const description = "Hide Hades from screenshots and screen sharing."

export function ScreenCaptureOption() {
  const [isEnabled, setIsEnabled] = useState(readScreenCaptureProtection)
  const [isBusy, setIsBusy] = useState(false)
  const [errorMessage, setErrorMessage] = useState<string>()

  function handleToggle() {
    if (isBusy) return
    const next = !isEnabled
    setIsBusy(true)
    setErrorMessage(undefined)
    applyScreenCaptureProtection(next)
      .then(
        () => setIsEnabled(next),
        () => setErrorMessage("Could not change screen capture protection.")
      )
      .finally(() => setIsBusy(false))
  }

  return (
    <>
      <SecurityOption
        role="switch"
        aria-checked={isEnabled}
        icon={<EyeClosedIcon size={22} />}
        title="Screen capture protection"
        description={description}
        trailing={<SwitchIndicator isOn={isEnabled} />}
        onClick={handleToggle}
      />
      <FormError message={errorMessage} />
    </>
  )
}
