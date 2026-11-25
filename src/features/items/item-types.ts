export type ItemKind = "password" | "apiKey" | "sshKey" | "file" | "note"

export type CreatableItemKind = Exclude<ItemKind, "file">

export type VaultItem = {
  id: string
  name: string
  kind: ItemKind
  folderId: string
  modifiedAt: number
}

export type ItemFieldView = {
  key: string
  isSecret: boolean
  hasValue: boolean
  value: string | null
}

export type ItemDetail = VaultItem & {
  fields: ItemFieldView[]
}

export type UpdateItemInput = {
  folderId: string
  name: string
  fields: Record<string, string>
}

export type NewItemInput = {
  folderId: string
  kind: CreatableItemKind
  name: string
  fields: Record<string, string>
}
