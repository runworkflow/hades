import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { Modal } from "@/components/modal"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import { FolderIconPicker } from "./folder-icon-picker"
import type { Folder, FolderIconName } from "./folder-types"
import { useCreateFolder } from "./use-folders"

type NewFolderFormProps = {
  onClose: () => void
  onCreated: (folder: Folder) => void
}

function NewFolderForm({ onClose, onCreated }: NewFolderFormProps) {
  const [name, setName] = useState("")
  const [iconName, setIconName] = useState<FolderIconName>("folder")
  const createFolder = useCreateFolder()

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    createFolder.mutate(
      { name, iconName },
      {
        onSuccess: (folder) => {
          onCreated(folder)
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
        maxLength={40}
        placeholder="Work, Personal, Clients"
        value={name}
        onChange={(event) => setName(event.target.value)}
      />
      <FolderIconPicker selected={iconName} onSelect={setIconName} />
      <FormError message={createFolder.error?.message} />
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
            disabled={!name.trim() || createFolder.isPending}
          >
            Create folder
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}

type NewFolderDialogProps = {
  isOpen: boolean
  onClose: () => void
  onCreated: (folder: Folder) => void
}

export function NewFolderDialog({
  isOpen,
  onClose,
  onCreated,
}: NewFolderDialogProps) {
  return (
    <Modal isOpen={isOpen} title="New folder" onClose={onClose}>
      <NewFolderForm onClose={onClose} onCreated={onCreated} />
    </Modal>
  )
}
