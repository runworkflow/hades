import type { ButtonHTMLAttributes } from "react"

const sizeClasses = {
  full: "h-11 w-full",
  compact:
    "inline-flex h-9 shrink-0 items-center gap-1.5 px-4 whitespace-nowrap",
}

type PrimaryButtonProps = Omit<
  ButtonHTMLAttributes<HTMLButtonElement>,
  "className"
> & {
  size?: keyof typeof sizeClasses
}

export function PrimaryButton({
  size = "full",
  children,
  ...buttonProps
}: PrimaryButtonProps) {
  return (
    <button
      {...buttonProps}
      className={`rounded-full bg-accent font-medium text-sidebar transition-opacity disabled:cursor-default disabled:opacity-40 ${sizeClasses[size]}`}
    >
      {children}
    </button>
  )
}
