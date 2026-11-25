import type { Icon } from "@solar-icons/react/lib/types"

type SidebarNavItemProps = {
  icon: Icon
  label: string
  isActive: boolean
  onSelect: () => void
}

export function SidebarNavItem({
  icon: NavIcon,
  label,
  isActive,
  onSelect,
}: SidebarNavItemProps) {
  const stateClasses = isActive ? "bg-raised font-medium" : "hover:bg-white/5"

  return (
    <button
      type="button"
      onClick={onSelect}
      className={`flex w-full items-center gap-3.5 rounded-full px-3.5 py-2.5 text-left text-base transition-colors ${stateClasses}`}
    >
      <NavIcon size={20} />
      <span className="truncate">{label}</span>
    </button>
  )
}
