import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"

import { useDeleteItem } from "./use-items"

type DeleteItemConfirmationProps = {
  itemId: string
  itemName: string
  onClose: () => void
  onDeleted: () => void
}

function DeleteItemConfirmation({
  itemId,
  itemName,
  onClose,
  onDeleted,
}: DeleteItemConfirmationProps) {
  const deleteItem = useDeleteItem(itemId)

  return (
    <div className="flex flex-col gap-5">
      <p className="text-ink-muted">
        &ldquo;{itemName}&rdquo; will be deleted from this vault. This cannot be
        undone.
      </p>
      <FormError message={deleteItem.error?.message} />
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
          disabled={deleteItem.isPending}
          onClick={() => deleteItem.mutate(undefined, { onSuccess: onDeleted })}
          className="h-11 flex-1 rounded-full bg-danger font-medium text-white transition-opacity disabled:cursor-default disabled:opacity-40"
        >
          Delete item
        </button>
      </div>
    </div>
  )
}

type DeleteItemDialogProps = {
  isOpen: boolean
  itemId: string
  itemName: string
  onClose: () => void
  onDeleted: () => void
}

export function DeleteItemDialog({
  isOpen,
  itemId,
  itemName,
  onClose,
  onDeleted,
}: DeleteItemDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Delete item?" onClose={onClose}>
      <DeleteItemConfirmation
        itemId={itemId}
        itemName={itemName}
        onClose={onClose}
        onDeleted={onDeleted}
      />
    </Modal>
  )
}
