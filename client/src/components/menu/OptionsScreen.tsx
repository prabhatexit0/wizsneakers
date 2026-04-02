import { useEffect, useState } from 'react'

type TextSpeed = 'Slow' | 'Medium' | 'Fast' | 'Instant'
const TEXT_SPEEDS: TextSpeed[] = ['Slow', 'Medium', 'Fast', 'Instant']

const SETTINGS_KEY = 'wizsneakers_settings'

interface Settings {
  textSpeed: TextSpeed
  musicVolume: number
  sfxVolume: number
}

function loadSettings(): Settings {
  try {
    const raw = localStorage.getItem(SETTINGS_KEY)
    if (raw) return JSON.parse(raw) as Settings
  } catch { /* ignore */ }
  return { textSpeed: 'Medium', musicVolume: 70, sfxVolume: 80 }
}

function saveSettings(settings: Settings) {
  localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings))
}

export function OptionsScreen() {
  const [settings, setSettings] = useState<Settings>(loadSettings)
  const [focusRow, setFocusRow] = useState(0)

  const updateSettings = (partial: Partial<Settings>) => {
    const next = { ...settings, ...partial }
    setSettings(next)
    saveSettings(next)
  }

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.repeat) return
      if (e.code === 'ArrowUp') setFocusRow(r => Math.max(0, r - 1))
      if (e.code === 'ArrowDown') setFocusRow(r => Math.min(2, r + 1))
      if (focusRow === 0) {
        if (e.code === 'ArrowLeft') {
          const i = TEXT_SPEEDS.indexOf(settings.textSpeed)
          if (i > 0) updateSettings({ textSpeed: TEXT_SPEEDS[i - 1] })
        }
        if (e.code === 'ArrowRight') {
          const i = TEXT_SPEEDS.indexOf(settings.textSpeed)
          if (i < TEXT_SPEEDS.length - 1) updateSettings({ textSpeed: TEXT_SPEEDS[i + 1] })
        }
      } else if (focusRow === 1) {
        if (e.code === 'ArrowLeft') updateSettings({ musicVolume: Math.max(0, settings.musicVolume - 10) })
        if (e.code === 'ArrowRight') updateSettings({ musicVolume: Math.min(100, settings.musicVolume + 10) })
      } else if (focusRow === 2) {
        if (e.code === 'ArrowLeft') updateSettings({ sfxVolume: Math.max(0, settings.sfxVolume - 10) })
        if (e.code === 'ArrowRight') updateSettings({ sfxVolume: Math.min(100, settings.sfxVolume + 10) })
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [focusRow, settings])

  const rowStyle = (row: number): React.CSSProperties => ({
    background: '#1a1a1a',
    border: `1px solid ${focusRow === row ? '#ff6b6b' : '#333'}`,
    borderRadius: 4,
    padding: '12px 16px',
    marginBottom: 8,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    cursor: 'pointer',
  })

  return (
    <div style={{ fontFamily: '"Courier New", monospace', color: '#e8e8e8' }}>
      <div style={{ fontSize: 16, color: '#ff6b6b', marginBottom: 16 }}>Options</div>

      <div style={rowStyle(0)} onClick={() => setFocusRow(0)}>
        <span style={{ fontSize: 14 }}>Text Speed</span>
        <div style={{ display: 'flex', gap: 8 }}>
          {TEXT_SPEEDS.map(s => (
            <span
              key={s}
              onClick={e => { e.stopPropagation(); updateSettings({ textSpeed: s }) }}
              style={{
                padding: '4px 10px',
                fontSize: 13,
                background: settings.textSpeed === s ? '#ff6b6b' : '#222',
                color: settings.textSpeed === s ? '#fff' : '#666',
                borderRadius: 3,
                cursor: 'pointer',
              }}
            >
              {s}
            </span>
          ))}
        </div>
      </div>

      <div style={rowStyle(1)} onClick={() => setFocusRow(1)}>
        <span style={{ fontSize: 14 }}>Music Volume</span>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <span style={{ fontSize: 12, color: '#888' }}>◀</span>
          <div style={{ width: 120, height: 6, background: '#222', borderRadius: 3, overflow: 'hidden' }}>
            <div style={{ width: `${settings.musicVolume}%`, height: '100%', background: '#3498db', borderRadius: 3 }} />
          </div>
          <span style={{ fontSize: 12, color: '#888' }}>▶</span>
          <span style={{ fontSize: 13, minWidth: 36, textAlign: 'right', color: '#aaa' }}>{settings.musicVolume}</span>
        </div>
      </div>

      <div style={rowStyle(2)} onClick={() => setFocusRow(2)}>
        <span style={{ fontSize: 14 }}>SFX Volume</span>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <span style={{ fontSize: 12, color: '#888' }}>◀</span>
          <div style={{ width: 120, height: 6, background: '#222', borderRadius: 3, overflow: 'hidden' }}>
            <div style={{ width: `${settings.sfxVolume}%`, height: '100%', background: '#27ae60', borderRadius: 3 }} />
          </div>
          <span style={{ fontSize: 12, color: '#888' }}>▶</span>
          <span style={{ fontSize: 13, minWidth: 36, textAlign: 'right', color: '#aaa' }}>{settings.sfxVolume}</span>
        </div>
      </div>

      <div
        style={{
          background: '#1a1a1a',
          border: '1px solid #333',
          borderRadius: 4,
          padding: '12px 16px',
          marginTop: 8,
        }}
      >
        <div style={{ fontSize: 14, marginBottom: 8, color: '#aaa' }}>Controls</div>
        <div style={{ fontSize: 12, color: '#666', lineHeight: 1.8 }}>
          Move: WASD / Arrow Keys<br />
          Action: Z / Enter<br />
          Cancel: X / Escape<br />
          Menu: Esc (overworld)<br />
          Sprint: Hold Shift
        </div>
      </div>
    </div>
  )
}
