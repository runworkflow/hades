import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"
import { SelectField } from "@/components/select-field"
import { TextField } from "@/components/text-field"
import type { Folder } from "@/features/folders/folder-types"

import { ItemFieldsInputs } from "./item-fields-inputs"
import type { CreatableItemKind, ItemDetail } from "./item-types"
import { useUpdateItem } from "./use-items"

type EditItemFormProps = {
  detail: ItemDetail
  kind: CreatableItemKind
  folders: Folder[]
  onClose: () => void
}

function EditItemForm({ detail, kind, folders, onClose }: EditItemFormProps) {
  const [name, setName] = useState(detail.name)
  const [folderId, setFolderId] = useState(detail.folderId)
  const [fields, setFields] = useState<Record<string, string>>(() =>
    Object.fromEntries(
      detail.fields
        .filter((field) => !field.isSecret)
        .map((field) => [field.key, field.value ?? ""])
    )
  )
  const updateItem = useUpdateItem(detail.id)

  const secretKeys = detail.fields
    .filter((field) => field.isSecret)
    .map((field) => field.key)
  const keptSecretKeys = detail.fields
    .filter((field) => field.isSecret && field.hasValue)
    .map((field) => field.key)

  function setField(key: string, value: string) {
    setFields((current) => ({ ...current, [key]: value }))
  }

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    const changedFields = Object.fromEntries(
      Object.entries(fields).filter(
        ([key, value]) => !secretKeys.includes(key) || value !== ""
      )
    )
    updateItem.mutate(
      { folderId, name, fields: changedFields },
      { onSuccess: onClose }
    )
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <TextField
        label="Name"
        autoFocus
        autoComplete="off"
        maxLength={80}
        value={name}
        onChange={(event) => setName(event.target.value)}
      />
      <SelectField
        label="Folder"
        value={folderId}
        onChange={(event) => setFolderId(event.target.value)}
        options={folders.map((folder) => ({
          value: folder.id,
          label: folder.name,
        }))}
      />
      <ItemFieldsInputs
        kind={kind}
        values={fields}
        onChange={setField}
        keptSecretKeys={keptSecretKeys}
      />
      <FormError message={updateItem.error?.message} />
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
            disabled={!name.trim() || updateItem.isPending}
          >
            Save changes
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}

type EditItemDialogProps = {
  isOpen: boolean
  detail: ItemDetail
  kind: CreatableItemKind
  folders: Folder[]
  onClose: () => void
}

export function EditItemDialog({
  isOpen,
  detail,
  kind,
  folders,
  onClose,
}: EditItemDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Edit item" onClose={onClose}>
      <EditItemForm
        detail={detail}
        kind={kind}
        folders={folders}
        onClose={onClose}
      />
    </Modal>
  )
}
