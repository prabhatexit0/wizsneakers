import { useCallback, useEffect, useRef, useState } from 'react'
import type { GameEngine } from '../../wasm/wizsneakers_engine.js'
import type {
  BattleRenderState,
  BattleTurnEvent,
  BagItems,
  PartySneaker,
  MoveDisplay,
} from '../../types/battle'
import { FACTION_BG } from '../../types/game'
import { playTurnEvents } from '../../rendering/battleAnimations'
import { BattleHUD } from './BattleHUD'
import { BattleMenu } from './BattleMenu'
import { MoveSelect } from './MoveSelect'
import { BattleLog } from './BattleLog'
import { BattleAnimations } from './BattleAnimations'
import { CaptureAnimation } from './CaptureAnimation'
import { LevelUpOverlay } from './LevelUpOverlay'
import { MoveLearnPrompt } from './MoveLearnPrompt'
import { EvolutionScene } from './EvolutionScene'
import { BagScreen } from './BagScreen'
import { PartyScreen } from './PartyScreen'

type UIState =
  | 'selecting_action'
  | 'selecting_move'
  | 'selecting_bag'
  | 'selecting_party'
  | 'animating'
  | 'capture'
  | 'level_up'
  | 'move_learn'
  | 'evolution'
  | 'battle_end'

interface BattleScreenProps {
  engine: GameEngine
  onBattleEnd: () => void
}

export function BattleScreen({ engine, onBattleEnd }: BattleScreenProps) {
  const [uiState, setUiState] = useState<UIState>('selecting_action')
  const [renderState, setRenderState] = useState<BattleRenderState | null>(null)
  const [displayedPlayerHP, setDisplayedPlayerHP] = useState(0)
  const [displayedOpponentHP, setDisplayedOpponentHP] = useState(0)
  const [messages, setMessages] = useState<string[]>([])
  const [shake, setShake] = useState(false)
  const [flash, setFlash] = useState<string | null>(null)
  const [bagItems, setBagItems] = useState<BagItems>({ heal: [], battle: [], cases: [] })
  const [party, setParty] = useState<PartySneaker[]>([])
  // Capture animation state
  const [captureShakes, setCaptureShakes] = useState(0)
  const [captureSuccess, setCaptureSuccess] = useState(false)
  const captureResolveRef = useRef<(() => void) | null>(null)
  // Level up state
  const [levelUpData, setLevelUpData] = useState<{ name: string; level: number } | null>(null)
  const levelUpResolveRef = useRef<(() => void) | null>(null)
  // Move learn state
  const [moveLearnData, setMoveLearnData] = useState<{ moveId: number } | null>(null)
  const moveLearnResolveRef = useRef<((slot: number) => void) | null>(null)
  // Evolution state
  const [evolutionData, setEvolutionData] = useState<{
    name: string
    newName: string
    speciesId: number
  } | null>(null)
  const evolutionResolveRef = useRef<((accept: boolean) => void) | null>(null)

  // Fetch battle state from engine
  const refreshState = useCallback(() => {
    try {
      const json = engine.get_battle_state()
      if (!json || json === '{}') return
      const state = JSON.parse(json) as BattleRenderState
      setRenderState(state)
      return state
    } catch {
      return null
    }
  }, [engine])

  // Initial load
  useEffect(() => {
    const state = refreshState()
    if (state) {
      setDisplayedPlayerHP(state.player_sneaker.current_hp)
      setDisplayedOpponentHP(state.opponent_sneaker.current_hp)
      setMessages([`A wild ${state.opponent_sneaker.name} appeared!`])
    }
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  // Submit an action to the engine and process events
  const submitAction = useCallback(async (actionJson: string) => {
    if (!renderState) return
    setUiState('animating')

    let events: BattleTurnEvent[]
    try {
      const eventsJson = engine.battle_action(actionJson)
      events = JSON.parse(eventsJson) as BattleTurnEvent[]
    } catch {
      setUiState('selecting_action')
      return
    }

    // Snapshot current state for animation reference
    const stateSnapshot = { ...renderState }
    stateSnapshot.player_sneaker = { ...renderState.player_sneaker }
    stateSnapshot.opponent_sneaker = { ...renderState.opponent_sneaker }

    await playTurnEvents(events, stateSnapshot, {
      addMessage: (msg) => setMessages(prev => [...prev, msg]),
      setPlayerHP: (hp) => setDisplayedPlayerHP(hp),
      setOpponentHP: (hp) => setDisplayedOpponentHP(hp),
      setShake: (active) => {
        setShake(active)
        if (active) setTimeout(() => setShake(false), 210)
      },
      setFlash: (color) => {
        setFlash(color)
        if (color) setTimeout(() => setFlash(null), 110)
      },
      onCapture: (shakes, success) =>
        new Promise<void>(resolve => {
          setCaptureShakes(shakes)
          setCaptureSuccess(success)
          captureResolveRef.current = resolve
          setUiState('capture')
        }),
      onLevelUp: (newLevel) =>
        new Promise<void>(resolve => {
          const name = renderState.player_sneaker.name
          setLevelUpData({ name, level: newLevel })
          levelUpResolveRef.current = resolve
          setUiState('level_up')
        }),
      onMoveLearnPrompt: (moveId) =>
        new Promise<void>(resolve => {
          setMoveLearnData({ moveId })
          // The resolve will be called after user makes a choice
          // We wrap it to also process the engine response
          moveLearnResolveRef.current = async (slot: number) => {
            try {
              const moreEventsJson = engine.battle_learn_move(slot)
              const moreEvents = JSON.parse(moreEventsJson) as BattleTurnEvent[]
              for (const ev of moreEvents) {
                if ('BattleEnd' in ev) {
                  const { result } = ev.BattleEnd
                  setMessages(prev => [...prev, result === 'PlayerWin' ? 'You won!' : 'Battle ended.'])
                  setTimeout(() => {
                    setUiState('battle_end')
                    setTimeout(onBattleEnd, 1500)
                  }, 600)
                } else if ('Message' in ev) {
                  setMessages(prev => [...prev, ev.Message.text])
                }
              }
            } catch { /* ignore */ }
            resolve()
          }
          setUiState('move_learn')
        }),
      onEvolutionPrompt: (speciesId) =>
        new Promise<void>(resolve => {
          const snkName = renderState.player_sneaker.name
          // Look up new species name from speciesId – we use species_id from renderState
          const newName = `Species #${speciesId}` // Placeholder – actual name shown by engine
          setEvolutionData({ name: snkName, newName, speciesId })
          evolutionResolveRef.current = async (accept: boolean) => {
            try {
              const moreEventsJson = engine.battle_evolution_choice(accept)
              const moreEvents = JSON.parse(moreEventsJson) as BattleTurnEvent[]
              for (const ev of moreEvents) {
                if ('Message' in ev) setMessages(prev => [...prev, ev.Message.text])
                if ('BattleEnd' in ev) {
                  setTimeout(() => {
                    setUiState('battle_end')
                    setTimeout(onBattleEnd, 1500)
                  }, 600)
                }
              }
            } catch { /* ignore */ }
            resolve()
          }
          setUiState('evolution')
        }),
      onBattleEnd: (result) => {
        if (result === 'PlayerCapture') {
          // Already handled by capture animation
        } else {
          setUiState('battle_end')
          setTimeout(onBattleEnd, 1500)
        }
      },
      refreshState: () => {
        const s = refreshState()
        if (s) {
          setDisplayedPlayerHP(s.player_sneaker.current_hp)
          setDisplayedOpponentHP(s.opponent_sneaker.current_hp)
        }
      },
    })

    // Animation done — refresh state and return to action selection
    const newState = refreshState()
    if (newState) {
      setDisplayedPlayerHP(newState.player_sneaker.current_hp)
      setDisplayedOpponentHP(newState.opponent_sneaker.current_hp)
    }

    // Check if still in battle or waiting for prompts
    const mode = engine.mode()
    if (mode !== 'Battle') {
      setUiState('battle_end')
      setTimeout(onBattleEnd, 1000)
    } else {
      const isWaiting = newState?.waiting_for != null
      if (!isWaiting) {
        setUiState('selecting_action')
      }
    }
  }, [engine, renderState, refreshState, onBattleEnd]) // eslint-disable-line react-hooks/exhaustive-deps

  function handleFight() {
    setUiState('selecting_move')
  }

  function handleBag() {
    try {
      const isWild = renderState?.is_wild ?? false
      const json = engine.get_bag_items(isWild)
      setBagItems(JSON.parse(json) as BagItems)
      const partyJson = engine.get_party_state()
      setParty(JSON.parse(partyJson) as PartySneaker[])
    } catch { /* ignore */ }
    setUiState('selecting_bag')
  }

  function handleSneakers() {
    try {
      const partyJson = engine.get_party_state()
      setParty(JSON.parse(partyJson) as PartySneaker[])
    } catch { /* ignore */ }
    setUiState('selecting_party')
  }

  function handleRun() {
    void submitAction('{"type":"run"}')
  }

  function handleMoveSelect(index: number) {
    void submitAction(JSON.stringify({ type: 'fight', move_index: index }))
  }

  function handleItemUse(itemId: number, targetIndex?: number) {
    const json =
      targetIndex !== undefined
        ? JSON.stringify({ type: 'bag', item_id: itemId, target_index: targetIndex })
        : JSON.stringify({ type: 'bag', item_id: itemId })
    void submitAction(json)
  }

  function handleSwitch(partyIndex: number) {
    void submitAction(JSON.stringify({ type: 'switch', party_index: partyIndex }))
  }

  if (!renderState) {
    return (
      <div style={{ fontFamily: 'monospace', padding: 40, color: '#888' }}>
        Loading battle...
      </div>
    )
  }

  const { player_sneaker, opponent_sneaker, player_stages, opponent_stages, available_moves } =
    renderState

  const bgColor = FACTION_BG[opponent_sneaker.faction] ?? '#0a0a0a'
  const isAnimating = uiState === 'animating'

  const moveLearnMove: MoveDisplay | null = (() => {
    if (!moveLearnData) return null
    const id = moveLearnData.moveId
    // Try to find move in available moves (might not be there yet)
    const existing = available_moves.find(m => m.id === id)
    if (existing) return existing
    // Return a placeholder with just the id
    return {
      id,
      name: `Move #${id}`,
      faction: 'Normal',
      category: 'Physical',
      power: 0,
      accuracy: 100,
      current_pp: 0,
      max_pp: 0,
    }
  })()

  return (
    <div
      style={{
        width: '100%',
        maxWidth: 560,
        minHeight: 400,
        background: `linear-gradient(160deg, ${bgColor} 0%, #0a0a0a 100%)`,
        borderRadius: 8,
        border: '2px solid #2a2a2a',
        padding: 16,
        fontFamily: 'monospace',
        position: 'relative',
      }}
    >
      <BattleAnimations shake={shake} flash={flash}>
        {/* Top area — opponent */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'flex-start',
            marginBottom: 8,
          }}
        >
          <BattleHUD
            sneaker={opponent_sneaker}
            side="opponent"
            displayedHP={displayedOpponentHP}
            stages={opponent_stages}
          />
          {/* Opponent sprite placeholder */}
          <div
            style={{
              width: 80,
              height: 80,
              background: '#111',
              border: '1px solid #333',
              borderRadius: 8,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: 36,
              opacity: displayedOpponentHP > 0 ? 1 : 0,
              transition: 'opacity 300ms',
            }}
          >
            👟
          </div>
        </div>

        {/* Middle area — player sprite */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'flex-end',
            marginBottom: 12,
          }}
        >
          {/* Player sprite placeholder */}
          <div
            style={{
              width: 80,
              height: 80,
              background: '#111',
              border: '1px solid #333',
              borderRadius: 8,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: 36,
            }}
          >
            🎮
          </div>
          <BattleHUD
            sneaker={player_sneaker}
            side="player"
            displayedHP={displayedPlayerHP}
            stages={player_stages}
          />
        </div>

        {/* Bottom area — log + action area */}
        <BattleLog messages={messages} isAnimating={isAnimating} />

        <div style={{ marginTop: 10 }}>
          {uiState === 'selecting_action' && (
            <BattleMenu
              canFlee={renderState.can_flee}
              onFight={handleFight}
              onBag={handleBag}
              onSneakers={handleSneakers}
              onRun={handleRun}
              disabled={false}
            />
          )}

          {uiState === 'selecting_move' && (
            <MoveSelect
              moves={available_moves}
              onSelect={handleMoveSelect}
              onCancel={() => setUiState('selecting_action')}
              disabled={false}
            />
          )}

          {uiState === 'selecting_bag' && (
            <BagScreen
              bagItems={bagItems}
              party={party}
              isWild={renderState.is_wild}
              onUseItem={handleItemUse}
              onCancel={() => setUiState('selecting_action')}
            />
          )}

          {uiState === 'selecting_party' && (
            <PartyScreen
              party={party}
              onSwitch={handleSwitch}
              onCancel={() => setUiState('selecting_action')}
            />
          )}

          {isAnimating && (
            <div style={{ color: '#555', fontSize: 12, textAlign: 'center', padding: 8 }}>
              ...
            </div>
          )}

          {uiState === 'battle_end' && (
            <div
              style={{
                color: '#ffd700',
                fontSize: 14,
                textAlign: 'center',
                padding: 12,
                animation: 'fadeIn 500ms',
              }}
            >
              Returning to overworld...
            </div>
          )}
        </div>
      </BattleAnimations>

      {/* Overlay components */}
      {uiState === 'capture' && (
        <CaptureAnimation
          shakes={captureShakes}
          success={captureSuccess}
          sneakerName={opponent_sneaker.name}
          onDone={() => {
            const resolve = captureResolveRef.current
            captureResolveRef.current = null
            if (captureSuccess) {
              setMessages(prev => [
                ...prev,
                `Gotcha! ${opponent_sneaker.name} was caught!`,
              ])
            }
            if (resolve) resolve()
          }}
        />
      )}

      {uiState === 'level_up' && levelUpData && (
        <LevelUpOverlay
          sneakerName={levelUpData.name}
          newLevel={levelUpData.level}
          onDone={() => {
            const resolve = levelUpResolveRef.current
            levelUpResolveRef.current = null
            setLevelUpData(null)
            setUiState('animating')
            if (resolve) resolve()
          }}
        />
      )}

      {uiState === 'move_learn' && moveLearnMove && (
        <MoveLearnPrompt
          sneakerName={player_sneaker.name}
          newMove={moveLearnMove}
          currentMoves={available_moves}
          onReplace={(slot) => {
            const resolve = moveLearnResolveRef.current
            moveLearnResolveRef.current = null
            setMoveLearnData(null)
            setUiState('animating')
            if (resolve) resolve(slot)
          }}
          onSkip={() => {
            const resolve = moveLearnResolveRef.current
            moveLearnResolveRef.current = null
            setMoveLearnData(null)
            setUiState('animating')
            if (resolve) resolve(4) // slot 4 = skip
          }}
        />
      )}

      {uiState === 'evolution' && evolutionData && (
        <EvolutionScene
          sneakerName={evolutionData.name}
          newSpeciesName={evolutionData.newName}
          onAccept={() => {
            const resolve = evolutionResolveRef.current
            evolutionResolveRef.current = null
            setEvolutionData(null)
            setUiState('animating')
            if (resolve) resolve(true)
          }}
          onCancel={() => {
            const resolve = evolutionResolveRef.current
            evolutionResolveRef.current = null
            setEvolutionData(null)
            setUiState('animating')
            if (resolve) resolve(false)
          }}
        />
      )}

      <style>{`
        @keyframes fadeIn {
          from { opacity: 0; }
          to   { opacity: 1; }
        }
      `}</style>
    </div>
  )
}
