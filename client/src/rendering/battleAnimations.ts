import type { BattleTurnEvent, BattleRenderState, OpponentMove } from '../types/battle'

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export interface AnimationCallbacks {
  addMessage: (msg: string) => void
  setPlayerHP: (hp: number) => void
  setOpponentHP: (hp: number) => void
  setShake: (active: boolean) => void
  setFlash: (color: string | null) => void
  onCapture: (shakes: number, success: boolean) => Promise<void>
  onLevelUp: (newLevel: number) => Promise<void>
  onMoveLearnPrompt: (moveId: number) => Promise<void>
  onEvolutionPrompt: (speciesId: number) => Promise<void>
  onBattleEnd: (result: string) => void
  refreshState: () => void
}

function getMoveName(
  moveId: number,
  renderState: BattleRenderState,
  opponentMoves: OpponentMove[],
): string {
  const playerMove = renderState.available_moves.find(m => m.id === moveId)
  if (playerMove) return playerMove.name
  const oppMove = opponentMoves.find(m => m.id === moveId)
  if (oppMove) return oppMove.name
  return `Move #${moveId}`
}

function getSneakerName(side: 'Player' | 'Opponent', renderState: BattleRenderState): string {
  return side === 'Player'
    ? renderState.player_sneaker.name
    : renderState.opponent_sneaker.name
}

export async function playTurnEvents(
  events: BattleTurnEvent[],
  renderState: BattleRenderState,
  callbacks: AnimationCallbacks,
): Promise<void> {
  // Snapshot opponent moves for lookup (these don't change during animation)
  const opponentMoves = renderState.opponent_moves

  for (const event of events) {
    if ('MoveUsed' in event) {
      const { side, move_id } = event.MoveUsed
      const name = getSneakerName(side, renderState)
      const moveName = getMoveName(move_id, renderState, opponentMoves)
      callbacks.addMessage(`${name} used ${moveName}!`)
      await sleep(600)

    } else if ('Damage' in event) {
      const { side, amount, effectiveness, is_critical } = event.Damage
      if (is_critical) {
        callbacks.addMessage('A critical hit!')
        await sleep(400)
      }
      if (effectiveness === 'SuperEffective') {
        callbacks.setFlash('#ffff99')
        await sleep(100)
        callbacks.setFlash(null)
        callbacks.addMessage("It's super effective!")
        await sleep(400)
      } else if (effectiveness === 'NotVeryEffective') {
        callbacks.addMessage("It's not very effective...")
        await sleep(400)
      }
      callbacks.setShake(true)
      await sleep(200)
      callbacks.setShake(false)
      if (side === 'Player') {
        callbacks.setPlayerHP(
          Math.max(0, renderState.player_sneaker.current_hp - amount)
        )
      } else {
        callbacks.setOpponentHP(
          Math.max(0, renderState.opponent_sneaker.current_hp - amount)
        )
      }
      await sleep(500)

    } else if ('StatChange' in event) {
      const { side, stat, stages } = event.StatChange
      const name = getSneakerName(side, renderState)
      const dir = stages > 0 ? 'rose' : 'fell'
      const sharply = Math.abs(stages) >= 2 ? ' sharply' : ''
      callbacks.addMessage(`${name}'s ${stat}${sharply} ${dir}!`)
      await sleep(500)

    } else if ('StatusApplied' in event) {
      const { side, status } = event.StatusApplied
      const name = getSneakerName(side, renderState)
      callbacks.addMessage(`${name} is now ${status.toLowerCase()}!`)
      await sleep(500)

    } else if ('StatusDamage' in event) {
      const { side, amount } = event.StatusDamage
      const name = getSneakerName(side, renderState)
      callbacks.addMessage(`${name} is hurt by its status!`)
      if (side === 'Player') {
        callbacks.setPlayerHP(
          Math.max(0, renderState.player_sneaker.current_hp - amount)
        )
      } else {
        callbacks.setOpponentHP(
          Math.max(0, renderState.opponent_sneaker.current_hp - amount)
        )
      }
      await sleep(400)

    } else if ('Healed' in event) {
      const { side, amount } = event.Healed
      const name = getSneakerName(side, renderState)
      callbacks.addMessage(`${name} healed ${amount} HP!`)
      if (side === 'Player') {
        callbacks.setPlayerHP(
          Math.min(renderState.player_sneaker.max_hp, renderState.player_sneaker.current_hp + amount)
        )
      } else {
        callbacks.setOpponentHP(
          Math.min(renderState.opponent_sneaker.max_hp, renderState.opponent_sneaker.current_hp + amount)
        )
      }
      await sleep(400)

    } else if ('Fainted' in event) {
      const { side } = event.Fainted
      const name = getSneakerName(side, renderState)
      callbacks.addMessage(`${name} fainted!`)
      if (side === 'Player') {
        callbacks.setPlayerHP(0)
      } else {
        callbacks.setOpponentHP(0)
      }
      await sleep(700)

    } else if ('SwitchedIn' in event) {
      const { side } = event.SwitchedIn
      callbacks.addMessage(side === 'Player' ? 'Go!' : 'Opponent sent out a sneaker!')
      callbacks.refreshState()
      await sleep(400)

    } else if ('ItemUsed' in event) {
      callbacks.addMessage('Used an item!')
      await sleep(400)

    } else if ('FleeAttempt' in event) {
      const { success } = event.FleeAttempt
      callbacks.addMessage(success ? 'Got away safely!' : "Can't escape!")
      await sleep(500)

    } else if ('CaptureAttempt' in event) {
      const { shakes, success } = event.CaptureAttempt
      await callbacks.onCapture(shakes, success)
      if (!success) {
        callbacks.addMessage('Oh no! The sneaker broke free!')
        await sleep(500)
      }

    } else if ('XpGained' in event) {
      const { amount } = event.XpGained
      callbacks.addMessage(`Gained ${amount} XP!`)
      callbacks.refreshState()
      await sleep(400)

    } else if ('LevelUp' in event) {
      const { new_level } = event.LevelUp
      await callbacks.onLevelUp(new_level)

    } else if ('MoveLearnPrompt' in event) {
      const { move_id } = event.MoveLearnPrompt
      await callbacks.onMoveLearnPrompt(move_id)

    } else if ('EvolutionPrompt' in event) {
      const { species_id } = event.EvolutionPrompt
      await callbacks.onEvolutionPrompt(species_id)

    } else if ('BattleEnd' in event) {
      const { result } = event.BattleEnd
      if (result === 'PlayerWin') {
        callbacks.addMessage('You won!')
      } else if (result === 'PlayerLose') {
        callbacks.addMessage('You lost...')
      } else if (result === 'PlayerCapture') {
        // handled by capture animation message
      } else if (result === 'PlayerFlee') {
        // handled by flee attempt message
      }
      await sleep(600)
      callbacks.onBattleEnd(result)

    } else if ('Message' in event) {
      const { text } = event.Message
      callbacks.addMessage(text)
      await sleep(400)
    }
  }
}
