import { useState } from "react"
import {
  FingerprintIcon,
  EyeClosedIcon,
  KeyIcon,
  ShieldKeyholeIcon,
} from "@solar-icons/react/linear"

import { Modal } from "@/components/modal"

import { TouchIdForm } from "@/features/biometrics/touch-id-form"

import { ChangePasswordForm } from "./change-password-form"
import { NewRecoveryKeyForm } from "./new-recovery-key-form"
import { ScreenCaptureForm } from "./screen-capture-form"

type SecurityView =
  "menu" | "password" | "recovery" | "touchId" | "screenCapture"

const optionClasses =
  "flex items-center gap-4 rounded-2xl bg-raised p-4 text-left transition-colors hover:bg-raised-hover"

function SecurityContent({ onClose }: { onClose: () => void }) {
  const [view, setView] = useState<SecurityView>("menu")

  if (view === "password") {
    return <ChangePasswordForm onDone={() => setView("menu")} />
  }
  if (view === "recovery") {
    return <NewRecoveryKeyForm onDone={() => setView("menu")} />
  }
  if (view === "touchId") {
    return <TouchIdForm onDone={() => setView("menu")} />
  }
  if (view === "screenCapture") {
    return <ScreenCaptureForm onDone={() => setView("menu")} />
  }

  return (
    <div className="flex flex-col gap-3">
      <button
        type="button"
        onClick={() => setView("password")}
        className={optionClasses}
      >
        <KeyIcon size={22} />
        <span className="flex flex-col">
          <span className="font-medium">Change master password</span>
          <span className="text-sm text-ink-muted">
            Your data stays as it is.
          </span>
        </span>
      </button>
      <button
        type="button"
        onClick={() => setView("recovery")}
        className={optionClasses}
      >
        <ShieldKeyholeIcon size={22} />
        <span className="flex flex-col">
          <span className="font-medium">Make a new recovery key</span>
          <span className="text-sm text-ink-muted">
            The old recovery key stops working.
          </span>
        </span>
      </button>
      <button
        type="button"
        onClick={() => setView("touchId")}
        className={optionClasses}
      >
        <FingerprintIcon size={22} />
        <span className="flex flex-col">
          <span className="font-medium">Touch ID unlock</span>
          <span className="text-sm text-ink-muted">
            Open the vault with your fingerprint.
          </span>
        </span>
      </button>
      <button
        type="button"
        onClick={() => setView("screenCapture")}
        className={optionClasses}
      >
        <EyeClosedIcon size={22} />
        <span className="flex flex-col">
          <span className="font-medium">Screen capture protection</span>
          <span className="text-sm text-ink-muted">
            Hide Hades from screenshots and screen sharing.
          </span>
        </span>
      </button>
      <button
        type="button"
        onClick={onClose}
        className="mt-2 h-11 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
      >
        Close
      </button>
    </div>
  )
}

type SecurityDialogProps = {
  isOpen: boolean
  onClose: () => void
}

export function SecurityDialog({ isOpen, onClose }: SecurityDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Security" onClose={onClose}>
      <SecurityContent onClose={onClose} />
    </Modal>
  )
}
