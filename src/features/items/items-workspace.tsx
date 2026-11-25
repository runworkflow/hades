import { useState, type ReactNode } from "react"

import type { Folder } from "@/features/folders/folder-types"

import { ItemDetailPanel } from "./item-detail-panel"
import type { VaultItem } from "./item-types"
import { ItemsView } from "./items-view"
import { NewItemDialog } from "./new-item-dialog"
import { selectItemsForView } from "./select-items"

type ItemsWorkspaceProps = {
  header: ReactNode
  title: string
  viewId: string
  folders: Folder[]
  items: VaultItem[]
}

export function ItemsWorkspace({
  header,
  title,
  viewId,
  folders,
  items,
}: ItemsWorkspaceProps) {
  const [openItemId, setOpenItemId] = useState<string | null>(null)
  const [isNewItemOpen, setIsNewItemOpen] = useState(false)

  const currentFolderId = folders.some((folder) => folder.id === viewId)
    ? viewId
    : null

  return (
    <div className="flex min-h-0 min-w-0 flex-1">
      <div className="flex min-w-0 flex-1 flex-col">
        {header}
        <ItemsView
          title={title}
          items={selectItemsForView(items, viewId)}
          folders={folders}
          openItemId={openItemId}
          onOpenItem={setOpenItemId}
          onNewItem={() => setIsNewItemOpen(true)}
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
        isOpen={isNewItemOpen}
        folders={folders}
        currentFolderId={currentFolderId}
        onClose={() => setIsNewItemOpen(false)}
        onCreated={(item) => setOpenItemId(item.id)}
      />
    </div>
  )
}
