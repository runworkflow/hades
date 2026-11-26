import { useState, type FormEvent } from "react"

import { DialogActions } from "@/components/dialog-actions"
import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"
import { SelectField } from "@/components/select-field"
import { TextField } from "@/components/text-field"
import type { Folder } from "@/features/folders/folder-types"

import { ImportFileForm } from "./import-file-form"
import { ItemFieldsInputs } from "./item-fields-inputs"
import { ItemKindTile } from "./item-kind-tile"
import { itemKinds } from "./item-kinds"
import type { CreatableItemKind, ItemKind, VaultItem } from "./item-types"
import { useCreateItem } from "./use-items"

type NewItemFormProps = {
  kind: CreatableItemKind
  folders: Folder[]
  defaultFolderId: string
  onClose: () => void
  onCreated: (item: VaultItem) => void
}

function NewItemForm({
  kind,
  folders,
  defaultFolderId,
  onClose,
  onCreated,
}: NewItemFormProps) {
  const [name, setName] = useState("")
  const [folderId, setFolderId] = useState(defaultFolderId)
  const [fields, setFields] = useState<Record<string, string>>({})
  const createItem = useCreateItem()

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
      <div className="h-px bg-line" />
      <ItemFieldsInputs kind={kind} values={fields} onChange={setField} />
      <FormError message={createItem.error?.message} />
      <DialogActions onCancel={onClose}>
        <PrimaryButton
          size="compact"
          type="submit"
          disabled={!name.trim() || createItem.isPending}
        >
          {createItem.isPending ? "Saving" : "Save item"}
        </PrimaryButton>
      </DialogActions>
    </form>
  )
}

type NewItemContentProps = Omit<NewItemFormProps, "kind"> & {
  kind: ItemKind
}

function NewItemContent({ kind, ...props }: NewItemContentProps) {
  if (kind === "file") return <ImportFileForm {...props} />
  return <NewItemForm key={kind} kind={kind} {...props} />
}

type NewItemDialogProps = {
  kind: ItemKind | null
  folders: Folder[]
  currentFolderId: string | null
  onClose: () => void
  onCreated: (item: VaultItem) => void
}

export function NewItemDialog({
  kind,
  folders,
  currentFolderId,
  onClose,
  onCreated,
}: NewItemDialogProps) {
  const hasFolders = folders.length > 0
  const formKind = hasFolders ? kind : null

  return (
    <Modal
      isOpen={kind !== null}
      title={formKind ? itemKinds[formKind].newTitle : "New item"}
      icon={formKind ? <ItemKindTile kind={formKind} /> : undefined}
      onClose={onClose}
    >
      {hasFolders ? (
        formKind && (
          <NewItemContent
            kind={formKind}
            folders={folders}
            defaultFolderId={currentFolderId ?? folders[0].id}
            onClose={onClose}
            onCreated={onCreated}
          />
        )
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
