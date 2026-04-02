# UI/UX Design

## Screen Flow

```
[Launch] → [Title Screen] → [New Game / Continue / Options]
                                  │              │
                                  ▼              ▼
                            [Name Entry]    [Load Slot]
                                  │              │
                                  └──────┬───────┘
                                         ▼
                                   [Game World]
                                    │    │    │
                          ┌─────────┘    │    └──────────┐
                          ▼              ▼               ▼
                    [Pause Menu]   [Battle Screen]  [Dialogue]
                     │ │ │ │
          ┌──────────┘ │ │ └──────────┐
          ▼            ▼ ▼            ▼
    [Party Screen] [Bag] [Dex]  [Save Screen]
```

## Title Screen

```
┌──────────────────────────────────────────┐
│                                          │
│                                          │
│         ░░░ WIZSNEAKERS ░░░              │
│         [Animated sneaker logo]          │
│                                          │
│                                          │
│           ► New Game                     │
│             Continue                     │
│             Options                      │
│                                          │
│                                          │
│     © 2026  —  Press Start              │
└──────────────────────────────────────────┘
```

- Logo: Pixel art sneaker with "WIZSNEAKERS" in stylized font
- Background: Slowly scrolling sneaker silhouettes
- Music: Upbeat chiptune theme

## HUD (Overworld)

Minimal — only appears when relevant:

```
┌──────────────────────────────────────────┐
│ [Boxfresh Town]              [$DD: 1,250]│  ← Fades in on area enter / menu button
│                                          │
│            [Game World]                  │
│                                          │
│                                          │
│                              [A] Interact│  ← Contextual prompt near NPCs/signs
│                              [M] Menu    │
└──────────────────────────────────────────┘
```

## Dialogue Box

```
┌──────────────────────────────────────────┐
│                                          │
│            [Game World]                  │
│                                          │
│  ┌──────────────────────────────────┐    │
│  │ [NPC Portrait]                    │    │
│  │  PROF. SOLE: Welcome to the       │    │
│  │  world of Wizsneakers! Are you    │    │
│  │  ready to choose your first pair? │    │
│  │                            [▼]    │    │
│  └──────────────────────────────────┘    │
└──────────────────────────────────────────┘
```

- Text appears character-by-character (typewriter effect)
- Speed: Slow (30ms/char), Medium (15ms/char), Fast (5ms/char), Instant
- Press action button to advance or skip to end of current text
- [▼] indicator bounces when waiting for input
- Choices appear as selectable options below text when needed

### Choice Dialogue

```
┌──────────────────────────────────┐
│  PROF. SOLE: Which pair calls     │
│  to you?                         │
│                                  │
│  ► Retro Runner  (Retro)        │
│    Tech Trainer  (Techwear)     │
│    Skate Blazer  (Skate)        │
└──────────────────────────────────┘
```

## Pause Menu

```
┌──────────────────────────────────────────┐
│  ┌────────────────┐                      │
│  │   WIZSNEAKERS   │                      │
│  ├────────────────┤                      │
│  │ ► Sneakers      │   [Lead Sneaker     │
│  │   Bag           │    Summary Card]     │
│  │   Sneakerdex    │                      │
│  │   Player Card   │   Name: Retro Runner │
│  │   Options       │   Lv. 24             │
│  │   Save          │   HP: 58/72          │
│  │   Quit          │   Retro type         │
│  └────────────────┘                      │
└──────────────────────────────────────────┘
```

## Party Screen

```
┌──────────────────────────────────────────┐
│  YOUR SNEAKER ROTATION                   │
│  ┌─────────────────────────────────────┐ │
│  │ ►[Icon] Retro Runner    Lv.24      │ │
│  │         HP: ████████░░  58/72      │ │
│  ├─────────────────────────────────────┤ │
│  │  [Icon] Tech Trainer    Lv.22      │ │
│  │         HP: ██████████  65/65      │ │
│  ├─────────────────────────────────────┤ │
│  │  [Icon] Grip Tape       Lv.18      │ │
│  │         HP: ██████░░░░  32/50      │ │
│  ├─────────────────────────────────────┤ │
│  │  [Empty Slot]                       │ │
│  ├─────────────────────────────────────┤ │
│  │  [Empty Slot]                       │ │
│  ├─────────────────────────────────────┤ │
│  │  [Empty Slot]                       │ │
│  └─────────────────────────────────────┘ │
│  [A] Select  [B] Back  [Y] Swap         │
└──────────────────────────────────────────┘
```

Selecting a sneaker opens its detail view:

```
┌──────────────────────────────────────────┐
│  RETRO RUNNER          Retro │ ★★ Uncomm │
│  ┌──────────┐                            │
│  │ [64x64   │  Lv. 24    Condition: Beat │
│  │  Sprite]  │  XP: ██████░░  1240/2000  │
│  └──────────┘                            │
│  ──────────────────────────────────────  │
│  DUR: 58/72   HYP: 45    CMF: 38        │
│  DRP: 32      RAR: 41                    │
│  ──────────────────────────────────────  │
│  MOVES:                                  │
│  1. Crease        [Retro]  PP: 20/25     │
│  2. Throwback     [Retro]  PP: 18/20     │
│  3. Lace Up       [Normal] PP: 15/20     │
│  4. Stomp         [Normal] PP: 12/20     │
│  ──────────────────────────────────────  │
│  Held Item: Heritage Sole                │
│  OT: Player  │  Met: Route 2, Lv.8      │
└──────────────────────────────────────────┘
```

## Battle UI

### Main Battle Screen
```
┌──────────────────────────────────────────┐
│                                          │
│  ┌─ CLASSIC DUNK ─────┐                 │
│  │ Lv.15              │   ░░░            │
│  │ HP: ████████░░░    │  ░░░░░           │
│  └────────────────────┘   ░░░            │
│                          (opponent)       │
│                                          │
│       ░░░         ┌─ RETRO RUNNER ─────┐ │
│      ░░░░░        │ Lv.14              │ │
│       ░░░         │ HP: ██████████     │ │
│     (player)      │ XP: ████░░░░░░    │ │
│                   └────────────────────┘ │
│                                          │
│  ┌──────────────────────────────────────┐│
│  │  What will Retro Runner do?          ││
│  │  ┌─────────┐  ┌─────────┐           ││
│  │  │ ► FIGHT │  │   BAG   │           ││
│  │  ├─────────┤  ├─────────┤           ││
│  │  │ SNEAKERS│  │   RUN   │           ││
│  │  └─────────┘  └─────────┘           ││
│  └──────────────────────────────────────┘│
└──────────────────────────────────────────┘
```

### Move Selection
```
┌──────────────────────────────────────────┐
│  ┌───────────────┐  ┌───────────────┐    │
│  │► Crease       │  │  Throwback    │    │
│  │  Retro PP:20  │  │  Retro PP:18  │    │
│  ├───────────────┤  ├───────────────┤    │
│  │  Lace Up      │  │  Stomp        │    │
│  │  Norm  PP:15  │  │  Norm  PP:12  │    │
│  └───────────────┘  └───────────────┘    │
│                                          │
│  [Move Info] Crease — Power: 40          │
│  Acc: 100  Type: Physical                │
│  "Bend the toe box. Classic damage."     │
└──────────────────────────────────────────┘
```

### Battle Log Messages

Messages appear one at a time, player advances with action button:

```
"Retro Runner used Crease!"
"It's super effective!"                    ← Green text
"Classic Dunk lost 24 HP!"
"Classic Dunk used Stomp!"
"It's not very effective..."               ← Gray text
"Retro Runner lost 8 HP!"
```

### Capture Sequence
```
Frame 1: "You threw a Sneaker Case!"
Frame 2: [Case shakes once]           "..."
Frame 3: [Case shakes twice]          "..."
Frame 4: [Case shakes three times]    "..."
Frame 5a: [Stars burst] "Gotcha! Classic Dunk was caught!"
Frame 5b: [Case breaks] "Oh no! It broke free!"
```

## Sneakerdex Screen

```
┌──────────────────────────────────────────┐
│  SNEAKERDEX              Caught: 12/30   │
│  ┌──────────────────────────────────────┐│
│  │ #001 [Icon] Retro Runner    ✓ CAUGHT ││
│  │ #002 [Icon] Retro Runner II ✓ CAUGHT ││
│  │ #003 [????] ???             — UNSEEN ││
│  │ #004 [Silh] Classic Dunk    ○ SEEN   ││
│  │ #005 [Icon] OG Force        ✓ CAUGHT ││
│  │ #006 [????] ???             — UNSEEN ││
│  │ ...                                  ││
│  └──────────────────────────────────────┘│
│  [Filter: All | Retro | Tech | Skate | HF]│
└──────────────────────────────────────────┘
```

## Shop Screen

```
┌──────────────────────────────────────────┐
│  RETRO RICK'S VINTAGE SHOP               │
│  "Only the classics, kid."               │
│  ──────────────────────────────────────  │
│  ┌──────────────────────────┐  Your $DD  │
│  │ ► Sole Sauce       $200  │   $1,250   │
│  │   Insole Pad       $500  │            │
│  │   Crease Guard     $300  │  ┌───────┐ │
│  │   Sneaker Case     $200  │  │[Item  ]│ │
│  │   Vintage Polish  $1000  │  │[Info  ]│ │
│  │   Heritage Sole   $2000  │  │[Panel ]│ │
│  │   ─ EXIT ─               │  └───────┘ │
│  └──────────────────────────┘            │
│  [A] Buy  [B] Back                       │
└──────────────────────────────────────────┘
```

## Accessibility

- All text meets 4.5:1 contrast ratio against backgrounds
- Color-coded elements (type effectiveness) also have text labels
- Keyboard-only navigation (no mouse required)
- Text speed options including "Instant"
- Screen shake can be disabled in Options
- Battle animations can be set to "Fast" or "Skip"
