import { AltArrowLeftIcon, AltArrowRightIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"

type PaneHeaderProps = {
  isSidebarOpen: boolean
  canGoBack: boolean
  canGoForward: boolean
  onGoBack: () => void
  onGoForward: () => void
}

export function PaneHeader({
  isSidebarOpen,
  canGoBack,
  canGoForward,
  onGoBack,
  onGoForward,
}: PaneHeaderProps) {
  const offsetClass = isSidebarOpen ? "ml-5" : "ml-34"

  return (
    <div
      className={`relative z-20 flex h-11 shrink-0 items-center gap-1.5 transition-[margin] duration-300 ease-out ${offsetClass}`}
    >
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
    </div>
  )
}
