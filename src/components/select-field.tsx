import type { SelectHTMLAttributes } from "react"

type SelectOption = {
  value: string
  label: string
}

type SelectFieldProps = Omit<
  SelectHTMLAttributes<HTMLSelectElement>,
  "className" | "children"
> & {
  label: string
  options: SelectOption[]
}

export function SelectField({
  label,
  options,
  ...selectProps
}: SelectFieldProps) {
  return (
    <label className="flex flex-col gap-2 text-sm text-ink-muted">
      {label}
      <select
        {...selectProps}
        className="h-11 rounded-xl border border-transparent bg-raised px-3 text-base text-ink transition-colors outline-none focus:border-accent"
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  )
}
