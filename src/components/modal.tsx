import { useEffect, useRef, type ReactNode } from "react"

type ModalProps = {
  isOpen: boolean
  title: string
  onClose: () => void
  children: ReactNode
}

export function Modal({ isOpen, title, onClose, children }: ModalProps) {
  const dialogRef = useRef<HTMLDialogElement>(null)

  useEffect(() => {
    const dialog = dialogRef.current
    if (!dialog) return
    if (isOpen && !dialog.open) dialog.showModal()
    if (!isOpen && dialog.open) dialog.close()
  }, [isOpen])

  return (
    <dialog
      ref={dialogRef}
      onClose={onClose}
      onClick={(event) => {
        if (event.target === event.currentTarget) onClose()
      }}
      className="m-auto max-h-[90vh] w-full max-w-sm overflow-y-auto rounded-2xl border border-line bg-pane text-ink shadow-2xl backdrop:bg-black/60"
    >
      {isOpen && (
        <div className="p-6">
          <h2 className="mb-5 text-xl font-semibold">{title}</h2>
          {children}
        </div>
      )}
    </dialog>
  )
}
