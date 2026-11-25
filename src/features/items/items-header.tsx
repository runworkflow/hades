import { AddIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { SearchField } from "@/components/search-field"

type ItemsHeaderProps = {
  title: string
  searchText: string
  onSearchChange: (value: string) => void
  onNewItem: () => void
}

export function ItemsHeader({
  title,
  searchText,
  onSearchChange,
  onNewItem,
}: ItemsHeaderProps) {
  return (
    <div className="flex items-center gap-4">
      <IconButton label="New item" size="large" onClick={onNewItem}>
        <AddIcon size={22} />
      </IconButton>
      <span className="h-7 w-px bg-white/15" />
      <h1 className="text-3xl font-bold tracking-tight whitespace-nowrap">
        {title}
      </h1>
      <SearchField
        value={searchText}
        onChange={onSearchChange}
        className="ml-auto"
      />
    </div>
  )
}
