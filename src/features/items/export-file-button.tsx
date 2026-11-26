import { FormError } from "@/components/form-error"

import { useExportFile } from "./use-items"

export function ExportFileButton({ itemId }: { itemId: string }) {
  const exportFile = useExportFile(itemId)

  return (
    <div className="flex flex-col gap-2">
      <button
        type="button"
        disabled={exportFile.isPending}
        onClick={() => exportFile.mutate()}
        className="h-11 rounded-full bg-accent font-medium text-sidebar transition-opacity disabled:cursor-default disabled:opacity-40"
      >
        {exportFile.isPending ? "Decrypting file" : "Export file"}
      </button>
      <FormError message={exportFile.error?.message} />
      <p className="px-1 text-xs text-ink-muted">
        {exportFile.data === true
          ? "Exported. The saved copy is not encrypted."
          : "An exported copy is saved without encryption."}
      </p>
    </div>
  )
}
