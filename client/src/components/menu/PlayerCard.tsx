import type { GameEngine } from '../../wasm/wizsneakers_engine.js'
import { formatPlayTime } from '../../state/saveLoad'

interface PlayerInfo {
  name: string
  money: number
  play_time_ms: number
  stamps: boolean[]
  stamps_earned: number
  sneakerdex_seen: number
  sneakerdex_caught: number
}

interface PlayerCardProps {
  engine: GameEngine
}

export function PlayerCard({ engine }: PlayerCardProps) {
  const anyEng = engine as unknown as Record<string, () => string>
  const info = JSON.parse(anyEng['get_player_info']()) as PlayerInfo

  return (
    <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
      <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 16 }}>Player</div>

      <div
        style={{
          background: '#1a1a1a',
          border: '1px solid #333',
          borderRadius: 4,
          padding: 16,
          marginBottom: 12,
        }}
      >
        <div style={{ fontSize: 20, fontWeight: 'bold', marginBottom: 4 }}>{info.name}</div>
        <div style={{ fontSize: 14, color: '#f39c12' }}>${info.money.toLocaleString()}</div>
        <div style={{ fontSize: 12, color: '#aaa', marginTop: 4 }}>
          Play Time: {formatPlayTime(info.play_time_ms)}
        </div>
      </div>

      <div
        style={{
          background: '#1a1a1a',
          border: '1px solid #333',
          borderRadius: 4,
          padding: 16,
          marginBottom: 12,
        }}
      >
        <div style={{ fontSize: 13, color: '#aaa', marginBottom: 8 }}>
          Authentication Stamps ({info.stamps_earned}/8)
        </div>
        <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
          {info.stamps.map((earned, i) => (
            <div
              key={i}
              style={{
                width: 32,
                height: 32,
                borderRadius: '50%',
                background: earned ? '#ff6b6b' : '#222',
                border: `2px solid ${earned ? '#ff4444' : '#444'}`,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: 14,
              }}
            >
              {earned ? '★' : '○'}
            </div>
          ))}
        </div>
      </div>

      <div
        style={{
          background: '#1a1a1a',
          border: '1px solid #333',
          borderRadius: 4,
          padding: 16,
        }}
      >
        <div style={{ fontSize: 13, color: '#aaa', marginBottom: 8 }}>Sneakerdex</div>
        <div style={{ fontSize: 14 }}>
          Seen: <span style={{ color: '#3498db' }}>{info.sneakerdex_seen}/30</span>
          &nbsp;&nbsp;|&nbsp;&nbsp;
          Caught: <span style={{ color: '#27ae60' }}>{info.sneakerdex_caught}/30</span>
        </div>
      </div>
    </div>
  )
}
