export type FolderIconName = "folder" | "case" | "user" | "wallet" | "server"

export type Folder = {
  id: string
  name: string
  iconName: FolderIconName
}
