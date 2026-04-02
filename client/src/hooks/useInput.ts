import { useEffect, useRef } from 'react'

// Direction codes matching Rust: 0=none, 1=up, 2=down, 3=left, 4=right
const KEY_MAP: Record<string, number> = {
  ArrowUp: 1, KeyW: 1, ArrowDown: 2, KeyS: 2,
  ArrowLeft: 3, KeyA: 3, ArrowRight: 4, KeyD: 4,
}

export function useInput() {
  const directionRef = useRef(0)
  const keysDown = useRef(new Set<number>())

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      const dir = KEY_MAP[e.code]
      if (dir !== undefined) {
        e.preventDefault()
        keysDown.current.add(dir)
        directionRef.current = dir
      }
    }

    function onKeyUp(e: KeyboardEvent) {
      const dir = KEY_MAP[e.code]
      if (dir !== undefined) {
        keysDown.current.delete(dir)
        // If other keys still held, use the last one
        const remaining = [...keysDown.current]
        directionRef.current = remaining.length > 0 ? remaining[remaining.length - 1] : 0
      }
    }

    window.addEventListener('keydown', onKeyDown)
    window.addEventListener('keyup', onKeyUp)
    return () => {
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('keyup', onKeyUp)
    }
  }, [])

  return directionRef
}
