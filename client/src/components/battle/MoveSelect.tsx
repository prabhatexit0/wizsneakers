import { useEffect, useState } from 'react'
import type { MoveDisplay } from '../../types/battle'
import { FACTION_COLORS } from '../../types/game'

interface MoveSelectProps {
  moves: MoveDisplay[]
  onSelect: (index: number) => void
  onCancel: () => void
  disabled: boolean
}

export function MoveSelect({ moves, onSelect, onCancel, disabled }: MoveSelectProps) {
  const [selected, setSelected] = useState(0)

  useEffect(() => {
    if (disabled) return
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      switch (e.code) {
        case 'ArrowLeft':
          setSelected(prev => (prev % 2 === 0 ? prev : prev - 1))
          break
        case 'ArrowRight':
          setSelected(prev => {
            if (prev % 2 === 1) return prev
            const next = prev + 1
            return next < moves.length ? next : prev
          })
          break
        case 'ArrowUp':
          setSelected(prev => (prev >= 2 ? prev - 2 : prev))
          break
        case 'ArrowDown':
          setSelected(prev => {
            const next = prev + 2
            return next < moves.length ? next : prev
          })
          break
        case 'KeyZ':
        case 'Enter': {
          e.preventDefault()
          const mv = moves[selected]
          if (mv && mv.current_pp > 0) onSelect(selected)
          break
        }
        case 'KeyX':
        case 'Escape':
          onCancel()
          break
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [disabled, selected, moves, onSelect, onCancel])

  const selectedMove = moves[selected]

  return (
    <div style={{ fontFamily: 'monospace' }}>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: '1fr 1fr',
          gap: 6,
          marginBottom: 8,
        }}
      >
        {[0, 1, 2, 3].map(i => {
          const mv = moves[i]
          const isSelected = selected === i
          const isEmpty = !mv
          const isExhausted = mv && mv.current_pp === 0

          const factionColor = mv ? FACTION_COLORS[mv.faction] ?? '#95a5a6' : '#333'

          return (
            <button
              key={i}
              onClick={() => {
                if (mv && mv.current_pp > 0 && !disabled) {
                  setSelected(i)
                  onSelect(i)
                }
              }}
              disabled={isEmpty || !!isExhausted || disabled}
              style={{
                padding: '10px 12px',
                background: isEmpty
                  ? '#111'
                  : isExhausted
                  ? '#1a1a1a'
                  : isSelected
                  ? factionColor + '33'
                  : '#181828',
                border: isSelected
                  ? `2px solid ${factionColor}`
                  : `2px solid #2a2a2a`,
                borderRadius: 6,
                cursor: isEmpty || isExhausted || disabled ? 'not-allowed' : 'pointer',
                textAlign: 'left',
                transition: 'all 0.1s',
              }}
            >
              {isEmpty ? (
                <span style={{ color: '#333', fontSize: 13 }}>—</span>
              ) : (
                <>
                  <div
                    style={{
                      fontWeight: 'bold',
                      fontSize: 13,
                      color: isExhausted ? '#444' : '#eee',
                      marginBottom: 2,
                    }}
                  >
                    {mv.name}
                  </div>
                  <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 11 }}>
                    <span
                      style={{
                        color: factionColor,
                        opacity: isExhausted ? 0.4 : 1,
                        fontWeight: 'bold',
                        textTransform: 'uppercase',
                      }}
                    >
                      {mv.faction}
                    </span>
                    <span style={{ color: isExhausted ? '#555' : '#aaa' }}>
                      {mv.current_pp}/{mv.max_pp} PP
                    </span>
                  </div>
                </>
              )}
            </button>
          )
        })}
      </div>

      {/* Move info panel */}
      {selectedMove && (
        <div
          style={{
            background: '#111',
            border: '1px solid #2a2a2a',
            borderRadius: 6,
            padding: '8px 12px',
            fontSize: 12,
            color: '#aaa',
            display: 'flex',
            gap: 16,
          }}
        >
          <span>
            <span style={{ color: '#666' }}>Cat: </span>
            {selectedMove.category}
          </span>
          {selectedMove.power > 0 && (
            <span>
              <span style={{ color: '#666' }}>Pwr: </span>
              {selectedMove.power}
            </span>
          )}
          <span>
            <span style={{ color: '#666' }}>Acc: </span>
            {selectedMove.accuracy}%
          </span>
        </div>
      )}

      <div style={{ fontSize: 11, color: '#555', marginTop: 6 }}>
        X / Esc — Back
      </div>
    </div>
  )
}
