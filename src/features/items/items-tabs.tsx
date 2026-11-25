import { FolderIcon, PulseIcon } from "@solar-icons/react/linear"

export type ItemsTab = "items" | "activity"

const tabs = [
  { id: "items", label: "Items", icon: FolderIcon },
  { id: "activity", label: "Activity", icon: PulseIcon },
] satisfies { id: ItemsTab; label: string; icon: typeof FolderIcon }[]

type ItemsTabsProps = {
  activeTab: ItemsTab
  onChange: (tab: ItemsTab) => void
}

export function ItemsTabs({ activeTab, onChange }: ItemsTabsProps) {
  return (
    <div className="flex gap-3">
      {tabs.map(({ id, label, icon: TabIcon }) => {
        const stateClasses =
          id === activeTab ? "bg-accent/15 text-accent" : "bg-raised"

        return (
          <button
            key={id}
            type="button"
            onClick={() => onChange(id)}
            className={`flex items-center gap-2 rounded-full px-4 py-2.5 font-medium ${stateClasses}`}
          >
            <TabIcon size={18} />
            {label}
          </button>
        )
      })}
    </div>
  )
}
