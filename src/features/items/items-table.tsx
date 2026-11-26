import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import type { Folder } from "@/features/folders/folder-types"

import { BulkActionsBar } from "./bulk-actions-bar"
import { DeleteItemsDialog } from "./delete-items-dialog"
import { ItemRow } from "./item-row"
import type { VaultItem } from "./item-types"
import {
  folderColumnClass,
  itemsGridClass,
  typeColumnClass,
} from "./items-grid"
import { MoveItemsDialog } from "./move-items-dialog"

type ItemsTableProps = {
  items: VaultItem[]
  folders: Folder[]
  openItemId: string | null
  onOpenItem: (itemId: string) => void
  onItemsDeleted: (itemIds: string[]) => void
}

export function ItemsTable({
  items,
  folders,
  openItemId,
  onOpenItem,
  onItemsDeleted,
}: ItemsTableProps) {
  const [selectedIds, setSelectedIds] = useState<string[]>([])
  const [isMoveOpen, setIsMoveOpen] = useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = useState(false)

  const folderNameById = new Map(
    folders.map((folder) => [folder.id, folder.name])
  )
  const selectedVisibleIds = items
    .filter((item) => selectedIds.includes(item.id))
    .map((item) => item.id)
  const areAllSelected =
    items.length > 0 && selectedVisibleIds.length === items.length

  function toggleSelected(itemId: string) {
    setSelectedIds((current) =>
      current.includes(itemId)
        ? current.filter((id) => id !== itemId)
        : [...current, itemId]
    )
  }

  function toggleAllSelected() {
    setSelectedIds(areAllSelected ? [] : items.map((item) => item.id))
  }

  function clearSelection() {
    setSelectedIds([])
  }

  if (items.length === 0) {
    return <p className="py-20 text-center text-ink-muted">Nothing here yet.</p>
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col">
      {selectedVisibleIds.length > 0 && (
        <BulkActionsBar
          count={selectedVisibleIds.length}
          onMove={() => setIsMoveOpen(true)}
          onDelete={() => setIsDeleteOpen(true)}
          onClear={clearSelection}
        />
      )}
      <div
        className={`${itemsGridClass} h-10 shrink-0 border-b border-line px-1 text-sm text-ink-muted`}
      >
        <Checkbox
          label="Select all items"
          isChecked={areAllSelected}
          onToggle={toggleAllSelected}
        />
        <span>Name</span>
        <span className={typeColumnClass}>Type</span>
        <span className={folderColumnClass}>Folder</span>
        <span>Modified</span>
      </div>
      <div className="min-h-0 flex-1 overflow-y-auto">
        {items.map((item) => (
          <ItemRow
            key={item.id}
            item={item}
            folderName={folderNameById.get(item.folderId) ?? ""}
            isSelected={selectedIds.includes(item.id)}
            isOpen={item.id === openItemId}
            onToggleSelected={() => toggleSelected(item.id)}
            onOpen={() => onOpenItem(item.id)}
          />
        ))}
      </div>
      <MoveItemsDialog
        isOpen={isMoveOpen}
        itemIds={selectedVisibleIds}
        folders={folders}
        onClose={() => setIsMoveOpen(false)}
        onMoved={() => {
          setIsMoveOpen(false)
          clearSelection()
        }}
      />
      <DeleteItemsDialog
        isOpen={isDeleteOpen}
        itemIds={selectedVisibleIds}
        onClose={() => setIsDeleteOpen(false)}
        onDeleted={() => {
          onItemsDeleted(selectedVisibleIds)
          setIsDeleteOpen(false)
          clearSelection()
        }}
      />
    </div>
  )
}
