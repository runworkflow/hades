import { useState, type FormEvent } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { TextField } from "@/components/text-field"

import { FolderIconPicker } from "./folder-icon-picker"
import type { FolderIconName, NewFolderInput } from "./folder-types"

type FolderFormProps = {
  initialName: string
  initialIconName: FolderIconName
  submitLabel: string
  isPending: boolean
  errorMessage: string | undefined
  onSubmit: (input: NewFolderInput) => void
  onCancel: () => void
}

export function FolderForm({
  initialName,
  initialIconName,
  submitLabel,
  isPending,
  errorMessage,
  onSubmit,
  onCancel,
}: FolderFormProps) {
  const [name, setName] = useState(initialName)
  const [iconName, setIconName] = useState<FolderIconName>(initialIconName)

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    onSubmit({ name, iconName })
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
      <FormError message={errorMessage} />
      <div className="mt-2 flex gap-3">
        <button
          type="button"
          onClick={onCancel}
          className="h-11 flex-1 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
        >
          Cancel
        </button>
        <div className="flex-1">
          <PrimaryButton type="submit" disabled={!name.trim() || isPending}>
            {submitLabel}
          </PrimaryButton>
        </div>
      </div>
    </form>
  )
}
