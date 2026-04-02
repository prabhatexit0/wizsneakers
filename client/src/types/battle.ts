import type { Faction, BattleSide, StatKind } from './game'

export interface SneakerSummary {
  uid: number
  species_id: number
  name: string
  level: number
  current_hp: number
  max_hp: number
  faction: Faction
  rarity_tier: string
  status: string | null
  // Player only
  current_xp?: number
  xp_current_level?: number
  xp_next_level?: number
}

export interface StatStages {
  hype: number
  comfort: number
  drip: number
  rarity: number
}

export interface MoveDisplay {
  id: number
  name: string
  faction: Faction
  category: string
  power: number
  accuracy: number
  current_pp: number
  max_pp: number
}

export interface OpponentMove {
  id: number
  name: string
  faction: Faction
}

export type WaitingFor =
  | { type: 'MoveLearn'; move_id: number }
  | { type: 'Evolution'; species_id: number }
  | null

export interface BattleRenderState {
  player_sneaker: SneakerSummary
  opponent_sneaker: SneakerSummary
  player_stages: StatStages
  opponent_stages: StatStages
  available_moves: MoveDisplay[]
  opponent_moves: OpponentMove[]
  can_flee: boolean
  is_wild: boolean
  waiting_for: WaitingFor
}

// Party member summary (from get_party_state)
export interface PartySneaker extends SneakerSummary {
  is_active: boolean
  is_fainted: boolean
}

// Bag item entry (from get_bag_items)
export interface BagItem {
  id: number
  name: string
  qty: number
  description: string
  category: string
}

export interface BagItems {
  heal: BagItem[]
  battle: BagItem[]
  cases: BagItem[]
}

// Battle turn event union type — mirrors Rust BattleTurnEvent
export type BattleTurnEvent =
  | { MoveUsed: { side: BattleSide; move_id: number } }
  | { Damage: { side: BattleSide; amount: number; effectiveness: string; is_critical: boolean } }
  | { StatChange: { side: BattleSide; stat: StatKind; stages: number } }
  | { StatusApplied: { side: BattleSide; status: string } }
  | { StatusDamage: { side: BattleSide; amount: number } }
  | { Healed: { side: BattleSide; amount: number } }
  | { Fainted: { side: BattleSide } }
  | { SwitchedIn: { side: BattleSide; species_id: number } }
  | { ItemUsed: { item_id: number } }
  | { FleeAttempt: { success: boolean } }
  | { CaptureAttempt: { shakes: number; success: boolean } }
  | { XpGained: { amount: number } }
  | { LevelUp: { new_level: number } }
  | { MoveLearnPrompt: { move_id: number } }
  | { EvolutionPrompt: { species_id: number } }
  | { BattleEnd: { result: string } }
  | { Message: { text: string } }
