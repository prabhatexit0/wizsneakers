import { useEffect, useRef } from 'react'

export function useGameLoop(callback: (dt: number) => void, active: boolean) {
  const callbackRef = useRef(callback)
  callbackRef.current = callback

  useEffect(() => {
    if (!active) return

    let rafId: number
    let lastTime: number | null = null

    function frame(time: number) {
      const dt = lastTime != null ? time - lastTime : 16.67
      lastTime = time
      callbackRef.current(dt)
      rafId = requestAnimationFrame(frame)
    }

    rafId = requestAnimationFrame(frame)
    return () => cancelAnimationFrame(rafId)
  }, [active])
}
