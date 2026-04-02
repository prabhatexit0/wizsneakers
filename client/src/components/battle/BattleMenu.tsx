import { useEffect, useState } from 'react'

interface BattleMenuProps {
  canFlee: boolean
  onFight: () => void
  onBag: () => void
  onSneakers: () => void
  onRun: () => void
  disabled: boolean
}

const MENU_ITEMS = ['FIGHT', 'BAG', 'SNEAKERS', 'RUN'] as const
type MenuItem = typeof MENU_ITEMS[number]

export function BattleMenu({ canFlee, onFight, onBag, onSneakers, onRun, disabled }: BattleMenuProps) {
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
          setSelected(prev => (prev % 2 === 1 ? prev : prev + 1 < 4 ? prev + 1 : prev))
          break
        case 'ArrowUp':
          setSelected(prev => (prev >= 2 ? prev - 2 : prev))
          break
        case 'ArrowDown':
          setSelected(prev => (prev < 2 ? prev + 2 : prev))
          break
        case 'KeyZ':
        case 'Enter':
          e.preventDefault()
          confirmSelection(selected)
          break
      }
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [disabled, selected]) // eslint-disable-line react-hooks/exhaustive-deps

  function confirmSelection(idx: number) {
    const item = MENU_ITEMS[idx]
    if (item === 'FIGHT') onFight()
    else if (item === 'BAG') onBag()
    else if (item === 'SNEAKERS') onSneakers()
    else if (item === 'RUN' && canFlee) onRun()
  }

  function handleClick(item: MenuItem) {
    if (disabled) return
    if (item === 'RUN' && !canFlee) return
    if (item === 'FIGHT') onFight()
    else if (item === 'BAG') onBag()
    else if (item === 'SNEAKERS') onSneakers()
    else if (item === 'RUN') onRun()
  }

  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr 1fr',
        gap: 6,
        padding: '8px 0',
      }}
    >
      <div
        style={{
          gridColumn: '1 / -1',
          fontSize: 13,
          color: '#aaa',
          marginBottom: 4,
          fontFamily: 'monospace',
        }}
      >
        What will you do?
      </div>
      {MENU_ITEMS.map((item, i) => {
        const isRun = item === 'RUN'
        const isDisabled = isRun && !canFlee
        const isSelected = selected === i
        return (
          <button
            key={item}
            onClick={() => handleClick(item)}
            disabled={isDisabled || disabled}
            style={{
              padding: '10px 14px',
              background: isSelected ? '#2c3e50' : '#1a1a2e',
              color: isDisabled ? '#444' : isSelected ? '#fff' : '#ddd',
              border: isSelected ? '2px solid #3498db' : '2px solid #333',
              borderRadius: 6,
              cursor: isDisabled || disabled ? 'not-allowed' : 'pointer',
              fontFamily: 'monospace',
              fontSize: 14,
              fontWeight: 'bold',
              textAlign: 'left',
              transform: isSelected ? 'scale(1.02)' : 'scale(1)',
              transition: 'all 0.1s',
            }}
          >
            {item}
            {isRun && !canFlee && (
              <span style={{ fontSize: 10, color: '#555', display: 'block', fontWeight: 'normal' }}>
                Trainer battle
              </span>
            )}
          </button>
        )
      })}
    </div>
  )
}
