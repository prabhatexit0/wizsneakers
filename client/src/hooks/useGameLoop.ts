import { useEffect, useRef } from 'react'

export function useGameLoop(callback: (dt: number) => void, active: boolean) {
  const callbackRef = useRef(callback)
  callbackRef.current = callback

  useEffect(() => {
    if (!active) return

    let rafId: number
    let lastTime = 0
    // Tick rate limiter: move every ~150ms for grid-based feel
    let accumulator = 0
    const TICK_INTERVAL = 150

    function frame(time: number) {
      const dt = lastTime ? time - lastTime : 0
      lastTime = time
      accumulator += dt

      if (accumulator >= TICK_INTERVAL) {
        callbackRef.current(accumulator)
        accumulator = 0
      }

      rafId = requestAnimationFrame(frame)
    }

    rafId = requestAnimationFrame(frame)
    return () => cancelAnimationFrame(rafId)
  }, [active])
}
