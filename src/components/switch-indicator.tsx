type SwitchIndicatorProps = {
  isOn: boolean
}

export function SwitchIndicator({ isOn }: SwitchIndicatorProps) {
  const trackClass = isOn ? "bg-accent" : "bg-white/20"
  const knobClass = isOn ? "translate-x-4" : "translate-x-0"

  return (
    <span
      aria-hidden="true"
      className={`relative inline-block h-6 w-10 shrink-0 rounded-full transition-colors ${trackClass}`}
    >
      <span
        className={`absolute top-0.5 left-0.5 size-5 rounded-full bg-white shadow transition-transform ${knobClass}`}
      />
    </span>
  )
}
