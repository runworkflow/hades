const minutesPerHour = 60
const hoursPerDay = 24
const millisecondsPerMinute = 60_000

export function formatRelativeTime(timestamp: number, now = Date.now()) {
  const minutes = Math.round((now - timestamp) / millisecondsPerMinute)
  if (minutes < 1) return "Just now"
  if (minutes < minutesPerHour) return `${minutes} min ago`

  const hours = Math.round(minutes / minutesPerHour)
  if (hours < hoursPerDay) return `${hours} h ago`

  return `${Math.round(hours / hoursPerDay)} d ago`
}
