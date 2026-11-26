import type { SyncSummary } from "./sync-api"

function countOf(count: number, singular: string, plural: string) {
  return `${count} ${count === 1 ? singular : plural}`
}

export function describeSyncResult(summary: SyncSummary): string[] {
  const lines: string[] = []

  if (summary.recordsPulled > 0 || summary.recordsPushed > 0) {
    lines.push(
      `Received ${summary.recordsPulled}, sent ${summary.recordsPushed}.`
    )
  }
  if (summary.conflictCopies > 0) {
    lines.push(
      `${countOf(summary.conflictCopies, "conflict copy was", "conflict copies were")} kept. Look for names ending in (conflict copy).`
    )
  }
  if (summary.settingsPulled) {
    lines.push("Your password or recovery key was changed on another Mac.")
  }
  if (summary.settingsConflict) {
    lines.push(
      "Two Macs changed the password at the same time. If your password does not work, try the other one."
    )
  }
  if (summary.settingsRejected) {
    lines.push(
      "A settings file in the folder was not made with your vault key. Hades ignored it and put your own settings back."
    )
  }
  if (summary.skippedFileCount > 0) {
    lines.push(
      `${countOf(summary.skippedFileCount, "file", "files")} in the folder could not be read and ${summary.skippedFileCount === 1 ? "was" : "were"} skipped.`
    )
  }
  if (summary.rejectedRecordCount > 0) {
    lines.push(
      `${countOf(summary.rejectedRecordCount, "record", "records")} could not be verified and ${summary.rejectedRecordCount === 1 ? "was" : "were"} not applied.`
    )
  }
  if (summary.missingBlobCount > 0) {
    lines.push(
      `${countOf(summary.missingBlobCount, "attached file has", "attached files have")} not arrived yet.`
    )
  }

  return lines.length > 0 ? lines : ["Everything is up to date."]
}
