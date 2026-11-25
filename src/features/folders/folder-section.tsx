import { AddIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { SidebarNavItem } from "@/features/shell/sidebar-nav-item"

import { folderIcons } from "./folder-icons"
import type { Folder } from "./folder-types"

type FolderSectionProps = {
  folders: Folder[]
  activeViewId: string
  onSelectFolder: (folderId: string) => void
}

export function FolderSection({
  folders,
  activeViewId,
  onSelectFolder,
}: FolderSectionProps) {
  return (
    <>
      <div className="flex items-center justify-between pt-4 pr-2 pb-2 pl-3.5 font-semibold">
        <span>Folders</span>
        <IconButton label="New folder">
          <AddIcon size={18} />
        </IconButton>
      </div>
      <nav className="flex min-h-0 flex-1 flex-col gap-1 overflow-y-auto">
        {folders.map((folder) => (
          <SidebarNavItem
            key={folder.id}
            icon={folderIcons[folder.iconName]}
            label={folder.name}
            isActive={folder.id === activeViewId}
            onSelect={() => onSelectFolder(folder.id)}
          />
        ))}
      </nav>
    </>
  )
}
