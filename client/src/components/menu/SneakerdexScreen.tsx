import { useEffect, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'

type DexFilter = 'All' | 'Retro' | 'Techwear' | 'Skate' | 'HighFashion'
const DEX_FILTERS: DexFilter[] = ['All', 'Retro', 'Techwear', 'Skate', 'HighFashion']

interface DexEntry {
  number: number
  seen: boolean
  caught: boolean
  name: string | null
  faction: string | null
  description: string | null
  base_stats: {
    durability: number
    hype: number
    comfort: number
    drip: number
    rarity: number
  } | null
}

interface DexData {
  entries: DexEntry[]
  total_seen: number
  total_caught: number
  total_species: number
}

interface SneakerdexScreenProps {
  engine: GameEngine
}

export function SneakerdexScreen({ engine }: SneakerdexScreenProps) {
  const anyEng = engine as unknown as Record<string, () => string>
  const [filter, setFilter] = useState<DexFilter>('All')
  const [selectedIdx, setSelectedIdx] = useState(0)

  const dexData = JSON.parse(anyEng['get_sneakerdex']()) as DexData

  const filtered = dexData.entries.filter(e => {
    if (filter === 'All') return true
    if (!e.faction) return false
    return e.faction === filter
  })

  const selected = filtered[selectedIdx]

  useEffect(() => { setSelectedIdx(0) }, [filter])

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (e.code === 'ArrowLeft') {
        const i = DEX_FILTERS.indexOf(filter)
        setFilter(DEX_FILTERS[Math.max(0, i - 1)])
      }
      if (e.code === 'ArrowRight') {
        const i = DEX_FILTERS.indexOf(filter)
        setFilter(DEX_FILTERS[Math.min(DEX_FILTERS.length - 1, i + 1)])
      }
      if (e.code === 'ArrowUp') setSelectedIdx(i => Math.max(0, i - 1))
      if (e.code === 'ArrowDown') setSelectedIdx(i => Math.min(filtered.length - 1, i + 1))
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [filter, filtered.length])

  return (
    <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 12 }}>
        <div style={{ fontSize: 16, color: '#ff6b6b' }}>Sneakerdex</div>
        <div style={{ fontSize: 12, color: '#888' }}>
          Seen: {dexData.total_seen}/{dexData.total_species} &nbsp;|&nbsp;
          Caught: {dexData.total_caught}/{dexData.total_species}
        </div>
      </div>

      {/* Filter tabs */}
      <div style={{ display: 'flex', gap: 4, marginBottom: 12 }}>
        {DEX_FILTERS.map(f => (
          <button
            key={f}
            onClick={() => setFilter(f)}
            style={{
              flex: 1,
              padding: '4px 2px',
              background: filter === f ? '#ff6b6b' : '#1a1a1a',
              color: filter === f ? '#fff' : '#666',
              border: '1px solid #333',
              borderRadius: 3,
              cursor: 'pointer',
              fontFamily: '"Courier New", monospace',
              fontSize: 10,
            }}
          >
            {f === 'HighFashion' ? 'Fashion' : f}
          </button>
        ))}
      </div>

      <div style={{ display: 'flex', gap: 12 }}>
        {/* List */}
        <div style={{ width: 200, overflow: 'auto', maxHeight: 400 }}>
          {filtered.map((entry, i) => (
            <div
              key={entry.number}
              onClick={() => setSelectedIdx(i)}
              style={{
                padding: '5px 8px',
                background: selectedIdx === i ? '#1e1e1e' : 'transparent',
                cursor: 'pointer',
                fontSize: 12,
                display: 'flex',
                gap: 8,
                alignItems: 'center',
                borderRadius: 3,
                opacity: !entry.seen && !entry.caught ? 0.4 : 1,
              }}
            >
              <span style={{ color: '#555', minWidth: 32 }}>#{String(entry.number).padStart(3, '0')}</span>
              {entry.caught && <span style={{ color: '#27ae60', fontSize: 10 }}>●</span>}
              {entry.seen && !entry.caught && <span style={{ color: '#3498db', fontSize: 10 }}>○</span>}
              {!entry.seen && !entry.caught && <span style={{ color: '#333', fontSize: 10 }}>○</span>}
              <span style={{ color: selectedIdx === i ? '#fff' : '#aaa' }}>
                {entry.name ?? '???'}
              </span>
            </div>
          ))}
        </div>

        {/* Detail panel */}
        {selected && (
          <div
            style={{
              flex: 1,
              background: '#1a1a1a',
              border: '1px solid #333',
              borderRadius: 4,
              padding: 12,
              fontSize: 12,
              alignSelf: 'flex-start',
            }}
          >
            <div style={{ fontSize: 16, fontWeight: 'bold', marginBottom: 4 }}>
              #{String(selected.number).padStart(3, '0')} {selected.name ?? '???'}
            </div>
            {selected.faction && (
              <div style={{ color: '#888', marginBottom: 6 }}>Faction: {selected.faction}</div>
            )}
            {!selected.seen && !selected.caught && (
              <div style={{ color: '#444', fontStyle: 'italic' }}>Not yet encountered.</div>
            )}
            {(selected.seen || selected.caught) && !selected.caught && (
              <div>
                <div
                  style={{
                    width: 60,
                    height: 60,
                    background: '#111',
                    border: '1px solid #333',
                    borderRadius: 4,
                    marginBottom: 8,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    fontSize: 28,
                    filter: 'grayscale(1) brightness(0.3)',
                  }}
                >
                  👟
                </div>
                <div style={{ color: '#666', fontStyle: 'italic' }}>Seen in the wild.</div>
              </div>
            )}
            {selected.caught && (
              <div>
                <div
                  style={{
                    width: 60,
                    height: 60,
                    background: '#111',
                    border: '1px solid #333',
                    borderRadius: 4,
                    marginBottom: 8,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    fontSize: 28,
                  }}
                >
                  👟
                </div>
                {selected.description && (
                  <div style={{ color: '#aaa', marginBottom: 8, lineHeight: 1.5 }}>{selected.description}</div>
                )}
                {selected.base_stats && (
                  <div>
                    <div style={{ color: '#888', marginBottom: 4 }}>Base Stats:</div>
                    {Object.entries(selected.base_stats).map(([stat, val]) => (
                      <div key={stat} style={{ display: 'flex', gap: 8, marginBottom: 3 }}>
                        <span style={{ color: '#666', textTransform: 'capitalize', minWidth: 80 }}>{stat}</span>
                        <div style={{ flex: 1, height: 10, background: '#222', borderRadius: 2, overflow: 'hidden', marginTop: 1 }}>
                          <div style={{ width: `${Math.min(100, (val / 150) * 100)}%`, height: '100%', background: '#3498db', borderRadius: 2 }} />
                        </div>
                        <span style={{ color: '#aaa', minWidth: 28, textAlign: 'right' }}>{val}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
