import { QueryClientProvider } from "@tanstack/react-query"

import { queryClient } from "@/lib/query-client"
import { useRestoreScreenCaptureProtection } from "@/features/security/use-restore-screen-capture-protection"
import { VaultGate } from "@/features/vault/vault-gate"

export function App() {
  useRestoreScreenCaptureProtection()

  return (
    <QueryClientProvider client={queryClient}>
      <VaultGate />
    </QueryClientProvider>
  )
}
