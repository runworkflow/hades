const units = ["B", "KB", "MB", "GB"]
const unitStep = 1024

export function formatFileSize(bytes: number) {
  if (!Number.isFinite(bytes) || bytes < 0) return ""

  let value = bytes
  let unitIndex = 0
  while (value >= unitStep && unitIndex < units.length - 1) {
    value /= unitStep
    unitIndex += 1
  }
  const digits = unitIndex === 0 || value >= 10 ? 0 : 1
  return `${value.toFixed(digits)} ${units[unitIndex]}`
}
