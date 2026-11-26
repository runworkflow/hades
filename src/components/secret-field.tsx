import { useId, useState, type InputHTMLAttributes } from "react"
import { EyeClosedIcon, EyeIcon } from "@solar-icons/react/linear"

import { IconButton } from "./icon-button"

type SecretFieldProps = Omit<
  InputHTMLAttributes<HTMLInputElement>,
  "className" | "type" | "id"
> & {
  label: string
}

export function SecretField({ label, ...inputProps }: SecretFieldProps) {
  const inputId = useId()
  const [isVisible, setIsVisible] = useState(false)
  const toggleLabel = `${isVisible ? "Hide" : "Show"} ${label.toLowerCase()}`

  return (
    <div className="flex flex-col gap-2 text-sm text-ink-muted">
      <label htmlFor={inputId}>{label}</label>
      <div className="relative">
        <input
          {...inputProps}
          id={inputId}
          type={isVisible ? "text" : "password"}
          className="h-11 w-full rounded-xl border border-transparent bg-raised pr-12 pl-4 text-base text-ink transition-colors outline-none select-text placeholder:text-ink-muted/60 focus:border-accent"
        />
        <IconButton
          label={toggleLabel}
          onClick={() => setIsVisible((isShown) => !isShown)}
          className="absolute top-1/2 right-1.5 -translate-y-1/2"
        >
          {isVisible ? <EyeClosedIcon size={18} /> : <EyeIcon size={18} />}
        </IconButton>
      </div>
    </div>
  )
}
