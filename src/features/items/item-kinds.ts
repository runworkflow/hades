import {
  CodeIcon,
  FileIcon,
  KeyIcon,
  NotesIcon,
  ShieldKeyholeIcon,
} from "@solar-icons/react/linear"

import type { ItemKind } from "./item-types"

export const itemKinds = {
  password: {
    label: "Password",
    newTitle: "New password",
    icon: KeyIcon,
    tileClass: "bg-kind-password/15 text-kind-password",
  },
  apiKey: {
    label: "API key",
    newTitle: "New API key",
    icon: CodeIcon,
    tileClass: "bg-kind-api-key/15 text-kind-api-key",
  },
  sshKey: {
    label: "SSH or PEM key",
    newTitle: "New SSH or PEM key",
    icon: ShieldKeyholeIcon,
    tileClass: "bg-kind-ssh-key/15 text-kind-ssh-key",
  },
  file: {
    label: "Secure file",
    newTitle: "New secure file",
    icon: FileIcon,
    tileClass: "bg-white/10 text-kind-file",
  },
  note: {
    label: "Note",
    newTitle: "New note",
    icon: NotesIcon,
    tileClass: "bg-kind-note/15 text-kind-note",
  },
} satisfies Record<
  ItemKind,
  {
    label: string
    newTitle: string
    icon: typeof KeyIcon
    tileClass: string
  }
>
