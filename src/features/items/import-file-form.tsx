import { useState } from "react"

import { FormError } from "@/components/form-error"
import { PrimaryButton } from "@/components/primary-button"
import { SelectField } from "@/components/select-field"
import type { Folder } from "@/features/folders/folder-types"

import type { VaultItem } from "./item-types"
import { useImportFile } from "./use-items"

type ImportFileFormProps = {
  folders: Folder[]
  defaultFolderId: string
  onClose: () => void
  onCreated: (item: VaultItem) => void
}

export function ImportFileForm({
  folders,
  defaultFolderId,
  onClose,
  onCreated,
}: ImportFileFormProps) {
  const [folderId, setFolderId] = useState(defaultFolderId)
  const importFile = useImportFile()

  function handleChooseFile() {
    importFile.mutate(folderId, {
      onSuccess: (item) => {
        if (!item) return
        onCreated(item)
        onClose()
      },
    })
  }

  return (
    <div className="flex flex-col gap-4">
      <SelectField
        label="Folder"
        value={folderId}
        onChange={(event) => setFolderId(event.target.value)}
        options={folders.map((folder) => ({
          value: folder.id,
          label: folder.name,
        }))}
      />
      <p className="text-sm text-ink-muted">
        Choose a file up to 100 MiB. It is encrypted into your vault and the
        original stays where it is.
      </p>
      <FormError message={importFile.error?.message} />
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
            disabled={importFile.isPending}
            onClick={handleChooseFile}
          >
            {importFile.isPending ? "Encrypting file" : "Choose file"}
          </PrimaryButton>
        </div>
      </div>
    </div>
  )
}
