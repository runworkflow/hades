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

export const folderIconNames = Object.keys(folderIcons) as FolderIconName[]

function isFolderIconName(value: string): value is FolderIconName {
  return Object.prototype.hasOwnProperty.call(folderIcons, value)
}

export function getFolderIcon(iconName: string) {
  return isFolderIconName(iconName) ? folderIcons[iconName] : folderIcons.folder
}

export function toFolderIconName(value: string): FolderIconName {
  return isFolderIconName(value) ? value : "folder"
}
