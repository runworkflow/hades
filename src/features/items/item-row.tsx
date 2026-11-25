import { Checkbox } from "@/components/checkbox"
import { formatRelativeTime } from "@/lib/format-relative-time"

import { itemKinds } from "./item-kinds"
import type { VaultItem } from "./item-types"
import { itemsGridClass } from "./items-grid"

type ItemRowProps = {
  item: VaultItem
  folderName: string
  isSelected: boolean
  onToggleSelected: () => void
}

export function ItemRow({
  item,
  folderName,
  isSelected,
  onToggleSelected,
}: ItemRowProps) {
  const kind = itemKinds[item.kind]
  const KindIcon = kind.icon

  return (
    <div
      className={`${itemsGridClass} h-14 border-b border-line px-1 hover:bg-white/[0.03]`}
    >
      <Checkbox
        label={`Select ${item.name}`}
        isChecked={isSelected}
        onToggle={onToggleSelected}
      />
      <div className="flex min-w-0 items-center gap-3.5">
        <span
          className={`grid size-8 shrink-0 place-items-center rounded-lg ${kind.tileClass}`}
        >
          <KindIcon size={17} />
        </span>
        <span className="truncate text-base">{item.name}</span>
      </div>
      <span className="text-sm text-ink-muted">{kind.label}</span>
      <span className="truncate text-sm text-ink-muted">{folderName}</span>
      <span className="text-sm text-ink-muted">
        {formatRelativeTime(item.modifiedAt)}
      </span>
    </div>
  )
}
