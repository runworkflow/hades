import { AddIcon, AltArrowDownIcon } from "@solar-icons/react/linear"

import { DropdownMenu } from "@/components/dropdown-menu"
import { PrimaryButton } from "@/components/primary-button"
import { SearchField } from "@/components/search-field"

import { creatableItemKinds } from "./item-field-specs"
import { ItemKindTile } from "./item-kind-tile"
import { itemKinds } from "./item-kinds"
import type { ItemKind } from "./item-types"

export type FolderActions = {
  onEdit: () => void
  onDelete: () => void
}

const newItemKinds: ItemKind[] = [...creatableItemKinds, "file"]

type ItemsHeaderProps = {
  title: string
  searchText: string
  onSearchChange: (value: string) => void
  onNewItem: (kind: ItemKind) => void
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
        <DropdownMenu
          label="Folder actions"
          items={[
            { label: "Edit folder", onSelect: folderActions.onEdit },
            {
              label: "Delete folder",
              onSelect: folderActions.onDelete,
              isDanger: true,
            },
          ]}
        />
      )}
      <SearchField
        value={searchText}
        onChange={onSearchChange}
        className="ml-auto"
      />
      <DropdownMenu
        label="New item"
        align="right"
        items={newItemKinds.map((kind) => ({
          label: itemKinds[kind].label,
          icon: <ItemKindTile kind={kind} size="small" />,
          onSelect: () => onNewItem(kind),
        }))}
        renderTrigger={({ isOpen, onToggle }) => (
          <PrimaryButton
            size="compact"
            aria-haspopup="menu"
            aria-expanded={isOpen}
            onClick={onToggle}
          >
            <AddIcon size={16} />
            New item
            <AltArrowDownIcon size={14} />
          </PrimaryButton>
        )}
      />
    </div>
  )
}
