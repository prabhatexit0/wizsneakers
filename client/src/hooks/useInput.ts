import { useEffect, useRef } from 'react'

type InputKey = 'up' | 'down' | 'left' | 'right' | 'action' | 'cancel' | 'menu' | 'none'

// Movement direction keys
const DIR_KEYS: Record<string, InputKey> = {
  ArrowUp: 'up',   KeyW: 'up',
  ArrowDown: 'down', KeyS: 'down',
  ArrowLeft: 'left', KeyA: 'left',
  ArrowRight: 'right', KeyD: 'right',
}

// Action / UI keys
const ACTION_KEYS: Record<string, InputKey> = {
  KeyZ: 'action', Enter: 'action', Space: 'action',
  KeyX: 'cancel', Backspace: 'cancel',
  Escape: 'menu',
}

export function useInput() {
  // Tracks which direction keys are currently held (for priority resolution)
  const dirKeysDown = useRef<InputKey[]>([])
  // Tracks whether a non-direction key (action/cancel/menu) is active this frame
  const actionKeyDown = useRef<InputKey | null>(null)
  // Sprint modifier (Shift held)
  const sprintRef = useRef(false)
  // The current combined input string polled each frame
  const inputRef = useRef<string>('none')

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      // Prevent default for all registered keys
      if (DIR_KEYS[e.code] || ACTION_KEYS[e.code] || e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
        e.preventDefault()
      }

      if (e.repeat) return // prevent key-repeat — only register on first press

      if (e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
        sprintRef.current = true
        updateInput()
        return
      }

      const dir = DIR_KEYS[e.code]
      if (dir) {
        if (!dirKeysDown.current.includes(dir)) {
          dirKeysDown.current.push(dir)
        }
        updateInput()
        return
      }

      const action = ACTION_KEYS[e.code]
      if (action) {
        actionKeyDown.current = action
        updateInput()
      }
    }

    function onKeyUp(e: KeyboardEvent) {
      if (e.code === 'ShiftLeft' || e.code === 'ShiftRight') {
        sprintRef.current = false
        updateInput()
        return
      }

      const dir = DIR_KEYS[e.code]
      if (dir) {
        dirKeysDown.current = dirKeysDown.current.filter(d => d !== dir)
        updateInput()
        return
      }

      const action = ACTION_KEYS[e.code]
      if (action && actionKeyDown.current === action) {
        actionKeyDown.current = null
        updateInput()
      }
    }

    function updateInput() {
      // Action/menu keys take priority over movement
      const action = actionKeyDown.current
      if (action) {
        inputRef.current = action
        return
      }

      // Most recently pressed direction wins
      const dirs = dirKeysDown.current
      if (dirs.length > 0) {
        const dir = dirs[dirs.length - 1]
        inputRef.current = sprintRef.current ? `sprint_${dir}` : dir
        return
      }

      inputRef.current = 'none'
    }

    window.addEventListener('keydown', onKeyDown)
    window.addEventListener('keyup', onKeyUp)
    return () => {
      window.removeEventListener('keydown', onKeyDown)
      window.removeEventListener('keyup', onKeyUp)
    }
  }, [])

  return inputRef
}
