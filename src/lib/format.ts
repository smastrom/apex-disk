/** Formats bytes into human-readable string (e.g. "1.2 GB"). */
export function formatBytes(bytes: number): string {
   if (bytes === 0) return '0 B'

   const k = 1024
   const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
   const i = Math.floor(Math.log(bytes) / Math.log(k))

   return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

/**
 * Formats a progress value for display: integer when whole (e.g. 100.0 → "100"),
 * otherwise one decimal (e.g. 32.5 → "32.5").
 */
export function formatProgressNumber(n: number): string {
   return n % 1 === 0 ? String(Math.round(n)) : n.toFixed(1)
}
