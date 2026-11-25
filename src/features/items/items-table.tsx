import { useState } from "react"

import { Checkbox } from "@/components/checkbox"
import type { Folder } from "@/features/folders/folder-types"

import { ItemRow } from "./item-row"
import type { VaultItem } from "./item-types"
import { itemsGridClass } from "./items-grid"

type ItemsTableProps = {
  items: VaultItem[]
  folders: Folder[]
}

export function ItemsTable({ items, folders }: ItemsTableProps) {
  const [selectedIds, setSelectedIds] = useState<string[]>([])

  const folderNameById = new Map(
    folders.map((folder) => [folder.id, folder.name])
  )
  const areAllSelected =
    items.length > 0 && items.every((item) => selectedIds.includes(item.id))

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

  if (items.length === 0) {
    return <p className="py-20 text-center text-ink-muted">Nothing here yet.</p>
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col">
      <div
        className={`${itemsGridClass} h-10 shrink-0 border-b border-line px-1 text-sm text-ink-muted`}
      >
        <Checkbox
          label="Select all items"
          isChecked={areAllSelected}
          onToggle={toggleAllSelected}
        />
        <span>Name</span>
        <span>Type</span>
        <span>Folder</span>
        <span>Modified</span>
      </div>
      <div className="min-h-0 flex-1 overflow-y-auto">
        {items.map((item) => (
          <ItemRow
            key={item.id}
            item={item}
            folderName={folderNameById.get(item.folderId) ?? ""}
            isSelected={selectedIds.includes(item.id)}
            onToggleSelected={() => toggleSelected(item.id)}
          />
        ))}
      </div>
    </div>
  )
}
