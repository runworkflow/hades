import { CloseCircleIcon } from "@solar-icons/react/linear"

import { FormError } from "@/components/form-error"
import { IconButton } from "@/components/icon-button"
import { formatRelativeTime } from "@/lib/format-relative-time"

import { ItemFieldRow } from "./item-field-row"
import { itemKinds } from "./item-kinds"
import type { ItemDetail } from "./item-types"
import { useItemDetail } from "./use-items"

type ItemDetailContentProps = {
  detail: ItemDetail
  folderName: string
  onClose: () => void
}

function ItemDetailContent({
  detail,
  folderName,
  onClose,
}: ItemDetailContentProps) {
  const kind = itemKinds[detail.kind]
  const KindIcon = kind.icon

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
    </>
  )
}

type ItemDetailPanelProps = {
  itemId: string
  folderName: string
  onClose: () => void
}

export function ItemDetailPanel({
  itemId,
  folderName,
  onClose,
}: ItemDetailPanelProps) {
  const detail = useItemDetail(itemId)

  return (
    <aside className="flex w-[29rem] shrink-0 flex-col gap-6 overflow-y-auto border-l border-line p-6">
      {detail.data && (
        <ItemDetailContent
          detail={detail.data}
          folderName={folderName}
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
