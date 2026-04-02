# Sound Design

## Music Direction

The soundtrack blends **chiptune** (8-bit/16-bit synths) with **lo-fi hip-hop** and **trap beats**. Think: if a GBA Pokemon soundtrack was produced by a SoundCloud beatmaker who loves sneakers.

## Background Music Tracks

### Required Tracks (MVP)

| Track ID | Name | Where | BPM | Mood | Duration |
|----------|------|-------|-----|------|----------|
| bgm_title | "Fresh Out the Box" | Title screen | 90 | Chill, inviting | 1:30 loop |
| bgm_boxfresh | "Home Turf" | Boxfresh Town | 95 | Warm, nostalgic | 1:00 loop |
| bgm_route_easy | "Step by Step" | Routes 1-2 | 110 | Light, adventurous | 1:15 loop |
| bgm_route_mid | "On the Move" | Routes 3-5 | 120 | Energetic, determined | 1:15 loop |
| bgm_route_hard | "Final Stretch" | Routes 6-8 | 130 | Intense, driven | 1:15 loop |
| bgm_laceup | "Vintage Vibes" | Lace-Up Village | 85 | Mellow, retro | 1:00 loop |
| bgm_kicksburg | "Concrete Wave" | Kicksburg | 115 | Gritty, skatepunk | 1:00 loop |
| bgm_neonsprings | "Digital Drip" | Neon Springs | 128 | Electronic, clean | 1:00 loop |
| bgm_hypetown | "The Drop" | Hypetown | 105 | Bustling, exciting | 1:30 loop |
| bgm_grailheim | "Sacred Sole" | Grailheim | 75 | Mysterious, reverent | 1:00 loop |
| bgm_resellrow | "Hustle" | Resell Row | 135 | Tense, shady | 1:00 loop |
| bgm_battle_wild | "Lace Up!" | Wild encounters | 140 | Pumping, fun | 1:00 loop |
| bgm_battle_trainer | "Step to This" | Trainer battles | 150 | Competitive, driving | 1:00 loop |
| bgm_battle_boss | "Sole Showdown" | Boss battles | 160 | Epic, high stakes | 1:30 loop |
| bgm_battle_elite | "No Cap" | Elite Resellers | 155 | Intense, dramatic | 1:30 loop |
| bgm_battle_champion | "Genesis" | Champion battle | 145 | Grand, climactic | 2:00 loop |
| bgm_victory | "W" | Battle won | 120 | Triumphant, short | 0:15 (no loop) |
| bgm_defeat | "Took the L" | Battle lost | 80 | Somber, short | 0:10 (no loop) |
| bgm_evolution | "Glow Up" | Evolution scene | 130 | Building, transformative | 0:20 (no loop) |
| bgm_sneaker_clinic | "Restore" | Sneaker Clinic interior | 90 | Calm, clean | 0:45 loop |
| bgm_shop | "Cop or Drop" | Shop interior | 100 | Upbeat, retail | 0:45 loop |
| bgm_credits | "Sole Legacy" | End credits | 100 | Reflective, epic | 3:00 |
| bgm_pinnacle | "Ascent" | Pinnacle Tower | 140 | Ominous, escalating | 1:15 loop |

## Sound Effects

### Movement & Overworld

| SFX ID | Description | Trigger |
|--------|-------------|---------|
| sfx_step_grass | Soft footstep on grass | Walking on grass tiles |
| sfx_step_path | Hard footstep on pavement | Walking on path tiles |
| sfx_step_indoor | Indoor footstep | Walking on interior floors |
| sfx_step_tallgrass | Rustling grass | Entering tall grass |
| sfx_bump | Soft thud | Walking into a wall |
| sfx_door_open | Door creak | Entering a building |
| sfx_door_close | Door shut | Exiting a building |
| sfx_menu_open | Quick whoosh | Opening pause menu |
| sfx_menu_close | Reverse whoosh | Closing pause menu |
| sfx_cursor_move | Soft click | Moving cursor in menus |
| sfx_cursor_select | Confirm beep | Selecting a menu option |
| sfx_cursor_cancel | Soft decline tone | Pressing cancel |
| sfx_item_pickup | Sparkle chime | Finding an item on the map |
| sfx_text_char | Soft tick | Each character appearing in dialogue |

### Battle

| SFX ID | Description | Trigger |
|--------|-------------|---------|
| sfx_encounter_wild | Dramatic sting + whoosh | Wild encounter starts |
| sfx_encounter_trainer | Challenge horn | Trainer spots you |
| sfx_battle_start | Sneaker swoosh | Battle transition |
| sfx_attack_physical | Impact thud | Physical move hits |
| sfx_attack_special | Energy blast | Special move hits |
| sfx_attack_miss | Whiff sound | Move misses |
| sfx_super_effective | High-pitched sting | Super effective hit |
| sfx_not_effective | Low dull thud | Not very effective hit |
| sfx_critical_hit | Sharp crack | Critical hit |
| sfx_stat_up | Rising chime | Stat boosted |
| sfx_stat_down | Falling tone | Stat lowered |
| sfx_status_applied | Negative buzz | Status condition applied |
| sfx_heal | Sparkle ascending | HP healed |
| sfx_faint | Descending tone | Sneaker faints |
| sfx_switch | Quick swap sound | Switching sneakers |
| sfx_xp_gain | Filling meter sound | XP bar filling |
| sfx_level_up | Triumphant jingle | Level up |
| sfx_case_throw | Whoosh | Throwing sneaker case |
| sfx_case_shake | Rattling | Case shakes |
| sfx_case_catch | Click + chime | Successful capture |
| sfx_case_break | Pop + escape | Failed capture |
| sfx_flee | Running sound | Fleeing battle |
| sfx_flee_fail | Blocked sound | Failed to flee |

### UI & Special

| SFX ID | Description | Trigger |
|--------|-------------|---------|
| sfx_save | Writing/ding | Game saved |
| sfx_stamp_earned | Stamp press + fanfare | Authentication stamp earned |
| sfx_evolution_start | Building energy | Evolution sequence begins |
| sfx_evolution_complete | Burst of energy | Evolution completes |
| sfx_sneakerdex_entry | Entry registered tone | New Sneakerdex entry |
| sfx_money | Cash register ching | Receiving $DD |
| sfx_error | Buzz/deny | Invalid action |

## Audio Technical Specs

### Format
- **Music**: OGG Vorbis, 128kbps, stereo
- **SFX**: WAV, 16-bit, 44.1kHz, mono (small file sizes for instant playback)

### Estimated File Sizes
- Music tracks: ~1-3 MB each (23 tracks = ~40 MB total)
- SFX: ~5-50 KB each (45 sounds = ~1 MB total)
- **Total audio budget: ~41 MB** (lazy-loaded, not all at once)

### Playback Rules
- Only one BGM track plays at a time
- BGM crossfades when changing (500ms fade out, 200ms silence, 500ms fade in)
- Up to 4 SFX can play simultaneously
- SFX never interrupt BGM
- Player can independently control BGM and SFX volume (0-100%)
- Mute toggle available

### Music Transition Map

```
Overworld area → Area BGM (fade)
Overworld → Battle encounter → sfx_encounter + fade to battle BGM
Battle → Victory → sfx + bgm_victory → fade back to area BGM
Battle → Defeat → sfx + bgm_defeat → fade to Sneaker Clinic BGM
Enter building → Fade to building BGM (if different)
Exit building → Fade back to area BGM
Boss defeated → bgm_victory → sfx_stamp_earned → resume area BGM
Evolution → bgm_evolution → resume previous BGM
```
