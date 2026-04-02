interface LevelUpOverlayProps {
  sneakerName: string
  newLevel: number
  onDone: () => void
}

export function LevelUpOverlay({ sneakerName, newLevel, onDone }: LevelUpOverlayProps) {
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
        gap: 16,
        animation: 'fadeIn 300ms ease-out',
      }}
    >
      <div
        style={{
          color: '#ffd700',
          fontSize: 22,
          fontWeight: 'bold',
          textAlign: 'center',
          textShadow: '0 0 16px #ffd700',
          animation: 'levelPop 500ms ease-out',
        }}
      >
        {sneakerName}<br />
        grew to Lv.{newLevel}!
      </div>

      <button
        onClick={onDone}
        autoFocus
        style={{
          marginTop: 16,
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

      <style>{`
        @keyframes fadeIn {
          from { opacity: 0; }
          to   { opacity: 1; }
        }
        @keyframes levelPop {
          0%   { transform: scale(0.6); opacity: 0; }
          60%  { transform: scale(1.1); }
          100% { transform: scale(1); opacity: 1; }
        }
      `}</style>
    </div>
  )
}
