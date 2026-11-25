export type ItemKind = "password" | "apiKey" | "sshKey" | "file" | "note"

export type VaultItem = {
  id: string
  name: string
  kind: ItemKind
  folderId: string
  modifiedAt: number
}
