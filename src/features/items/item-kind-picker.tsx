import { creatableItemKinds } from "./item-field-specs"
import { itemKinds } from "./item-kinds"
import type { CreatableItemKind } from "./item-types"

type ItemKindPickerProps = {
  selected: CreatableItemKind
  onSelect: (kind: CreatableItemKind) => void
}

export function ItemKindPicker({ selected, onSelect }: ItemKindPickerProps) {
  return (
    <div className="flex flex-wrap gap-2">
      {creatableItemKinds.map((kind) => {
        const { label, icon: KindIcon } = itemKinds[kind]
        const stateClasses =
          kind === selected
            ? "bg-accent/15 text-accent"
            : "bg-raised hover:bg-raised-hover"

        return (
          <button
            key={kind}
            type="button"
            aria-pressed={kind === selected}
            onClick={() => onSelect(kind)}
            className={`flex items-center gap-2 rounded-full px-3.5 py-2 text-sm transition-colors ${stateClasses}`}
          >
            <KindIcon size={16} />
            {label}
          </button>
        )
      })}
    </div>
  )
}
