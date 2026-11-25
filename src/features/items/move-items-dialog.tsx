import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"
import { SelectField } from "@/components/select-field"
import type { Folder } from "@/features/folders/folder-types"

import { useMoveItems } from "./use-items"

type MoveItemsDialogProps = {
  isOpen: boolean
  itemIds: string[]
  folders: Folder[]
  onClose: () => void
  onMoved: () => void
}

function MoveItemsForm({
  itemIds,
  folders,
  onClose,
  onMoved,
}: Omit<MoveItemsDialogProps, "isOpen">) {
  const [folderId, setFolderId] = useState(folders[0]?.id ?? "")
  const moveItems = useMoveItems()
  const noun = itemIds.length === 1 ? "item" : "items"

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    moveItems.mutate({ itemIds, folderId }, { onSuccess: onMoved })
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <SelectField
        label={`Move ${itemIds.length} ${noun} to`}
        value={folderId}
        onChange={(event) => setFolderId(event.target.value)}
        options={folders.map((folder) => ({
          value: folder.id,
          label: folder.name,
        }))}
      />
      <FormError message={moveItems.error?.message} />
      <div className="mt-2 flex gap-3">
        <button
          type="button"
          onClick={onClose}
          className="h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
        >
          Cancel
        </button>
        <div className="flex-1">
          <PrimaryButton
            type="submit"
            disabled={!folderId || moveItems.isPending}
          >
            Move {noun}
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}

export function MoveItemsDialog({
  isOpen,
  itemIds,
  folders,
  onClose,
  onMoved,
}: MoveItemsDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Move items" onClose={onClose}>
      <MoveItemsForm
        itemIds={itemIds}
        folders={folders}
        onClose={onClose}
        onMoved={onMoved}
      />
    </Modal>
  )
}
