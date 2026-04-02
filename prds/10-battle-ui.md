# PRD 10 — Battle UI: Full React Battle Screen (Phase 4)

## Goal
Replace the debug battle overlay with a polished battle screen. HP bars with smooth animation, 2×2 menu grids, move selection with type colors, turn animation sequencing, capture animation, level-up/evolution/move-learning UIs. Battles should feel like a real RPG.

## Dependencies
- PRD 09 (complete battle engine)

## Deliverables

### Files to Create

**`client/src/types/game.ts`**
- TypeScript types mirroring Rust: `Faction`, `Direction`, `GameMode`, `Stats`, `BattleSide`
- Color mappings for factions: Retro=#c0392b, Techwear=#2980b9, Skate=#27ae60, HighFashion=#8e44ad, Normal=#95a5a6

**`client/src/types/battle.ts`**
- `SneakerSummary`: uid, species_id, name, level, current_hp, max_hp, faction, rarity_tier, status
- `BattleRenderState`: player_sneaker, opponent_sneaker, player_stages, opponent_stages, available_moves, can_flee, is_wild
- `MoveDisplay`: id, name, faction, category, power, accuracy, current_pp, max_pp
- `BattleTurnEvent`: union type matching all Rust event variants

**`client/src/components/battle/BattleScreen.tsx`**
- Top-level battle container
- States: `selecting_action` | `selecting_move` | `selecting_bag` | `selecting_party` | `animating` | `battle_end`
- Fetches `engine.get_battle_state()` for display data
- Layout (per spec art/03-ui-ux.md):
  - Top area: opponent sneaker info (top-left) + opponent placeholder sprite (center-right)
  - Middle: player placeholder sprite (center-left) + player sneaker info (bottom-right)
  - Bottom: action area (menu or log)
- Background: faction-colored gradient based on opponent's type

**`client/src/components/battle/BattleHUD.tsx`**
- Props: `sneaker: SneakerSummary, side: 'player' | 'opponent', stages?: StatStages`
- Shows: name, level, HP bar, HP text (player only), XP bar (player only), status icon
- HP bar: CSS transition for smooth drain (500ms ease-out)
- HP color: green (>50%), yellow (25-50%), red (<25%)
- Stat stage indicators: small up/down arrows near the name

**`client/src/components/battle/BattleMenu.tsx`**
- 2×2 grid: FIGHT | BAG / SNEAKERS | RUN
- Keyboard navigation: arrow keys move selection, Z/Enter confirms, X/Escape cancels
- Selected item highlighted with border + slight scale
- RUN disabled and grayed out in trainer/boss battles

**`client/src/components/battle/MoveSelect.tsx`**
- 2×2 grid of the active sneaker's moves
- Each cell shows: move name, faction (color-coded background), PP (current/max)
- Moves with 0 PP are grayed out and unselectable
- On hover/select: show power, accuracy, category below
- X/Escape returns to main menu

**`client/src/components/battle/BattleLog.tsx`**
- Message box at bottom of screen
- Typewriter text effect (configurable speed)
- Press action to advance to next message
- Queue of messages processed sequentially

**`client/src/rendering/battleAnimations.ts`**
- `async function playTurnEvents(events: BattleTurnEvent[], callbacks: AnimationCallbacks): Promise<void>`
- Processes events sequentially with appropriate delays:
  - `MoveUsed`: show "[Name] used [Move]!" in log (300ms + typewriter)
  - `Damage`: animate HP bar drain (500ms), show effectiveness text if SE/NVE (400ms)
  - `StatChange`: "[Name]'s [Stat] rose/fell!" with arrow animation (300ms)
  - `StatusApplied`: "[Name] was [Status]!" with status icon (400ms)
  - `StatusDamage`: "[Name] is hurt by [Status]!" + HP drain (400ms)
  - `Healed`: HP bar fills + "[Name] healed!" (400ms)
  - `Fainted`: "[Name] fainted!" + slide-down effect (600ms)
  - `FleeAttempt`: "Got away safely!" or "Can't escape!" (400ms)
  - `CaptureAttempt`: delegate to CaptureAnimation
  - `XpGained`: XP bar fills with animation (300ms per segment)
  - `LevelUp`: "[Name] grew to Lv.[X]!" overlay (500ms)
  - `BattleEnd`: handle transition
  - `Message`: show in log
- Input disabled during animation sequence

**`client/src/components/battle/BattleAnimations.tsx`**
- Screen shake (CSS transform translateX oscillation, 200ms)
- Flash overlay (white/colored flash for super effective, 100ms)
- Stat change arrows (up=green arrow, down=red arrow, animated)

**`client/src/components/battle/CaptureAnimation.tsx`**
- Case flies to opponent position (300ms)
- Opponent disappears (fade out 200ms)
- Case wobbles 0-3 times (600ms per wobble, CSS keyframe)
- Success: click sound + stars, "Gotcha! [Name] was caught!" (500ms)
- Fail: case breaks open, opponent reappears (400ms)

**`client/src/components/battle/LevelUpOverlay.tsx`**
- "[Name] grew to Lv.[X]!" text
- Stat comparison: old stats → new stats with +delta highlighted in green

**`client/src/components/battle/MoveLearnPrompt.tsx`**
- "[Name] wants to learn [Move]!"
- Show new move details (name, type, power, PP)
- "Choose a move to replace:" with 4 current moves listed
- Option to skip: "Give up learning [Move]?"
- Keyboard: 1-4 to replace, Escape to skip

**`client/src/components/battle/EvolutionScene.tsx`**
- "What? [Name] is evolving!"
- Name changes animation: old name → new name
- "Congratulations! [Name] evolved into [New Name]!"
- "Press B to cancel" prompt (X/Escape to cancel)

**`client/src/components/battle/BagScreen.tsx`**
- 5-pocket tabs at top (Heal / Battle / Cases / Key / Held)
- Item list: name, quantity, description on select
- Use button → select target sneaker (for heals) or use directly (for battle items/cases)
- Cases only shown in wild battles
- Keyboard: arrow keys navigate, Z to use, X to cancel back

**`client/src/components/battle/PartyScreen.tsx`**
- List of party sneakers (up to 6)
- Each shows: faction color strip, name, level, HP bar, status icon
- Select to switch. Fainted sneakers have "FAINTED" overlay and can't switch in.
- Active sneaker marked with "IN BATTLE" tag
- X/Escape to go back

### Files to Modify

**`client/src/App.tsx`**
- Remove the old debug battle overlay
- When `engine.mode() === "Battle"`: render `<BattleScreen>` component
- Add encounter transition: brief flash/overlay when transitioning from overworld to battle
- On battle end: transition back to overworld

## Tests Required

No Rust tests (this is React-only). Verification through:
1. `tsc --noEmit` passes (type correctness)
2. `vite build` succeeds (bundle works)
3. Manual: trigger wild encounter → full battle → win → return to overworld

## Verification
```bash
./verify.sh
```

## Acceptance Criteria
- [ ] Battle screen renders with HUD (HP bars, names, levels)
- [ ] HP bars animate smoothly with color transitions
- [ ] Move selection shows all 4 moves with type colors and PP
- [ ] Turn animations play sequentially (attack → damage → effects)
- [ ] Super effective / not very effective messages display
- [ ] Bag screen shows items, can use heals and cases
- [ ] Party screen shows team, can switch sneakers
- [ ] Capture animation with case wobble plays correctly
- [ ] Level-up overlay shows stat changes
- [ ] Move learning prompt works (replace or skip)
- [ ] Evolution scene works (accept or cancel)
- [ ] Flee works (returns to overworld)
- [ ] Keyboard navigation throughout all battle screens
- [ ] All components type-check (no TS errors)
- [ ] `./verify.sh` exits 0
