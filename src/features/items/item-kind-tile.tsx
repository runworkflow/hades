import { itemKinds } from "./item-kinds"
import type { ItemKind } from "./item-types"

const sizes = {
  medium: { boxClass: "size-10 rounded-xl", iconSize: 20 },
  small: { boxClass: "size-6 rounded-md", iconSize: 14 },
}

type ItemKindTileProps = {
  kind: ItemKind
  size?: keyof typeof sizes
}

export function ItemKindTile({ kind, size = "medium" }: ItemKindTileProps) {
  const { icon: KindIcon, tileClass } = itemKinds[kind]
  const { boxClass, iconSize } = sizes[size]

  return (
    <span
      className={`grid shrink-0 place-items-center ${boxClass} ${tileClass}`}
    >
      <KindIcon size={iconSize} />
    </span>
  )
}
