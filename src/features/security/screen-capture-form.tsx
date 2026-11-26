import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"

import {
  applyScreenCaptureProtection,
  readScreenCaptureProtection,
} from "./screen-capture-protection"

type ScreenCaptureFormProps = {
  onDone: () => void
}

export function ScreenCaptureForm({ onDone }: ScreenCaptureFormProps) {
  const [isEnabled, setIsEnabled] = useState(readScreenCaptureProtection)
  const [errorMessage, setErrorMessage] = useState<string>()

  function handleToggle() {
    const next = !isEnabled
    setErrorMessage(undefined)
    applyScreenCaptureProtection(next).then(
      () => setIsEnabled(next),
      () => setErrorMessage("Could not change screen capture protection.")
    )
  }

  return (
    <div className="flex flex-col gap-5">
      <p className="text-ink-muted">
        When on, screenshots, screen recordings and screen sharing show the
        Hades window as blank. It only hides the window, it does not stop
        someone looking at your screen.
      </p>
      <div className="flex items-center gap-3">
        <Checkbox
          label="Hide Hades from screen capture"
          isChecked={isEnabled}
          onToggle={handleToggle}
        />
        <span>Hide Hades from screen capture</span>
      </div>
      <FormError message={errorMessage} />
      <PrimaryButton onClick={onDone}>Back</PrimaryButton>
    </div>
  )
}
