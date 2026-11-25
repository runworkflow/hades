import { useState } from "react"

import type { Folder } from "@/features/folders/folder-types"

import type { VaultItem } from "./item-types"
import { ItemsHeader } from "./items-header"
import { ItemsTable } from "./items-table"
import { ItemsTabs, type ItemsTab } from "./items-tabs"
import { filterItemsBySearch } from "./select-items"

type ItemsViewProps = {
  title: string
  items: VaultItem[]
  folders: Folder[]
}

export function ItemsView({ title, items, folders }: ItemsViewProps) {
  const [searchText, setSearchText] = useState("")
  const [activeTab, setActiveTab] = useState<ItemsTab>("items")

  return (
    <section className="flex min-h-0 flex-1 flex-col px-8 pt-6">
      <ItemsHeader
        title={title}
        searchText={searchText}
        onSearchChange={setSearchText}
      />
      <div className="my-6">
        <ItemsTabs activeTab={activeTab} onChange={setActiveTab} />
      </div>
      {activeTab === "items" ? (
        <ItemsTable
          items={filterItemsBySearch(items, searchText)}
          folders={folders}
        />
      ) : (
        <p className="py-20 text-center text-ink-muted">No activity yet.</p>
      )}
    </section>
  )
}
