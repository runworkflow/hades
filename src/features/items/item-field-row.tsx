import {
  CheckCircleIcon,
  CopyIcon,
  EyeClosedIcon,
  EyeIcon,
} from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"
import { IconButton } from "@/components/icon-button"
import { formatFileSize } from "@/lib/format-file-size"

import { getFieldSpec } from "./item-field-specs"
import type { ItemFieldView, ItemKind } from "./item-types"
import { useCopyField, useRevealField } from "./use-items"

const maskedValue = "••••••••••"

type ItemFieldRowProps = {
  itemId: string
  kind: ItemKind
  field: ItemFieldView
}

export function ItemFieldRow({ itemId, kind, field }: ItemFieldRowProps) {
  const spec = getFieldSpec(kind, field.key)
  const reveal = useRevealField(itemId, field.key)
  const copy = useCopyField(itemId, field.key)

  const revealedValue = reveal.data
  const isHidden = field.isSecret && revealedValue === undefined
  const plainValue =
    kind === "file" && field.key === "size"
      ? formatFileSize(Number(field.value))
      : (field.value ?? "")
  const displayValue = field.isSecret
    ? (revealedValue ?? maskedValue)
    : plainValue
  const textClasses = spec?.isMultiline && !isHidden ? "font-mono text-sm" : ""
  const errorMessage = reveal.error?.message ?? copy.error?.message

  return (
    <div className="flex flex-col gap-1.5 rounded-xl bg-raised px-4 py-3">
      <span className="text-xs tracking-wide text-ink-muted uppercase">
        {spec?.label ?? field.key}
      </span>
      <div className="flex items-start gap-1">
        {field.hasValue ? (
          <span
            className={`min-w-0 flex-1 break-all whitespace-pre-wrap ${textClasses}`}
          >
            {displayValue}
          </span>
        ) : (
          <span className="flex-1 text-ink-muted">Not set</span>
        )}
        {field.hasValue && (
          <div className="flex shrink-0">
            {field.isSecret && (
              <IconButton
                label={isHidden ? "Reveal" : "Hide"}
                onClick={() => (isHidden ? reveal.mutate() : reveal.reset())}
              >
                {isHidden ? <EyeIcon size={18} /> : <EyeClosedIcon size={18} />}
              </IconButton>
            )}
            <IconButton label="Copy" onClick={() => copy.mutate()}>
              {copy.isSuccess ? (
                <CheckCircleIcon size={18} />
              ) : (
                <CopyIcon size={18} />
              )}
            </IconButton>
          </div>
        )}
      </div>
      <FormError message={errorMessage} />
      {copy.isSuccess && (
        <span className="text-xs text-ink-muted">
          Copied. The clipboard clears in 30 seconds.
        </span>
      )}
    </div>
  )
}
