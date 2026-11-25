import { CheckIcon } from "@solar-icons/react/linear"

type CheckboxProps = {
  label: string
  isChecked: boolean
  onToggle: () => void
}

export function Checkbox({ label, isChecked, onToggle }: CheckboxProps) {
  const stateClasses = isChecked
    ? "border-accent bg-accent text-white"
    : "border-white/20 bg-white/5 hover:border-white/40"

  return (
    <button
      type="button"
      role="checkbox"
      aria-checked={isChecked}
      aria-label={label}
      onClick={onToggle}
      className={`grid size-5 place-items-center rounded-md border-[1.5px] transition-colors ${stateClasses}`}
    >
      {isChecked && <CheckIcon size={13} strokeWidth={2.5} />}
    </button>
  )
}
