import { useState } from "react"

type History<Value> = {
  entries: Value[]
  position: number
}

export function useNavigationHistory<Value>(initialValue: Value) {
  const [history, setHistory] = useState<History<Value>>({
    entries: [initialValue],
    position: 0,
  })

  const current = history.entries[history.position]
  const canGoBack = history.position > 0
  const canGoForward = history.position < history.entries.length - 1

  function goTo(value: Value) {
    if (value === current) return
    const keptEntries = history.entries.slice(0, history.position + 1)
    setHistory({
      entries: [...keptEntries, value],
      position: keptEntries.length,
    })
  }

  function goBack() {
    if (canGoBack) setHistory({ ...history, position: history.position - 1 })
  }

  function goForward() {
    if (canGoForward) setHistory({ ...history, position: history.position + 1 })
  }

  return { current, canGoBack, canGoForward, goTo, goBack, goForward }
}
