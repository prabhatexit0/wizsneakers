import type { SneakerSummary, StatStages } from '../../types/battle'
import { FACTION_COLORS } from '../../types/game'

interface BattleHUDProps {
  sneaker: SneakerSummary
  side: 'player' | 'opponent'
  displayedHP: number
  stages?: StatStages
}

function hpColor(current: number, max: number): string {
  const pct = current / max
  if (pct > 0.5) return '#2ecc71'
  if (pct > 0.25) return '#f39c12'
  return '#e74c3c'
}

function stageArrows(stages: StatStages): Array<{ label: string; value: number }> {
  return [
    { label: 'HYP', value: stages.hype },
    { label: 'COM', value: stages.comfort },
    { label: 'DRP', value: stages.drip },
    { label: 'RAR', value: stages.rarity },
  ].filter(s => s.value !== 0)
}

export function BattleHUD({ sneaker, side, displayedHP, stages }: BattleHUDProps) {
  const hpPct = Math.max(0, Math.min(1, displayedHP / sneaker.max_hp))
  const color = hpColor(displayedHP, sneaker.max_hp)
  const factionColor = FACTION_COLORS[sneaker.faction] ?? '#95a5a6'

  const xpPct =
    side === 'player' &&
    sneaker.current_xp !== undefined &&
    sneaker.xp_current_level !== undefined &&
    sneaker.xp_next_level !== undefined &&
    sneaker.xp_next_level > sneaker.xp_current_level
      ? (sneaker.current_xp - sneaker.xp_current_level) /
        (sneaker.xp_next_level - sneaker.xp_current_level)
      : 0

  const activeStages = stages ? stageArrows(stages) : []

  return (
    <div
      style={{
        background: side === 'player' ? '#0f1e0f' : '#1e0f0f',
        border: `2px solid ${factionColor}`,
        borderRadius: 8,
        padding: '10px 14px',
        minWidth: 200,
        fontFamily: 'monospace',
      }}
    >
      {/* Name row */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 4 }}>
        <span
          style={{
            fontWeight: 'bold',
            fontSize: 14,
            color: '#eee',
            textShadow: `0 0 8px ${factionColor}`,
          }}
        >
          {sneaker.name}
        </span>
        <span style={{ fontSize: 12, color: '#aaa' }}>Lv.{sneaker.level}</span>
        {sneaker.status && (
          <span
            style={{
              fontSize: 10,
              background: '#7f8c8d',
              color: '#fff',
              borderRadius: 3,
              padding: '1px 4px',
              textTransform: 'uppercase',
            }}
          >
            {sneaker.status.substring(0, 3)}
          </span>
        )}
        {/* Stat stage arrows */}
        {activeStages.map(s => (
          <span
            key={s.label}
            style={{
              fontSize: 9,
              color: s.value > 0 ? '#2ecc71' : '#e74c3c',
              fontWeight: 'bold',
            }}
          >
            {s.label}{s.value > 0 ? '▲' : '▼'}
          </span>
        ))}
      </div>

      {/* HP bar */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
        <span style={{ fontSize: 10, color: '#888', width: 20 }}>HP</span>
        <div
          style={{
            flex: 1,
            height: 8,
            background: '#2a2a2a',
            borderRadius: 4,
            overflow: 'hidden',
          }}
        >
          <div
            style={{
              height: '100%',
              width: `${hpPct * 100}%`,
              background: color,
              borderRadius: 4,
              transition: 'width 500ms ease-out, background 300ms',
            }}
          />
        </div>
        {side === 'player' && (
          <span style={{ fontSize: 11, color, minWidth: 60, textAlign: 'right' }}>
            {displayedHP}/{sneaker.max_hp}
          </span>
        )}
      </div>

      {/* XP bar — player only */}
      {side === 'player' && (
        <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginTop: 4 }}>
          <span style={{ fontSize: 10, color: '#888', width: 20 }}>XP</span>
          <div
            style={{
              flex: 1,
              height: 4,
              background: '#2a2a2a',
              borderRadius: 2,
              overflow: 'hidden',
            }}
          >
            <div
              style={{
                height: '100%',
                width: `${Math.max(0, Math.min(1, xpPct)) * 100}%`,
                background: '#3498db',
                borderRadius: 2,
                transition: 'width 300ms ease-out',
              }}
            />
          </div>
        </div>
      )}
    </div>
  )
}
