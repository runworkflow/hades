import { recentViewId } from "@/features/navigation/view-ids"

import { itemKinds } from "./item-kinds"
import type { VaultItem } from "./item-types"

export function selectItemsForView(items: VaultItem[], viewId: string) {
  const itemsInView =
    viewId === recentViewId
      ? items
      : items.filter((item) => item.folderId === viewId)

  return [...itemsInView].sort(
    (first, second) => second.modifiedAt - first.modifiedAt
  )
}

export function filterItemsBySearch(items: VaultItem[], searchText: string) {
  const query = searchText.trim().toLowerCase()
  if (!query) return items

  return items.filter(
    (item) =>
      item.name.toLowerCase().includes(query) ||
      itemKinds[item.kind].label.toLowerCase().includes(query)
  )
}
