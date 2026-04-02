import { useEffect, useState } from 'react'
import type { PartySneaker } from '../../types/battle'
import { FACTION_COLORS } from '../../types/game'

interface PartyScreenProps {
  party: PartySneaker[]
  onSwitch: (index: number) => void
  onCancel: () => void
}

export function PartyScreen({ party, onSwitch, onCancel }: PartyScreenProps) {
  const [selected, setSelected] = useState(0)

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (e.code === 'ArrowUp') setSelected(p => Math.max(0, p - 1))
      if (e.code === 'ArrowDown') setSelected(p => Math.min(party.length - 1, p + 1))
      if (e.code === 'KeyZ' || e.code === 'Enter') {
        e.preventDefault()
        const snk = party[selected]
        if (snk && !snk.is_active && !snk.is_fainted) {
          onSwitch(selected)
        }
      }
      if (e.code === 'KeyX' || e.code === 'Escape') onCancel()
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [selected, party, onSwitch, onCancel])

  return (
    <div style={{ fontFamily: 'monospace', color: '#ddd' }}>
      <div style={{ fontSize: 13, color: '#aaa', marginBottom: 8 }}>Choose a sneaker:</div>
      {party.map((snk, i) => {
        const hpPct = snk.current_hp / snk.max_hp
        const hpColor = hpPct > 0.5 ? '#2ecc71' : hpPct > 0.25 ? '#f39c12' : '#e74c3c'
        const factionColor = FACTION_COLORS[snk.faction] ?? '#95a5a6'
        const isSelected = selected === i

        return (
          <div
            key={snk.uid}
            onClick={() => {
              if (!snk.is_active && !snk.is_fainted) {
                setSelected(i)
                onSwitch(i)
              }
            }}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 10,
              padding: '8px 10px',
              background: isSelected ? '#1e2e1e' : '#111',
              border: isSelected ? `2px solid ${factionColor}` : '2px solid #1a1a1a',
              borderRadius: 6,
              marginBottom: 4,
              cursor:
                snk.is_active || snk.is_fainted ? 'not-allowed' : 'pointer',
              opacity: snk.is_fainted ? 0.5 : 1,
              position: 'relative',
            }}
          >
            {/* Faction color strip */}
            <div
              style={{
                width: 4,
                height: 36,
                background: factionColor,
                borderRadius: 2,
                flexShrink: 0,
              }}
            />

            <div style={{ flex: 1 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 3 }}>
                <span style={{ fontWeight: 'bold', fontSize: 14 }}>{snk.name}</span>
                <span style={{ color: '#888', fontSize: 12 }}>Lv.{snk.level}</span>
                {snk.status && (
                  <span
                    style={{
                      fontSize: 10,
                      background: '#7f8c8d',
                      color: '#fff',
                      borderRadius: 3,
                      padding: '1px 4px',
                    }}
                  >
                    {snk.status.substring(0, 3)}
                  </span>
                )}
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <div
                  style={{
                    flex: 1,
                    height: 6,
                    background: '#2a2a2a',
                    borderRadius: 3,
                    overflow: 'hidden',
                  }}
                >
                  <div
                    style={{
                      height: '100%',
                      width: `${Math.max(0, hpPct * 100)}%`,
                      background: hpColor,
                      borderRadius: 3,
                    }}
                  />
                </div>
                <span style={{ fontSize: 11, color: hpColor, minWidth: 50, textAlign: 'right' }}>
                  {snk.current_hp}/{snk.max_hp}
                </span>
              </div>
            </div>

            {snk.is_active && (
              <span
                style={{
                  position: 'absolute',
                  right: 10,
                  top: 6,
                  fontSize: 10,
                  color: '#2ecc71',
                  fontWeight: 'bold',
                  background: '#0d2e17',
                  padding: '2px 6px',
                  borderRadius: 3,
                }}
              >
                IN BATTLE
              </span>
            )}
            {snk.is_fainted && (
              <span
                style={{
                  position: 'absolute',
                  right: 10,
                  top: 6,
                  fontSize: 10,
                  color: '#e74c3c',
                  fontWeight: 'bold',
                  background: '#2e0d0d',
                  padding: '2px 6px',
                  borderRadius: 3,
                }}
              >
                FAINTED
              </span>
            )}
          </div>
        )
      })}
      <div style={{ fontSize: 11, color: '#555', marginTop: 6 }}>
        ↑↓ Navigate · Z Select · X Back
      </div>
    </div>
  )
}
