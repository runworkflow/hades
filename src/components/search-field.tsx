import { MagnifierIcon } from "@solar-icons/react/linear"

type SearchFieldProps = {
  value: string
  onChange: (value: string) => void
  className?: string
}

export function SearchField({
  value,
  onChange,
  className = "",
}: SearchFieldProps) {
  return (
    <label
      className={`flex h-9 w-60 items-center gap-2 rounded-full bg-raised px-3.5 text-ink-muted ${className}`}
    >
      <MagnifierIcon size={16} />
      <input
        type="search"
        value={value}
        placeholder="Search"
        autoComplete="off"
        onChange={(event) => onChange(event.target.value)}
        className="min-w-0 flex-1 bg-transparent text-ink outline-none select-text placeholder:text-ink-muted"
      />
    </label>
  )
}
