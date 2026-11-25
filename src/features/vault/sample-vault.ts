import type { Folder } from "@/features/folders/folder-types"
import type { VaultItem } from "@/features/items/item-types"

const minute = 60_000
const hour = 60 * minute
const day = 24 * hour
const now = Date.now()

export const sampleFolders: Folder[] = [
  { id: "folder-work", name: "Work", iconName: "case" },
  { id: "folder-personal", name: "Personal", iconName: "user" },
  { id: "folder-finance", name: "Finance", iconName: "wallet" },
  { id: "folder-servers", name: "Servers", iconName: "server" },
]

export const sampleItems: VaultItem[] = [
  {
    id: "item-1",
    name: "Demo mail login",
    kind: "password",
    folderId: "folder-work",
    modifiedAt: now - 12 * minute,
  },
  {
    id: "item-2",
    name: "Demo payments key",
    kind: "apiKey",
    folderId: "folder-work",
    modifiedAt: now - 3 * hour,
  },
  {
    id: "item-3",
    name: "Staging box",
    kind: "sshKey",
    folderId: "folder-servers",
    modifiedAt: now - day,
  },
  {
    id: "item-4",
    name: "demo-certificate.pem",
    kind: "file",
    folderId: "folder-servers",
    modifiedAt: now - 2 * day,
  },
  {
    id: "item-5",
    name: "Recovery codes",
    kind: "note",
    folderId: "folder-personal",
    modifiedAt: now - 4 * day,
  },
  {
    id: "item-6",
    name: "Demo bank portal",
    kind: "password",
    folderId: "folder-finance",
    modifiedAt: now - 6 * day,
  },
]
