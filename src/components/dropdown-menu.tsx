import {
  Fragment,
  useEffect,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent,
} from "react"
import { MenuDotsIcon } from "@solar-icons/react/linear"

import { IconButton } from "./icon-button"

export type DropdownMenuItem = {
  label: string
  onSelect: () => void
  isDanger?: boolean
}

type DropdownMenuProps = {
  label: string
  items: DropdownMenuItem[]
}

export function DropdownMenu({ label, items }: DropdownMenuProps) {
  const [isOpen, setIsOpen] = useState(false)
  const rootRef = useRef<HTMLDivElement>(null)
  const shouldFocusFirstRef = useRef(false)

  useEffect(() => {
    if (!isOpen) return
    if (shouldFocusFirstRef.current) {
      rootRef.current?.querySelector<HTMLElement>('[role="menuitem"]')?.focus()
    }
    function closeOnOutsidePress(event: PointerEvent) {
      if (!rootRef.current?.contains(event.target as Node)) setIsOpen(false)
    }
    function closeOnEscape(event: KeyboardEvent) {
      if (event.key === "Escape") setIsOpen(false)
    }
    document.addEventListener("pointerdown", closeOnOutsidePress)
    document.addEventListener("keydown", closeOnEscape)
    return () => {
      document.removeEventListener("pointerdown", closeOnOutsidePress)
      document.removeEventListener("keydown", closeOnEscape)
    }
  }, [isOpen])

  function toggle(event: MouseEvent<HTMLButtonElement>) {
    shouldFocusFirstRef.current = event.detail === 0
    setIsOpen((isOpenNow) => !isOpenNow)
  }

  function moveFocus(event: ReactKeyboardEvent<HTMLDivElement>) {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return
    event.preventDefault()
    const entries = Array.from(
      event.currentTarget.querySelectorAll<HTMLElement>('[role="menuitem"]')
    )
    const index = entries.indexOf(document.activeElement as HTMLElement)
    const step = event.key === "ArrowDown" ? 1 : -1
    const nextIndex =
      index < 0
        ? step === 1
          ? 0
          : entries.length - 1
        : (index + step + entries.length) % entries.length
    entries[nextIndex]?.focus()
  }

  return (
    <div ref={rootRef} className="relative">
      <IconButton
        label={label}
        aria-haspopup="menu"
        aria-expanded={isOpen}
        onClick={toggle}
      >
        <MenuDotsIcon size={18} />
      </IconButton>
      {isOpen && (
        <div
          role="menu"
          onKeyDown={moveFocus}
          className="absolute top-full left-0 z-30 mt-1 min-w-44 rounded-xl border border-line bg-raised p-1 shadow-2xl"
        >
          {items.map((item, index) => (
            <Fragment key={item.label}>
              {item.isDanger && index > 0 && (
                <div role="separator" className="my-1 h-px bg-white/10" />
              )}
              <button
                type="button"
                role="menuitem"
                onClick={() => {
                  setIsOpen(false)
                  item.onSelect()
                }}
                className={`flex w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-raised-hover focus:bg-raised-hover ${item.isDanger ? "text-danger" : "text-ink"}`}
              >
                {item.label}
              </button>
            </Fragment>
          ))}
        </div>
      )}
    </div>
  )
}
