import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"

import { useDeleteItems } from "./use-items"

type DeleteItemsDialogProps = {
  isOpen: boolean
  itemIds: string[]
  onClose: () => void
  onDeleted: () => void
}

function DeleteItemsConfirmation({
  itemIds,
  onClose,
  onDeleted,
}: Omit<DeleteItemsDialogProps, "isOpen">) {
  const deleteItems = useDeleteItems()
  const noun = itemIds.length === 1 ? "item" : "items"

  return (
    <div className="flex flex-col gap-5">
      <p className="text-ink-muted">
        {itemIds.length} {noun} will be deleted from this vault. This cannot be
        undone.
      </p>
      <FormError message={deleteItems.error?.message} />
      <div className="flex gap-3">
        <button
          type="button"
          onClick={onClose}
          className="h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
        >
          Cancel
        </button>
        <button
          type="button"
          disabled={deleteItems.isPending}
          onClick={() => deleteItems.mutate(itemIds, { onSuccess: onDeleted })}
          className="h-11 flex-1 rounded-full bg-danger font-medium text-white transition-opacity disabled:cursor-default disabled:opacity-40"
        >
          Delete {noun}
        </button>
      </div>
    </div>
  )
}

export function DeleteItemsDialog({
  isOpen,
  itemIds,
  onClose,
  onDeleted,
}: DeleteItemsDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Delete items?" onClose={onClose}>
      <DeleteItemsConfirmation
        itemIds={itemIds}
        onClose={onClose}
        onDeleted={onDeleted}
      />
    </Modal>
  )
}
