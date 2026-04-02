import { useEffect, useState } from 'react'

interface CaptureAnimationProps {
  shakes: number
  success: boolean
  sneakerName: string
  onDone: () => void
}

type Phase =
  | 'flying'
  | 'wobble1'
  | 'wobble2'
  | 'wobble3'
  | 'success'
  | 'fail'
  | 'done'

export function CaptureAnimation({ shakes, success, sneakerName, onDone }: CaptureAnimationProps) {
  const [phase, setPhase] = useState<Phase>('flying')
  const [opacity, setOpacity] = useState(1)
  const [caseVisible, setCaseVisible] = useState(true)
  const [stars, setStars] = useState(false)

  useEffect(() => {
    let cancelled = false

    async function run() {
      const delay = (ms: number) => new Promise<void>(r => setTimeout(r, ms))

      // Case flies to opponent
      await delay(300)
      if (cancelled) return

      // Opponent disappears
      setOpacity(0)
      await delay(200)
      if (cancelled) return

      // Wobble phases
      const wobblePhases: Phase[] = ['wobble1', 'wobble2', 'wobble3']
      for (let i = 0; i < shakes; i++) {
        setPhase(wobblePhases[i] as Phase)
        await delay(600)
        if (cancelled) return
      }

      if (success) {
        setStars(true)
        setPhase('success')
        await delay(1000)
        if (cancelled) return
        onDone()
      } else {
        setPhase('fail')
        setCaseVisible(false)
        setOpacity(1) // opponent reappears
        await delay(400)
        if (cancelled) return
        onDone()
      }
    }

    run()
    return () => { cancelled = true }
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  const wobbling = phase === 'wobble1' || phase === 'wobble2' || phase === 'wobble3'

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.85)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 200,
        fontFamily: 'monospace',
        gap: 24,
      }}
    >
      {/* Sneaker indicator */}
      <div
        style={{
          width: 80,
          height: 80,
          background: '#1a1a2e',
          border: '2px solid #555',
          borderRadius: 8,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontSize: 32,
          opacity,
          transition: 'opacity 200ms',
        }}
      >
        👟
      </div>

      {/* Sneaker Case */}
      {caseVisible && (
        <div
          style={{
            fontSize: 48,
            animation: wobbling
              ? 'wobble 600ms ease-in-out'
              : phase === 'flying'
              ? 'caseFloat 300ms ease-out'
              : undefined,
          }}
        >
          {phase === 'fail' ? '💥' : '⬜'}
        </div>
      )}

      {/* Stars for success */}
      {stars && (
        <div style={{ fontSize: 24, animation: 'starburst 500ms ease-out' }}>
          ✨ ⭐ ✨
        </div>
      )}

      {/* Message */}
      {phase === 'success' && (
        <div style={{ color: '#ffd700', fontSize: 18, fontWeight: 'bold', textAlign: 'center' }}>
          Gotcha!<br />
          <span style={{ fontSize: 14, color: '#e0e0e0' }}>{sneakerName} was caught!</span>
        </div>
      )}
      {phase === 'fail' && (
        <div style={{ color: '#e74c3c', fontSize: 16 }}>Oh no! It broke free!</div>
      )}

      <style>{`
        @keyframes wobble {
          0%   { transform: rotate(0deg); }
          25%  { transform: rotate(-15deg); }
          50%  { transform: rotate(15deg); }
          75%  { transform: rotate(-10deg); }
          100% { transform: rotate(0deg); }
        }
        @keyframes caseFloat {
          0%   { transform: translateY(60px); opacity: 0; }
          100% { transform: translateY(0); opacity: 1; }
        }
        @keyframes starburst {
          0%   { transform: scale(0); opacity: 0; }
          60%  { transform: scale(1.3); opacity: 1; }
          100% { transform: scale(1); opacity: 1; }
        }
      `}</style>
    </div>
  )
}
