import type { InputHTMLAttributes } from "react"

type TextFieldProps = Omit<
  InputHTMLAttributes<HTMLInputElement>,
  "className"
> & {
  label: string
}

export function TextField({ label, ...inputProps }: TextFieldProps) {
  return (
    <label className="flex flex-col gap-2 text-sm text-ink-muted">
      {label}
      <input
        {...inputProps}
        className="h-11 rounded-xl border border-transparent bg-raised px-4 text-base text-ink transition-colors outline-none select-text placeholder:text-ink-muted/60 focus:border-accent"
      />
    </label>
  )
}
