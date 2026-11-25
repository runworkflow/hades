import {
  CaseIcon,
  FolderIcon,
  ServerIcon,
  UserIcon,
  WalletIcon,
} from "@solar-icons/react/linear"

import type { FolderIconName } from "./folder-types"

export const folderIcons = {
  folder: FolderIcon,
  case: CaseIcon,
  user: UserIcon,
  wallet: WalletIcon,
  server: ServerIcon,
} satisfies Record<FolderIconName, typeof FolderIcon>
