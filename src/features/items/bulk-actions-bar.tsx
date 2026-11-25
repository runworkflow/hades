import { FolderIcon, TrashBinTrashIcon } from "@solar-icons/react/linear"

type BulkActionsBarProps = {
  count: number
  onMove: () => void
  onDelete: () => void
  onClear: () => void
}

const actionClasses =
  "flex items-center gap-2 rounded-full bg-raised-hover px-4 py-2.5 text-sm transition-opacity hover:opacity-80"

export function BulkActionsBar({
  count,
  onMove,
  onDelete,
  onClear,
}: BulkActionsBarProps) {
  return (
    <div className="mb-4 flex items-center gap-3 rounded-2xl bg-raised px-5 py-3.5">
      <span className="mr-auto font-medium">{count} selected</span>
      <button type="button" onClick={onMove} className={actionClasses}>
        <FolderIcon size={16} />
        Move
      </button>
      <button
        type="button"
        onClick={onDelete}
        className={`${actionClasses} text-danger`}
      >
        <TrashBinTrashIcon size={16} />
        Delete
      </button>
      <button type="button" onClick={onClear} className={actionClasses}>
        Clear
      </button>
    </div>
  )
}
