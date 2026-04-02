# PRD 12 — Menus, Inventory, Save/Load & Sneakerdex (Phase 7)

## Goal
Build all "between-battle" infrastructure: pause menu with tabs, item usage in overworld, shop buy/sell, save/load with 3 slots + autosave, Sneakerdex, Sneaker Clinics. After this PRD, the full game loop is complete (explore → battle → heal → manage → save).

## Dependencies
- PRD 10 (BagScreen, PartyScreen already exist from battle), PRD 11 (NPC interaction for shops/clinics)

## Deliverables

### Files to Create

**`client/src/components/menu/PauseMenu.tsx`**
- Full-screen overlay triggered by Escape/Start
- Tab bar: Party | Bag | Dex | Player | Options | Save
- Arrow keys navigate tabs, action key opens selected tab, cancel closes menu
- Dark semi-transparent background

**`client/src/components/menu/PlayerCard.tsx`**
- Player name, money ($DD), play time (HH:MM), Authentication Stamps (8 slots, filled/empty)
- Sneakerdex progress: "Seen: X/30 | Caught: Y/30"

**`client/src/components/menu/OptionsScreen.tsx`**
- Text Speed: Slow / Medium / Fast / Instant (arrow key selection)
- Music Volume: 0-100 slider (stub until audio PRD)
- SFX Volume: 0-100 slider (stub until audio PRD)
- Controls: display key bindings

**`client/src/components/menu/InventoryScreen.tsx`**
- Reuse/refactor `BagScreen.tsx` for overworld context
- 5 pocket tabs (same as battle bag)
- Heal items: select → choose party member → use
- Key items: select → show description / use (Repel, Escape Rope)
- Held items: select → equip/remove from sneaker
- No cases or battle items usable outside battle

**`client/src/components/menu/ShopScreen.tsx`**
- Triggered when interacting with shopkeeper NPC
- Two tabs: Buy | Sell
- Buy: item list with costs, quantity selector (+/-), "Total: $X", confirm
- Sell: show bag items with sell prices (50% of buy price), quantity selector
- Player money displayed at top
- "You bought [item]!" / "You sold [item]!" confirmation text

**`client/src/components/menu/SneakerdexScreen.tsx`**
- List of 30 entries (scrollable)
- Three states per entry:
  - Unseen: "???" with number only (#001, #002...)
  - Seen: silhouette/gray icon, name visible, faction shown, "Area: [location hint]"
  - Caught: full color icon, name, faction, base stats, description
- Filter tabs: All | Retro | Techwear | Skate | High-Fashion
- Selected entry shows detail panel on right

**`client/src/components/menu/SneakerBoxScreen.tsx`**
- Accessible from Sneaker Clinic NPC
- Two panels: Party (left, 6 slots) | Box (right, up to 50)
- Operations: Deposit (party→box), Withdraw (box→party)
- Must keep at least 1 in party
- Party max 6, box max 50
- Each sneaker shows: name, level, HP bar, faction color, status
- Sort options: Level, Name, Faction, Rarity

**`client/src/components/TitleScreen.tsx`**
- Shown on game start (before entering game)
- "WIZSNEAKERS" title text (styled)
- Options: "New Game" | "Continue" | "Options"
- Continue: show 3 save slots + autosave slot with preview
- Preview per slot: player name, play time, stamps earned, location, lead sneaker name/level
- Empty slot shows "— Empty —"
- New Game → name entry → start game sequence

**`client/src/components/menu/SaveScreen.tsx`**
- 3 save slots with same preview as title screen
- Select slot → "Save to Slot [X]?" → confirm/cancel
- If slot occupied: "Overwrite existing save?" warning
- "Game Saved!" confirmation (1 second)

**`client/src/state/saveLoad.ts`**
```typescript
interface SaveFile {
  version: number;
  slot: number;
  timestamp: string;
  checksum: string;
  data: string;  // JSON from engine.export_save()
  preview: SavePreview;
}

interface SavePreview {
  player_name: string;
  play_time_ms: number;
  stamps_earned: number;
  party_lead_species: number;
  party_lead_level: number;
  location_name: string;
  sneakerdex_caught: number;
}

export function saveToSlot(slot: number, engine: GameEngine): void;
export function loadFromSlot(slot: number): SaveFile | null;
export function getSlotPreviews(): (SavePreview | null)[];
export function autoSave(engine: GameEngine): void;
export function calculateChecksum(data: string): string;
```
- localStorage keys: `wizsneakers_save_1`, `_2`, `_3`, `wizsneakers_autosave`, `wizsneakers_settings`

### Files to Modify

**`engine/src/lib.rs`**
- Add WASM methods:
  - `export_save(&self) -> String` — serialize full GameState via serde_json
  - `load_save(json: &str) -> Result<GameEngine, JsValue>` — deserialize, reconstruct engine
  - `set_player_name(&mut self, name: &str)`
  - `use_item(&mut self, item_id: u16, target_index: u8) -> String` — use item on party member in overworld
  - `buy_item(&mut self, item_id: u16, quantity: u16) -> String` — deduct money, add to bag
  - `sell_item(&mut self, item_id: u16, quantity: u16) -> String` — add money, remove from bag
  - `get_inventory(&self) -> String` — JSON of bag contents
  - `get_party(&self) -> String` — JSON of party summaries
  - `get_player_info(&self) -> String` — JSON of name, money, time, stamps
  - `get_sneakerdex(&self) -> String` — JSON of dex data
  - `heal_party(&mut self)` — restore all HP/PP/status
  - `deposit_sneaker(&mut self, party_index: u8) -> String`
  - `withdraw_sneaker(&mut self, box_index: u16) -> String`

**`engine/src/state/game_state.rs`**
- Ensure all fields have `Serialize, Deserialize`
- Add save version constant: `pub const SAVE_VERSION: u32 = 1;`

**`client/src/App.tsx`**
- Add title screen as initial state
- Route: TitleScreen → (New Game → game) or (Continue → load → game)
- Escape key toggles PauseMenu in overworld mode
- Menu takes input priority

## Tests Required

```rust
#[cfg(test)]
mod tests_phase_7 {
    // Save/Load
    - export_save produces valid JSON
    - load_save(export_save()) produces identical state
    - Player position preserved through save/load
    - Party preserved through save/load
    - Inventory preserved through save/load
    - Event flags preserved through save/load

    // Inventory operations
    - buy_item deducts money and adds item
    - buy_item fails with insufficient money (returns error)
    - sell_item adds money and removes item
    - use_item(Sole Sauce) heals 20 HP
    - use_item(Full Restore) heals to max
    - Can't use item on fainted sneaker (unless revive)

    // Heal party
    - All sneakers restored to max HP
    - All PP restored
    - All status conditions cleared

    // Sneaker Box
    - Deposit moves sneaker from party to box
    - Withdraw moves sneaker from box to party
    - Can't deposit last party member
    - Can't withdraw when party is full (6)
    - Can't deposit when box is full (50)

    // Sneakerdex
    - New game: all entries unseen
    - After encountering: entry marked seen
    - After catching: entry marked caught
    - get_sneakerdex returns correct counts
}
```

## Verification
```bash
cd engine && cargo test tests_phase_7 && cd .. && ./verify.sh
```

## Acceptance Criteria
- [ ] Pause menu opens/closes with Escape
- [ ] All 6 tabs navigable (Party, Bag, Dex, Player, Options, Save)
- [ ] Items usable in overworld (heals work)
- [ ] Shop buy/sell functional with money deduction
- [ ] Save to slot persists to localStorage
- [ ] Load from slot restores game state
- [ ] Autosave triggers (on flag — implementation can be simple)
- [ ] Title screen with New Game / Continue
- [ ] Sneakerdex shows seen/caught states
- [ ] Sneaker Box deposit/withdraw works
- [ ] Sneaker Clinic heals party
- [ ] Save file under 32KB
- [ ] `./verify.sh` exits 0
