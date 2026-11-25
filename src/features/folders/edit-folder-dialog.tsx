import { Modal } from "@/components/modal"

import { toFolderIconName } from "./folder-icons"
import { FolderForm } from "./folder-form"
import type { Folder } from "./folder-types"
import { useUpdateFolder } from "./use-folders"

type EditFolderDialogProps = {
  isOpen: boolean
  folder: Folder
  onClose: () => void
}

function EditFolderContent({
  folder,
  onClose,
}: Omit<EditFolderDialogProps, "isOpen">) {
  const updateFolder = useUpdateFolder(folder.id)

  return (
    <FolderForm
      initialName={folder.name}
      initialIconName={toFolderIconName(folder.iconName)}
      submitLabel="Save changes"
      isPending={updateFolder.isPending}
      errorMessage={updateFolder.error?.message}
      onCancel={onClose}
      onSubmit={(input) => updateFolder.mutate(input, { onSuccess: onClose })}
    />
  )
}

export function EditFolderDialog({
  isOpen,
  folder,
  onClose,
}: EditFolderDialogProps) {
  return (
    <Modal isOpen={isOpen} title="Edit folder" onClose={onClose}>
      <EditFolderContent folder={folder} onClose={onClose} />
    </Modal>
  )
}
