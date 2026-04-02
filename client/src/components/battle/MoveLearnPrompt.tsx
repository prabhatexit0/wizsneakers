import { useEffect, useState } from 'react'
import type { MoveDisplay } from '../../types/battle'
import { FACTION_COLORS } from '../../types/game'

interface MoveLearnPromptProps {
  sneakerName: string
  newMove: MoveDisplay
  currentMoves: MoveDisplay[]
  onReplace: (slot: number) => void
  onSkip: () => void
}

export function MoveLearnPrompt({
  sneakerName,
  newMove,
  currentMoves,
  onReplace,
  onSkip,
}: MoveLearnPromptProps) {
  const [selectedSlot, setSelectedSlot] = useState<number | null>(null)
  const [confirmSkip, setConfirmSkip] = useState(false)

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (confirmSkip) {
        if (e.code === 'KeyZ' || e.code === 'Enter') onSkip()
        if (e.code === 'KeyX' || e.code === 'Escape') setConfirmSkip(false)
        return
      }
      if (e.code === 'Digit1') { setSelectedSlot(0); onReplace(0) }
      if (e.code === 'Digit2') { setSelectedSlot(1); onReplace(1) }
      if (e.code === 'Digit3') { setSelectedSlot(2); onReplace(2) }
      if (e.code === 'Digit4') { setSelectedSlot(3); onReplace(3) }
      if (e.code === 'KeyX' || e.code === 'Escape') setConfirmSkip(true)
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [confirmSkip, onReplace, onSkip])

  const newFactionColor = FACTION_COLORS[newMove.faction] ?? '#95a5a6'

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.9)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 200,
        fontFamily: 'monospace',
        gap: 12,
        padding: 24,
      }}
    >
      <div style={{ color: '#fff', fontSize: 16, textAlign: 'center' }}>
        {sneakerName} wants to learn
      </div>

      {/* New move card */}
      <div
        style={{
          background: newFactionColor + '22',
          border: `2px solid ${newFactionColor}`,
          borderRadius: 8,
          padding: '10px 20px',
          textAlign: 'center',
          minWidth: 200,
        }}
      >
        <div style={{ fontWeight: 'bold', fontSize: 16, color: '#fff' }}>{newMove.name}</div>
        <div style={{ fontSize: 12, color: '#aaa', marginTop: 4 }}>
          {newMove.faction} · {newMove.category}
          {newMove.power > 0 && ` · Pwr ${newMove.power}`}
          {` · Acc ${newMove.accuracy}% · PP ${newMove.max_pp}`}
        </div>
      </div>

      {confirmSkip ? (
        <div style={{ textAlign: 'center', color: '#e74c3c', fontSize: 14 }}>
          Give up learning {newMove.name}?<br />
          <span style={{ fontSize: 12, color: '#aaa' }}>Z = Yes · X = No</span>
        </div>
      ) : (
        <>
          <div style={{ color: '#aaa', fontSize: 13 }}>
            Choose a move to replace (1–4), or Escape to skip:
          </div>
          <div style={{ display: 'flex', flexDirection: 'column', gap: 6, width: '100%', maxWidth: 320 }}>
            {[0, 1, 2, 3].map(i => {
              const mv = currentMoves[i]
              const isSelected = selectedSlot === i
              const factionColor = mv ? FACTION_COLORS[mv.faction] ?? '#95a5a6' : '#333'
              return (
                <button
                  key={i}
                  onClick={() => mv && onReplace(i)}
                  disabled={!mv}
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    padding: '8px 12px',
                    background: isSelected ? '#2c3e50' : '#181828',
                    border: `1px solid ${mv ? factionColor : '#333'}`,
                    borderRadius: 6,
                    color: mv ? '#ddd' : '#444',
                    fontFamily: 'monospace',
                    fontSize: 13,
                    cursor: mv ? 'pointer' : 'default',
                  }}
                >
                  <span style={{ color: '#888' }}>{i + 1}.</span>
                  <span>{mv ? mv.name : '—'}</span>
                  {mv && (
                    <span style={{ color: '#666' }}>
                      {mv.faction} · {mv.current_pp}/{mv.max_pp}PP
                    </span>
                  )}
                </button>
              )
            })}
          </div>
          <div style={{ fontSize: 11, color: '#555' }}>Press 1–4 to replace · Esc to skip</div>
        </>
      )}
    </div>
  )
}
