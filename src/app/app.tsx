import { QueryClientProvider } from "@tanstack/react-query"

import { queryClient } from "@/lib/query-client"
import { VaultGate } from "@/features/vault/vault-gate"

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <VaultGate />
    </QueryClientProvider>
  )
}
