import { AddIcon, PenIcon, TrashBinTrashIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { PrimaryButton } from "@/components/primary-button"
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
    <div className="ml-2.5 flex min-w-0 flex-1 items-center gap-3">
      <h1 className="text-xl font-semibold tracking-tight whitespace-nowrap">
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
      <PrimaryButton size="compact" onClick={onNewItem}>
        <AddIcon size={16} />
        New item
      </PrimaryButton>
    </div>
  )
}
