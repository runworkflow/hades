import type { ReactNode } from "react"

type DialogActionsProps = {
  onCancel: () => void
  children: ReactNode
}

export function DialogActions({ onCancel, children }: DialogActionsProps) {
  return (
    <div className="mt-2 flex items-center justify-end gap-3">
      <button
        type="button"
        onClick={onCancel}
        className="inline-flex h-9 items-center rounded-full bg-raised px-4 font-medium transition-colors hover:bg-raised-hover"
      >
        Cancel
      </button>
      {children}
    </div>
  )
}
