import { ClockCircleIcon } from "@solar-icons/react/linear"

import { FolderSection } from "@/features/folders/folder-section"
import type { Folder } from "@/features/folders/folder-types"
import { recentViewId } from "@/features/navigation/view-ids"

import { SidebarNavItem } from "./sidebar-nav-item"

type SidebarProps = {
  isOpen: boolean
  folders: Folder[]
  activeViewId: string
  onSelectView: (viewId: string) => void
}

export function Sidebar({
  isOpen,
  folders,
  activeViewId,
  onSelectView,
}: SidebarProps) {
  const visibilityClasses = isOpen ? "opacity-100 delay-75" : "opacity-0"

  return (
    <aside
      inert={!isOpen}
      className={`min-w-0 overflow-hidden transition-opacity duration-200 ${visibilityClasses}`}
    >
      <div className="flex h-full w-66 flex-col px-3 pt-14 pb-3">
        <SidebarNavItem
          icon={ClockCircleIcon}
          label="Recent Items"
          isActive={activeViewId === recentViewId}
          onSelect={() => onSelectView(recentViewId)}
        />
        <FolderSection
          folders={folders}
          activeViewId={activeViewId}
          onSelectFolder={onSelectView}
        />
        <div className="mt-2 flex items-center gap-2 border-t border-line px-3.5 pt-3 text-sm text-ink-muted">
          <span className="size-2 rounded-full bg-success" />
          Local sync up to date
        </div>
      </div>
    </aside>
  )
}
