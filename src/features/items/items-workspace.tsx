import { useState, type ReactNode } from "react"

import { DeleteFolderDialog } from "@/features/folders/delete-folder-dialog"
import { EditFolderDialog } from "@/features/folders/edit-folder-dialog"
import type { Folder } from "@/features/folders/folder-types"

import { ItemDetailPanel } from "./item-detail-panel"
import type { ItemKind, VaultItem } from "./item-types"
import { ItemsHeader } from "./items-header"
import { ItemsView } from "./items-view"
import { NewItemDialog } from "./new-item-dialog"
import { selectItemsForView } from "./select-items"

type ItemsWorkspaceProps = {
  renderHeader: (content: ReactNode) => ReactNode
  title: string
  viewId: string
  folders: Folder[]
  items: VaultItem[]
  onFolderDeleted: () => void
}

export function ItemsWorkspace({
  renderHeader,
  title,
  viewId,
  folders,
  items,
  onFolderDeleted,
}: ItemsWorkspaceProps) {
  const [openItemId, setOpenItemId] = useState<string | null>(null)
  const [searchText, setSearchText] = useState("")
  const [newItemKind, setNewItemKind] = useState<ItemKind | null>(null)
  const [isEditFolderOpen, setIsEditFolderOpen] = useState(false)
  const [isDeleteFolderOpen, setIsDeleteFolderOpen] = useState(false)

  const currentFolder = folders.find((folder) => folder.id === viewId)
  const currentFolderId = currentFolder?.id ?? null

  return (
    <div className="flex min-h-0 min-w-0 flex-1">
      <div className="flex min-w-0 flex-1 flex-col">
        {renderHeader(
          <ItemsHeader
            title={title}
            searchText={searchText}
            onSearchChange={setSearchText}
            onNewItem={setNewItemKind}
            folderActions={
              currentFolder
                ? {
                    onEdit: () => setIsEditFolderOpen(true),
                    onDelete: () => setIsDeleteFolderOpen(true),
                  }
                : null
            }
          />
        )}
        <ItemsView
          searchText={searchText}
          items={selectItemsForView(items, viewId)}
          folders={folders}
          openItemId={openItemId}
          onOpenItem={setOpenItemId}
          onItemsDeleted={(itemIds) => {
            if (openItemId && itemIds.includes(openItemId)) setOpenItemId(null)
          }}
        />
      </div>
      {openItemId && (
        <ItemDetailPanel
          key={openItemId}
          itemId={openItemId}
          folders={folders}
          onClose={() => setOpenItemId(null)}
        />
      )}
      <NewItemDialog
        kind={newItemKind}
        folders={folders}
        currentFolderId={currentFolderId}
        onClose={() => setNewItemKind(null)}
        onCreated={(item) => setOpenItemId(item.id)}
      />
      {currentFolder && (
        <>
          <EditFolderDialog
            isOpen={isEditFolderOpen}
            folder={currentFolder}
            onClose={() => setIsEditFolderOpen(false)}
          />
          <DeleteFolderDialog
            isOpen={isDeleteFolderOpen}
            folder={currentFolder}
            onClose={() => setIsDeleteFolderOpen(false)}
            onDeleted={onFolderDeleted}
          />
        </>
      )}
    </div>
  )
}
