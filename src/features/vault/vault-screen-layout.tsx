import type { ReactNode } from "react"

import logoUrl from "@/assets/logo.png"

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
        <img src={logoUrl} alt="Hades" className="size-16 rounded-2xl" />
        <div className="text-center">
          <h1 className="text-2xl font-semibold">{title}</h1>
          <p className="mt-2 text-ink-muted">{description}</p>
        </div>
        {children}
      </div>
    </main>
  )
}
