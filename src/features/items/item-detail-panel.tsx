import { useState } from "react"
import {
  CloseCircleIcon,
  PenIcon,
  TrashBinTrashIcon,
} from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"
import { IconButton } from "@/components/icon-button"
import type { Folder } from "@/features/folders/folder-types"
import { formatRelativeTime } from "@/lib/format-relative-time"

import { DeleteItemDialog } from "./delete-item-dialog"
import { EditItemDialog } from "./edit-item-dialog"
import { ExportFileButton } from "./export-file-button"
import { ItemFieldRow } from "./item-field-row"
import { itemKinds } from "./item-kinds"
import type { ItemDetail } from "./item-types"
import { useItemDetail } from "./use-items"

type ItemDetailContentProps = {
  detail: ItemDetail
  folders: Folder[]
  onClose: () => void
}

function ItemDetailContent({
  detail,
  folders,
  onClose,
}: ItemDetailContentProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [isDeleting, setIsDeleting] = useState(false)
  const kind = itemKinds[detail.kind]
  const KindIcon = kind.icon
  const folderName =
    folders.find((folder) => folder.id === detail.folderId)?.name ?? ""

  return (
    <>
      <div className="flex items-center gap-3">
        <span
          className={`grid size-11 shrink-0 place-items-center rounded-xl ${kind.tileClass}`}
        >
          <KindIcon size={22} />
        </span>
        <div className="min-w-0 flex-1">
          <h2 className="truncate text-lg font-semibold">{detail.name}</h2>
          <span className="text-sm text-ink-muted">{kind.label}</span>
        </div>
        <IconButton label="Edit item" onClick={() => setIsEditing(true)}>
          <PenIcon size={18} />
        </IconButton>
        <IconButton label="Delete item" onClick={() => setIsDeleting(true)}>
          <TrashBinTrashIcon size={18} />
        </IconButton>
        <IconButton label="Close details" onClick={onClose}>
          <CloseCircleIcon size={20} />
        </IconButton>
      </div>
      <div className="flex flex-col gap-3">
        {detail.fields.map((field) => (
          <ItemFieldRow
            key={field.key}
            itemId={detail.id}
            kind={detail.kind}
            field={field}
          />
        ))}
      </div>
      {detail.kind === "file" && <ExportFileButton itemId={detail.id} />}
      <dl className="flex gap-8 px-1 text-sm">
        <div>
          <dt className="text-xs tracking-wide text-ink-muted uppercase">
            Folder
          </dt>
          <dd>{folderName}</dd>
        </div>
        <div>
          <dt className="text-xs tracking-wide text-ink-muted uppercase">
            Modified
          </dt>
          <dd>{formatRelativeTime(detail.modifiedAt)}</dd>
        </div>
      </dl>
      <EditItemDialog
        isOpen={isEditing}
        detail={detail}
        kind={detail.kind}
        folders={folders}
        onClose={() => setIsEditing(false)}
      />
      <DeleteItemDialog
        isOpen={isDeleting}
        itemId={detail.id}
        itemName={detail.name}
        onClose={() => setIsDeleting(false)}
        onDeleted={onClose}
      />
    </>
  )
}

type ItemDetailPanelProps = {
  itemId: string
  folders: Folder[]
  onClose: () => void
}

export function ItemDetailPanel({
  itemId,
  folders,
  onClose,
}: ItemDetailPanelProps) {
  const detail = useItemDetail(itemId)

  return (
    <aside className="flex w-[29rem] shrink-0 flex-col gap-6 overflow-y-auto border-l border-line p-6">
      {detail.data && (
        <ItemDetailContent
          detail={detail.data}
          folders={folders}
          onClose={onClose}
        />
      )}
      {detail.isError && (
        <>
          <FormError message={detail.error.message} />
          <button
            type="button"
            onClick={onClose}
            className="h-11 rounded-full bg-raised font-medium transition-colors hover:bg-raised-hover"
          >
            Close
          </button>
        </>
      )}
    </aside>
  )
}
