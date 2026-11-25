import { callCommand } from "@/lib/call-command"

import type { Folder, NewFolderInput } from "./folder-types"

export const folderApi = {
  list: () => callCommand<Folder[]>("list_folders"),
  create: (input: NewFolderInput) =>
    callCommand<Folder>("create_folder", { ...input }),
}
