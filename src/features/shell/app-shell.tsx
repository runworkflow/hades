import { useState } from "react"
import { SidebarMinimalisticIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { ItemsView } from "@/features/items/items-view"
import { selectItemsForView } from "@/features/items/select-items"
import { useNavigationHistory } from "@/features/navigation/use-navigation-history"
import { recentViewId } from "@/features/navigation/view-ids"
import { sampleFolders, sampleItems } from "@/features/vault/sample-vault"
import { useActivityReporter } from "@/features/vault/use-activity-reporter"
import { useLockVault } from "@/features/vault/use-vault"

import { PaneHeader } from "./pane-header"
import { Sidebar } from "./sidebar"

export function AppShell() {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true)
  const navigation = useNavigationHistory(recentViewId)
  const lockVault = useLockVault()
  useActivityReporter()

  const activeFolder = sampleFolders.find(
    (folder) => folder.id === navigation.current
  )
  const title = activeFolder?.name ?? "Recent Items"
  const columnsClass = isSidebarOpen
    ? "grid-cols-[16.5rem_1fr]"
    : "grid-cols-[0rem_1fr]"
  const paneMarginClass = isSidebarOpen ? "ml-0" : "ml-2"

  return (
    <div
      className={`relative grid h-screen bg-sidebar transition-[grid-template-columns] duration-300 ease-out ${columnsClass}`}
    >
      <div
        data-tauri-drag-region
        className="absolute inset-x-0 top-0 z-10 h-12"
      />
      <IconButton
        label="Toggle sidebar"
        onClick={() => setIsSidebarOpen((isOpen) => !isOpen)}
        className="absolute top-3 left-23 z-20"
      >
        <SidebarMinimalisticIcon size={20} />
      </IconButton>
      <Sidebar
        isOpen={isSidebarOpen}
        folders={sampleFolders}
        activeViewId={navigation.current}
        onSelectView={navigation.goTo}
        onLock={() => lockVault.mutate()}
      />
      <main
        className={`mt-2 mr-2 mb-2 flex min-w-0 flex-col overflow-hidden rounded-pane bg-pane transition-[margin] duration-300 ease-out ${paneMarginClass}`}
      >
        <PaneHeader
          isSidebarOpen={isSidebarOpen}
          canGoBack={navigation.canGoBack}
          canGoForward={navigation.canGoForward}
          onGoBack={navigation.goBack}
          onGoForward={navigation.goForward}
        />
        <ItemsView
          key={navigation.current}
          title={title}
          items={selectItemsForView(sampleItems, navigation.current)}
          folders={sampleFolders}
        />
      </main>
    </div>
  )
}
