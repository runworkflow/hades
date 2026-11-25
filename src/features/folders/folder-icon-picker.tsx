import { folderIconNames, folderIcons } from "./folder-icons"
import type { FolderIconName } from "./folder-types"

type FolderIconPickerProps = {
  selected: FolderIconName
  onSelect: (iconName: FolderIconName) => void
}

export function FolderIconPicker({
  selected,
  onSelect,
}: FolderIconPickerProps) {
  return (
    <div className="flex flex-col gap-2 text-sm text-ink-muted">
      Icon
      <div className="flex gap-2">
        {folderIconNames.map((iconName) => {
          const PickerIcon = folderIcons[iconName]
          const stateClasses =
            iconName === selected
              ? "bg-accent/15 text-accent"
              : "bg-raised text-ink hover:bg-raised-hover"

          return (
            <button
              key={iconName}
              type="button"
              aria-pressed={iconName === selected}
              aria-label={iconName}
              onClick={() => onSelect(iconName)}
              className={`grid size-10 place-items-center rounded-xl transition-colors ${stateClasses}`}
            >
              <PickerIcon size={20} />
            </button>
          )
        })}
      </div>
    </div>
  )
}
