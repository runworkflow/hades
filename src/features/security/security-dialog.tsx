import { useState } from "react"
import { KeyIcon, ShieldKeyholeIcon } from "@solar-icons/react/linear"

import { Modal } from "@/components/modal"

import { TouchIdForm } from "@/features/biometrics/touch-id-form"

import { ChangePasswordForm } from "./change-password-form"
import { NewRecoveryKeyForm } from "./new-recovery-key-form"
import { ScreenCaptureOption } from "./screen-capture-option"
import { SecurityOption } from "./security-option"
import { TouchIdOption } from "./touch-id-option"

type SecurityView = "menu" | "password" | "recovery" | "touchId"

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

  return (
    <div className="flex flex-col gap-3">
      <SecurityOption
        icon={<KeyIcon size={22} />}
        title="Change master password"
        description="Your data stays as it is."
        hasChevron
        onClick={() => setView("password")}
      />
      <SecurityOption
        icon={<ShieldKeyholeIcon size={22} />}
        title="Make a new recovery key"
        description="The old recovery key stops working."
        hasChevron
        onClick={() => setView("recovery")}
      />
      <TouchIdOption onOpen={() => setView("touchId")} />
      <ScreenCaptureOption />
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
