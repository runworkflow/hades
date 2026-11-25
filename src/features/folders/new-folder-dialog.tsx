import { Modal } from "@/components/modal"

import { FolderForm } from "./folder-form"
import type { Folder } from "./folder-types"
import { useCreateFolder } from "./use-folders"

type NewFolderDialogProps = {
  isOpen: boolean
  onClose: () => void
  onCreated: (folder: Folder) => void
}

function NewFolderContent({
  onClose,
  onCreated,
}: Omit<NewFolderDialogProps, "isOpen">) {
  const createFolder = useCreateFolder()

  return (
    <FolderForm
      initialName=""
      initialIconName="folder"
      submitLabel="Create folder"
      isPending={createFolder.isPending}
      errorMessage={createFolder.error?.message}
      onCancel={onClose}
      onSubmit={(input) =>
        createFolder.mutate(input, {
          onSuccess: (folder) => {
            onCreated(folder)
            onClose()
          },
        })
      }
    />
  )
}

export function NewFolderDialog({
  isOpen,
  onClose,
  onCreated,
}: NewFolderDialogProps) {
  return (
    <Modal isOpen={isOpen} title="New folder" onClose={onClose}>
      <NewFolderContent onClose={onClose} onCreated={onCreated} />
    </Modal>
  )
}
