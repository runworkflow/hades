import { useState } from "react"

import type { Folder } from "@/features/folders/folder-types"

import type { VaultItem } from "./item-types"
import { ItemsTable } from "./items-table"
import { ItemsTabs, type ItemsTab } from "./items-tabs"
import { filterItemsBySearch } from "./select-items"

type ItemsViewProps = {
  searchText: string
  items: VaultItem[]
  folders: Folder[]
  openItemId: string | null
  onOpenItem: (itemId: string) => void
  onItemsDeleted: (itemIds: string[]) => void
}

export function ItemsView({
  searchText,
  items,
  folders,
  openItemId,
  onOpenItem,
  onItemsDeleted,
}: ItemsViewProps) {
  const [activeTab, setActiveTab] = useState<ItemsTab>("items")

  return (
    <section className="@container flex min-h-0 min-w-0 flex-1 flex-col px-8 pt-2">
      <div className="mb-6">
        <ItemsTabs activeTab={activeTab} onChange={setActiveTab} />
      </div>
      {activeTab === "items" ? (
        <ItemsTable
          items={filterItemsBySearch(items, searchText)}
          folders={folders}
          openItemId={openItemId}
          onOpenItem={onOpenItem}
          onItemsDeleted={onItemsDeleted}
        />
      ) : (
        <p className="py-20 text-center text-ink-muted">No activity yet.</p>
      )}
    </section>
  )
}
