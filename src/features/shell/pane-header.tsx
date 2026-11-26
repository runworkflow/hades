import type { ReactNode } from "react"
import {
  AltArrowLeftIcon,
  AltArrowRightIcon,
  SidebarMinimalisticIcon,
} from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"

type PaneHeaderProps = {
  isSidebarOpen: boolean
  canGoBack: boolean
  canGoForward: boolean
  onToggleSidebar: () => void
  onGoBack: () => void
  onGoForward: () => void
  children?: ReactNode
}

export function PaneHeader({
  isSidebarOpen,
  canGoBack,
  canGoForward,
  onToggleSidebar,
  onGoBack,
  onGoForward,
  children,
}: PaneHeaderProps) {
  const offsetClass = isSidebarOpen ? "ml-5" : "ml-24"

  return (
    <div
      className={`relative z-20 mr-2 flex h-13 shrink-0 items-center gap-1.5 transition-[margin] duration-300 ease-out ${offsetClass}`}
    >
      <IconButton label="Toggle sidebar" onClick={onToggleSidebar}>
        <SidebarMinimalisticIcon size={20} />
      </IconButton>
      <IconButton label="Back" disabled={!canGoBack} onClick={onGoBack}>
        <AltArrowLeftIcon size={20} />
      </IconButton>
      <IconButton
        label="Forward"
        disabled={!canGoForward}
        onClick={onGoForward}
      >
        <AltArrowRightIcon size={20} />
      </IconButton>
      {children}
    </div>
  )
}
