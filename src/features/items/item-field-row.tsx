import { EyeClosedIcon, EyeIcon } from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"
import { IconButton } from "@/components/icon-button"

import { getFieldSpec } from "./item-field-specs"
import type { ItemFieldView, ItemKind } from "./item-types"
import { useRevealField } from "./use-items"

const maskedValue = "••••••••••"

type ItemFieldRowProps = {
  itemId: string
  kind: ItemKind
  field: ItemFieldView
}

function EmptyValue() {
  return <span className="text-ink-muted">Not set</span>
}

function PlainValue({ value }: { value: string }) {
  if (!value) return <EmptyValue />
  return <span className="break-words select-text">{value}</span>
}

function SecretValue({ itemId, field, isMultiline }: SecretValueProps) {
  const reveal = useRevealField(itemId, field.key)
  const revealedValue = reveal.data

  if (!field.hasValue) return <EmptyValue />

  return (
    <>
      <div className="flex items-start gap-2">
        <span
          className={`min-w-0 flex-1 break-all whitespace-pre-wrap select-text ${
            isMultiline && revealedValue !== undefined
              ? "font-mono text-sm"
              : ""
          }`}
        >
          {revealedValue ?? maskedValue}
        </span>
        <IconButton
          label={revealedValue === undefined ? "Reveal" : "Hide"}
          onClick={() =>
            revealedValue === undefined ? reveal.mutate() : reveal.reset()
          }
        >
          {revealedValue === undefined ? (
            <EyeIcon size={18} />
          ) : (
            <EyeClosedIcon size={18} />
          )}
        </IconButton>
      </div>
      <FormError message={reveal.error?.message} />
    </>
  )
}

type SecretValueProps = {
  itemId: string
  field: ItemFieldView
  isMultiline: boolean
}

export function ItemFieldRow({ itemId, kind, field }: ItemFieldRowProps) {
  const spec = getFieldSpec(kind, field.key)

  return (
    <div className="flex flex-col gap-1.5 rounded-xl bg-raised px-4 py-3">
      <span className="text-xs tracking-wide text-ink-muted uppercase">
        {spec?.label ?? field.key}
      </span>
      {field.isSecret ? (
        <SecretValue
          itemId={itemId}
          field={field}
          isMultiline={spec?.isMultiline ?? false}
        />
      ) : (
        <PlainValue value={field.value ?? ""} />
      )}
    </div>
  )
}
