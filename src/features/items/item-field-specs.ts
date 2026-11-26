import type { CreatableItemKind, ItemKind } from "./item-types"

export type FieldSpec = {
  key: string
  label: string
  isSecret: boolean
  isMultiline: boolean
}

export const itemFieldSpecs: Record<CreatableItemKind, FieldSpec[]> = {
  password: [
    { key: "username", label: "Username", isSecret: false, isMultiline: false },
    { key: "password", label: "Password", isSecret: true, isMultiline: false },
    { key: "website", label: "Website", isSecret: false, isMultiline: false },
  ],
  apiKey: [
    { key: "service", label: "Service", isSecret: false, isMultiline: false },
    { key: "key", label: "Key", isSecret: true, isMultiline: false },
  ],
  sshKey: [
    { key: "host", label: "Host", isSecret: false, isMultiline: false },
    {
      key: "privateKey",
      label: "Private key",
      isSecret: true,
      isMultiline: true,
    },
  ],
  note: [{ key: "text", label: "Note", isSecret: true, isMultiline: true }],
}

export const creatableItemKinds = Object.keys(
  itemFieldSpecs
) as CreatableItemKind[]

const fileFieldSpecs: FieldSpec[] = [
  { key: "fileName", label: "File name", isSecret: false, isMultiline: false },
  { key: "size", label: "Size", isSecret: false, isMultiline: false },
]

export function getFieldSpec(kind: ItemKind, key: string) {
  const specs = kind === "file" ? fileFieldSpecs : itemFieldSpecs[kind]
  return specs.find((spec) => spec.key === key)
}
