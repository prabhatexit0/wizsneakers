export type Faction = 'Retro' | 'Techwear' | 'Skate' | 'HighFashion' | 'Normal'

export type Direction = 'up' | 'down' | 'left' | 'right'

export type GameMode = 'Overworld' | 'Battle'

export type StatKind = 'Durability' | 'Hype' | 'Comfort' | 'Drip' | 'Rarity'

export type BattleSide = 'Player' | 'Opponent'

export const FACTION_COLORS: Record<Faction, string> = {
  Retro: '#c0392b',
  Techwear: '#2980b9',
  Skate: '#27ae60',
  HighFashion: '#8e44ad',
  Normal: '#95a5a6',
}

export const FACTION_BG: Record<Faction, string> = {
  Retro: '#3d1010',
  Techwear: '#0d2a3d',
  Skate: '#0d2e17',
  HighFashion: '#250d3a',
  Normal: '#1a1a1a',
}
