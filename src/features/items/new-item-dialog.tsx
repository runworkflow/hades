import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"
import { SelectField } from "@/components/select-field"
import { TextAreaField } from "@/components/text-area-field"
import { TextField } from "@/components/text-field"
import type { Folder } from "@/features/folders/folder-types"

import { itemFieldSpecs } from "./item-field-specs"
import { ItemKindPicker } from "./item-kind-picker"
import type { CreatableItemKind, VaultItem } from "./item-types"
import { useCreateItem } from "./use-items"

type NewItemFormProps = {
  folders: Folder[]
  defaultFolderId: string
  onClose: () => void
  onCreated: (item: VaultItem) => void
}

function NewItemForm({
  folders,
  defaultFolderId,
  onClose,
  onCreated,
}: NewItemFormProps) {
  const [kind, setKind] = useState<CreatableItemKind>("password")
  const [name, setName] = useState("")
  const [folderId, setFolderId] = useState(defaultFolderId)
  const [fields, setFields] = useState<Record<string, string>>({})
  const createItem = useCreateItem()

  function changeKind(nextKind: CreatableItemKind) {
    setKind(nextKind)
    setFields({})
  }

  function setField(key: string, value: string) {
    setFields((current) => ({ ...current, [key]: value }))
  }

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    createItem.mutate(
      { folderId, kind, name, fields },
      {
        onSuccess: (item) => {
          onCreated(item)
          onClose()
        },
      }
    )
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <ItemKindPicker selected={kind} onSelect={changeKind} />
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
      {itemFieldSpecs[kind].map((spec) => {
        const fieldKey = `${kind}-${spec.key}`
        const value = fields[spec.key] ?? ""
        return spec.isMultiline ? (
          <TextAreaField
            key={fieldKey}
            label={spec.label}
            value={value}
            onChange={(event) => setField(spec.key, event.target.value)}
          />
        ) : (
          <TextField
            key={fieldKey}
            label={spec.label}
            type={spec.isSecret ? "password" : "text"}
            autoComplete="off"
            value={value}
            onChange={(event) => setField(spec.key, event.target.value)}
          />
        )
      })}
      <FormError message={createItem.error?.message} />
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
            disabled={!name.trim() || createItem.isPending}
          >
            Save item
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}

type NewItemDialogProps = {
  isOpen: boolean
  folders: Folder[]
  currentFolderId: string | null
  onClose: () => void
  onCreated: (item: VaultItem) => void
}

export function NewItemDialog({
  isOpen,
  folders,
  currentFolderId,
  onClose,
  onCreated,
}: NewItemDialogProps) {
  const hasFolders = folders.length > 0

  return (
    <Modal isOpen={isOpen} title="New item" onClose={onClose}>
      {hasFolders ? (
        <NewItemForm
          folders={folders}
          defaultFolderId={currentFolderId ?? folders[0].id}
          onClose={onClose}
          onCreated={onCreated}
        />
      ) : (
        <div className="flex flex-col gap-5">
          <p className="text-ink-muted">
            Items live in folders. Create a folder first, then add your item to
            it.
          </p>
          <PrimaryButton onClick={onClose}>Got it</PrimaryButton>
        </div>
      )}
    </Modal>
  )
}
