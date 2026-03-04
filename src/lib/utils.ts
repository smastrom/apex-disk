/**
 * Simple debounce function.
 */
export function debounce<T extends (...args: any[]) => any>(
   fn: T,
   delay: number
): (...args: Parameters<T>) => void {
   let timeoutId: ReturnType<typeof setTimeout> | null = null

   return (...args: Parameters<T>) => {
      if (timeoutId) clearTimeout(timeoutId)
      timeoutId = setTimeout(() => fn(...args), delay)
   }
}

/**
 * Returns true if the code is running in a browser-like environment.
 */
export const isClient = typeof window !== 'undefined'

/**
 * No-operation function.
 */
export const noop = () => {}
