import { callCommand } from "@/lib/call-command"

import type { Folder, NewFolderInput } from "./folder-types"

export const folderApi = {
  list: () => callCommand<Folder[]>("list_folders"),
  create: (input: NewFolderInput) =>
    callCommand<Folder>("create_folder", { ...input }),
  update: (folderId: string, input: NewFolderInput) =>
    callCommand<Folder>("update_folder", { folderId, ...input }),
  remove: (folderId: string) =>
    callCommand<void>("delete_folder", { folderId }),
}
