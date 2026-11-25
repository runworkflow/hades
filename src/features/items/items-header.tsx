import { AddIcon, PenIcon, TrashBinTrashIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { SearchField } from "@/components/search-field"

export type FolderActions = {
  onEdit: () => void
  onDelete: () => void
}

type ItemsHeaderProps = {
  title: string
  searchText: string
  onSearchChange: (value: string) => void
  onNewItem: () => void
  folderActions: FolderActions | null
}

export function ItemsHeader({
  title,
  searchText,
  onSearchChange,
  onNewItem,
  folderActions,
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
      {folderActions && (
        <div className="flex gap-1">
          <IconButton label="Edit folder" onClick={folderActions.onEdit}>
            <PenIcon size={18} />
          </IconButton>
          <IconButton label="Delete folder" onClick={folderActions.onDelete}>
            <TrashBinTrashIcon size={18} />
          </IconButton>
        </div>
      )}
      <SearchField
        value={searchText}
        onChange={onSearchChange}
        className="ml-auto"
      />
    </div>
  )
}
