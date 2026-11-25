import type { ButtonHTMLAttributes } from "react"

export function PrimaryButton({
  children,
  ...buttonProps
}: Omit<ButtonHTMLAttributes<HTMLButtonElement>, "className">) {
  return (
    <button
      {...buttonProps}
      className="h-11 w-full rounded-full bg-accent font-medium text-sidebar transition-opacity disabled:cursor-default disabled:opacity-40"
    >
      {children}
    </button>
  )
}
