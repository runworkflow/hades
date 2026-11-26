import { useState } from "react"

import { DialogActions } from "@/components/dialog-actions"
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
      <DialogActions onCancel={onClose}>
        <PrimaryButton
          size="compact"
          disabled={importFile.isPending}
          onClick={handleChooseFile}
        >
          {importFile.isPending ? "Encrypting file" : "Choose file"}
        </PrimaryButton>
      </DialogActions>
    </div>
  )
}
