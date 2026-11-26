import type { ButtonHTMLAttributes, ReactNode } from "react"
import { AltArrowRightIcon } from "@solar-icons/react/linear"

type SecurityOptionProps = Omit<
  ButtonHTMLAttributes<HTMLButtonElement>,
  "className" | "title"
> & {
  icon: ReactNode
  title: string
  description: string
  trailing?: ReactNode
  hasChevron?: boolean
}

export function SecurityOption({
  icon,
  title,
  description,
  trailing,
  hasChevron = false,
  ...buttonProps
}: SecurityOptionProps) {
  return (
    <button
      type="button"
      {...buttonProps}
      className="flex items-center gap-4 rounded-2xl bg-raised p-4 text-left transition-colors hover:bg-raised-hover"
    >
      <span className="grid size-7 shrink-0 place-items-center">{icon}</span>
      <span className="flex min-w-0 flex-1 flex-col">
        <span className="font-medium">{title}</span>
        <span className="text-sm text-ink-muted">{description}</span>
      </span>
      {trailing}
      {hasChevron && (
        <AltArrowRightIcon size={18} className="shrink-0 text-ink-muted" />
      )}
    </button>
  )
}
