import { Checkbox } from "@/components/checkbox"
import { formatRelativeTime } from "@/lib/format-relative-time"

import { itemKinds } from "./item-kinds"
import type { VaultItem } from "./item-types"
import {
  folderColumnClass,
  itemsGridClass,
  typeColumnClass,
} from "./items-grid"

type ItemRowProps = {
  item: VaultItem
  folderName: string
  isSelected: boolean
  isOpen: boolean
  onToggleSelected: () => void
  onOpen: () => void
}

export function ItemRow({
  item,
  folderName,
  isSelected,
  isOpen,
  onToggleSelected,
  onOpen,
}: ItemRowProps) {
  const kind = itemKinds[item.kind]
  const KindIcon = kind.icon
  const stateClasses = isOpen ? "bg-accent/10" : "hover:bg-white/[0.03]"

  return (
    <div
      onClick={onOpen}
      className={`${itemsGridClass} h-14 cursor-pointer border-b border-line px-1 ${stateClasses}`}
    >
      <span onClick={(event) => event.stopPropagation()}>
        <Checkbox
          label={`Select ${item.name}`}
          isChecked={isSelected}
          onToggle={onToggleSelected}
        />
      </span>
      <button
        type="button"
        className="flex min-w-0 items-center gap-3.5 text-left"
      >
        <span
          className={`grid size-8 shrink-0 place-items-center rounded-lg ${kind.tileClass}`}
        >
          <KindIcon size={17} />
        </span>
        <span className="truncate text-base">{item.name}</span>
      </button>
      <span className={`${typeColumnClass} text-sm text-ink-muted`}>
        {kind.label}
      </span>
      <span className={`${folderColumnClass} truncate text-sm text-ink-muted`}>
        {folderName}
      </span>
      <span className="text-sm text-ink-muted">
        {formatRelativeTime(item.modifiedAt)}
      </span>
    </div>
  )
}
