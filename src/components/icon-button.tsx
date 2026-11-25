import type { ButtonHTMLAttributes } from "react"

const sizeClasses = {
  medium: "size-9 text-ink-muted hover:bg-raised hover:text-ink",
  large: "size-12 bg-raised text-ink hover:bg-raised-hover",
}

type IconButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  label: string
  size?: keyof typeof sizeClasses
}

export function IconButton({
  label,
  size = "medium",
  className = "",
  children,
  ...buttonProps
}: IconButtonProps) {
  return (
    <button
      type="button"
      aria-label={label}
      className={`grid shrink-0 place-items-center rounded-full transition-colors disabled:hover:bg-transparent ${sizeClasses[size]} ${className}`}
      {...buttonProps}
    >
      {children}
    </button>
  )
}
