import type { TextareaHTMLAttributes } from "react"

type TextAreaFieldProps = Omit<
  TextareaHTMLAttributes<HTMLTextAreaElement>,
  "className"
> & {
  label: string
}

export function TextAreaField({ label, ...textareaProps }: TextAreaFieldProps) {
  return (
    <label className="flex flex-col gap-2 text-sm text-ink-muted">
      {label}
      <textarea
        {...textareaProps}
        rows={4}
        spellCheck={false}
        className="resize-y rounded-xl border border-transparent bg-raised px-4 py-3 font-mono text-sm text-ink transition-colors outline-none select-text placeholder:text-ink-muted/60 focus:border-accent"
      />
    </label>
  )
}
