import { useState } from "react"
import { SidebarMinimalisticIcon } from "@solar-icons/react/linear"

import { IconButton } from "@/components/icon-button"
import { ItemsWorkspace } from "@/features/items/items-workspace"
import { useItems } from "@/features/items/use-items"
import { useNavigationHistory } from "@/features/navigation/use-navigation-history"
import { recentViewId } from "@/features/navigation/view-ids"
import { useFolders } from "@/features/folders/use-folders"
import { SecurityDialog } from "@/features/security/security-dialog"
import { useActivityReporter } from "@/features/vault/use-activity-reporter"
import { useLockVault } from "@/features/vault/use-vault"

import { PaneHeader } from "./pane-header"
import { Sidebar } from "./sidebar"

export function AppShell() {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true)
  const [isSecurityOpen, setIsSecurityOpen] = useState(false)
  const navigation = useNavigationHistory(recentViewId)
  const lockVault = useLockVault()
  useActivityReporter()
  const foldersQuery = useFolders()
  const folders = foldersQuery.data ?? []
  const items = useItems().data ?? []

  const activeFolder = folders.find(
    (folder) => folder.id === navigation.current
  )
  const isFolderMissing =
    foldersQuery.isSuccess &&
    navigation.current !== recentViewId &&
    !activeFolder
  const viewId = isFolderMissing ? recentViewId : navigation.current
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
        folders={folders}
        activeViewId={viewId}
        onSelectView={navigation.goTo}
        onLock={() => lockVault.mutate()}
        onOpenSecurity={() => setIsSecurityOpen(true)}
      />
      <main
        className={`mt-2 mr-2 mb-2 flex min-w-0 overflow-hidden rounded-pane bg-pane transition-[margin] duration-300 ease-out ${paneMarginClass}`}
      >
        <ItemsWorkspace
          key={viewId}
          header={
            <PaneHeader
              isSidebarOpen={isSidebarOpen}
              canGoBack={navigation.canGoBack}
              canGoForward={navigation.canGoForward}
              onGoBack={navigation.goBack}
              onGoForward={navigation.goForward}
            />
          }
          title={title}
          viewId={viewId}
          folders={folders}
          items={items}
          onFolderDeleted={() => navigation.goTo(recentViewId)}
        />
      </main>
      <SecurityDialog
        isOpen={isSecurityOpen}
        onClose={() => setIsSecurityOpen(false)}
      />
    </div>
  )
}
