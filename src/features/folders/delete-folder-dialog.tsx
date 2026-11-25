import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"

import type { Folder } from "./folder-types"
import { useDeleteFolder } from "./use-folders"

type DeleteFolderDialogProps = {
  isOpen: boolean
  folder: Folder
  onClose: () => void
  onDeleted: () => void
}

function DeleteFolderConfirmation({
  folder,
  onClose,
  onDeleted,
}: Omit<DeleteFolderDialogProps, "isOpen">) {
  const deleteFolder = useDeleteFolder(folder.id)

  return (
    <div className="flex flex-col gap-5">
      <p className="text-ink-muted">
        &ldquo;{folder.name}&rdquo; will be deleted. A folder can only be
        deleted when it has no items.
      </p>
      <FormError message={deleteFolder.error?.message} />
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
          disabled={deleteFolder.isPending}
          onClick={() =>
            deleteFolder.mutate(undefined, { onSuccess: onDeleted })
          }
          className="h-11 flex-1 rounded-full bg-danger font-medium text-white transition-opacity disabled:cursor-default disabled:opacity-40"
        >
          Delete folder
        </button>
      </div>
    </div>
  )
}

export function DeleteFolderDialog({
  isOpen,
  folder,
  onClose,
  onDeleted,
}: DeleteFolderDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Delete folder?" onClose={onClose}>
      <DeleteFolderConfirmation
        folder={folder}
        onClose={onClose}
        onDeleted={onDeleted}
      />
    </Modal>
  )
}
