interface BattleAnimationsProps {
  shake: boolean
  flash: string | null
  children: React.ReactNode
}

export function BattleAnimations({ shake, flash, children }: BattleAnimationsProps) {
  return (
    <div
      style={{
        position: 'relative',
        animation: shake ? 'shake 200ms ease-in-out' : undefined,
        overflow: 'hidden',
      }}
    >
      {flash && (
        <div
          style={{
            position: 'absolute',
            inset: 0,
            background: flash,
            opacity: 0.4,
            pointerEvents: 'none',
            zIndex: 100,
            animation: 'flashFade 100ms ease-out forwards',
          }}
        />
      )}
      {children}
      <style>{`
        @keyframes shake {
          0%   { transform: translateX(0); }
          20%  { transform: translateX(-6px); }
          40%  { transform: translateX(6px); }
          60%  { transform: translateX(-4px); }
          80%  { transform: translateX(4px); }
          100% { transform: translateX(0); }
        }
        @keyframes flashFade {
          0%   { opacity: 0.5; }
          100% { opacity: 0; }
        }
      `}</style>
    </div>
  )
}
