import { useEffect, useState } from 'react'

interface EvolutionSceneProps {
  sneakerName: string
  newSpeciesName: string
  onAccept: () => void
  onCancel: () => void
}

type Phase = 'intro' | 'evolving' | 'done' | 'cancelled'

export function EvolutionScene({ sneakerName, newSpeciesName, onAccept, onCancel }: EvolutionSceneProps) {
  const [phase, setPhase] = useState<Phase>('intro')
  const [canCancel, setCanCancel] = useState(true)

  useEffect(() => {
    const timer = setTimeout(() => setPhase('evolving'), 1000)
    return () => clearTimeout(timer)
  }, [])

  useEffect(() => {
    if (phase !== 'evolving') return
    // After a bit, disable cancel
    const t1 = setTimeout(() => setCanCancel(false), 1500)
    // Show done state
    const t2 = setTimeout(() => setPhase('done'), 2500)
    return () => { clearTimeout(t1); clearTimeout(t2) }
  }, [phase])

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.code === 'KeyX' || e.code === 'Escape') && canCancel) {
        setPhase('cancelled')
        onCancel()
      }
      if ((e.code === 'KeyZ' || e.code === 'Enter') && phase === 'done') {
        onAccept()
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [canCancel, phase, onAccept, onCancel])

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: phase === 'evolving' ? '#fff' : 'rgba(0,0,0,0.92)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 200,
        fontFamily: 'monospace',
        gap: 16,
        transition: 'background 500ms',
      }}
    >
      {phase === 'intro' && (
        <div style={{ color: '#ffd700', fontSize: 18, textAlign: 'center' }}>
          What? {sneakerName} is evolving!
        </div>
      )}

      {phase === 'evolving' && (
        <div
          style={{
            color: '#000',
            fontSize: 24,
            fontWeight: 'bold',
            animation: 'evolveFlash 2500ms ease-in-out',
          }}
        >
          {sneakerName} → {newSpeciesName}
        </div>
      )}

      {phase === 'done' && (
        <>
          <div style={{ color: '#ffd700', fontSize: 18, textAlign: 'center' }}>
            Congratulations!<br />
            <span style={{ color: '#e0e0e0', fontSize: 15 }}>
              {sneakerName} evolved into {newSpeciesName}!
            </span>
          </div>
          <button
            onClick={onAccept}
            autoFocus
            style={{
              padding: '8px 24px',
              background: '#1a2e1a',
              border: '2px solid #2ecc71',
              color: '#2ecc71',
              borderRadius: 6,
              fontFamily: 'monospace',
              fontSize: 14,
              cursor: 'pointer',
            }}
          >
            Press Z to continue
          </button>
        </>
      )}

      {canCancel && phase !== 'done' && (
        <div style={{ fontSize: 11, color: '#888', marginTop: 8 }}>
          Hold B (X/Esc) to cancel evolution
        </div>
      )}

      <style>{`
        @keyframes evolveFlash {
          0%   { opacity: 0; }
          20%  { opacity: 1; }
          40%  { opacity: 0.2; }
          60%  { opacity: 1; }
          80%  { opacity: 0.2; }
          100% { opacity: 1; }
        }
      `}</style>
    </div>
  )
}
