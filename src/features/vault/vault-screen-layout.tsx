import type { ReactNode } from "react"
import { ShieldIcon } from "@solar-icons/react/linear"

type VaultScreenLayoutProps = {
  title: string
  description: string
  children: ReactNode
}

export function VaultScreenLayout({
  title,
  description,
  children,
}: VaultScreenLayoutProps) {
  return (
    <main className="relative grid h-screen place-items-center bg-sidebar px-6">
      <div data-tauri-drag-region className="absolute inset-x-0 top-0 h-12" />
      <div className="flex w-full max-w-sm flex-col items-center gap-6">
        <span className="grid size-14 place-items-center rounded-2xl border border-line bg-black">
          <ShieldIcon size={28} />
        </span>
        <div className="text-center">
          <h1 className="text-2xl font-semibold">{title}</h1>
          <p className="mt-2 text-ink-muted">{description}</p>
        </div>
        {children}
      </div>
    </main>
  )
}
