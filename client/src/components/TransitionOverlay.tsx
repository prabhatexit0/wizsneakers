import { useEffect, useRef, useState } from 'react'

export type TransitionType = 'walk' | 'fade' | 'warp'

interface Props {
  type: TransitionType
  onComplete: () => void
}

/**
 * TransitionOverlay renders a full-screen overlay during map transitions.
 *
 * - walk:  No overlay — the new map loads instantly.
 * - fade:  Black overlay: fade in (150ms) → hold (100ms) → fade out (150ms).
 * - warp:  White flash: fade in (100ms) → hold (50ms) → fade out (200ms).
 */
export function TransitionOverlay({ type, onComplete }: Props) {
  const [opacity, setOpacity] = useState(0)
  const doneRef = useRef(false)

  useEffect(() => {
    if (doneRef.current) return
    doneRef.current = true

    if (type === 'walk') {
      onComplete()
      return
    }

    const fadeInMs  = type === 'warp' ? 100 : 150
    const holdMs    = type === 'warp' ? 50  : 100

    // Fade in
    setOpacity(1)
    const holdTimer = window.setTimeout(() => {
      // Hold — trigger map load callback at peak opacity
      onComplete()
      // Fade out
      setOpacity(0)
    }, fadeInMs + holdMs)

    return () => {
      clearTimeout(holdTimer)
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [type])

  if (type === 'walk') return null

  const color = type === 'warp' ? '#ffffff' : '#000000'
  const fadeInMs  = type === 'warp' ? 100 : 150
  const fadeOutMs = type === 'warp' ? 200 : 150

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        backgroundColor: color,
        opacity,
        pointerEvents: 'none',
        zIndex: 9999,
        transition: opacity === 1
          ? `opacity ${fadeInMs}ms linear`
          : `opacity ${fadeOutMs}ms linear`,
      }}
    />
  )
}
