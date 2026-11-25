import { callCommand } from "@/lib/call-command"

import type {
  ItemDetail,
  NewItemInput,
  UpdateItemInput,
  VaultItem,
} from "./item-types"

export const itemApi = {
  list: () => callCommand<VaultItem[]>("list_items"),
  get: (itemId: string) => callCommand<ItemDetail>("get_item", { itemId }),
  create: (input: NewItemInput) =>
    callCommand<VaultItem>("create_item", { ...input }),
  revealField: (itemId: string, key: string) =>
    callCommand<string>("reveal_item_field", { itemId, key }),
  copyField: (itemId: string, key: string) =>
    callCommand<void>("copy_item_field", { itemId, key }),
  update: (itemId: string, input: UpdateItemInput) =>
    callCommand<VaultItem>("update_item", { itemId, ...input }),
  remove: (itemId: string) => callCommand<void>("delete_item", { itemId }),
}
