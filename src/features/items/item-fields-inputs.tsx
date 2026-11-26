import { SecretField } from "@/components/secret-field"
import { TextAreaField } from "@/components/text-area-field"
import { TextField } from "@/components/text-field"

import { itemFieldSpecs } from "./item-field-specs"
import type { CreatableItemKind } from "./item-types"

const keepSecretHint = "Leave empty to keep the current value"

type ItemFieldsInputsProps = {
  kind: CreatableItemKind
  values: Record<string, string>
  onChange: (key: string, value: string) => void
  keptSecretKeys?: string[]
}

export function ItemFieldsInputs({
  kind,
  values,
  onChange,
  keptSecretKeys = [],
}: ItemFieldsInputsProps) {
  return (
    <>
      {itemFieldSpecs[kind].map((spec) => {
        const fieldKey = `${kind}-${spec.key}`
        const value = values[spec.key] ?? ""
        const placeholder = keptSecretKeys.includes(spec.key)
          ? keepSecretHint
          : undefined

        if (spec.isMultiline) {
          return (
            <TextAreaField
              key={fieldKey}
              label={spec.label}
              value={value}
              placeholder={placeholder}
              onChange={(event) => onChange(spec.key, event.target.value)}
            />
          )
        }

        const Field = spec.isSecret ? SecretField : TextField

        return (
          <Field
            key={fieldKey}
            label={spec.label}
            autoComplete="off"
            value={value}
            placeholder={placeholder}
            onChange={(event) => onChange(spec.key, event.target.value)}
          />
        )
      })}
    </>
  )
}
