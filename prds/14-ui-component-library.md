# PRD 14 — UI Component Library (Phase 9A)

## Goal
Extract all inline UI from App.tsx into a proper reusable component library. Every menu, HUD element, and overlay should be a standalone component with consistent styling, keyboard navigation, and pixel-art theming. This unblocks all future UI PRDs by providing building blocks.

## Dependencies
- PRD 10 (battle components exist), PRD 12 (menu components exist), PRD 13 (transition overlay exists)

## Deliverables

### Shared UI Primitives

**Create `client/src/components/ui/PixelBox.tsx`**
- Reusable container with pixel-art border (2px solid, inset shadow for 3D effect)
- Props: `variant: 'dialog' | 'menu' | 'hud' | 'tooltip'`, `padding`, `children`
- Color themes per variant: dialog=dark blue, menu=dark gray, hud=semi-transparent black, tooltip=cream

**Create `client/src/components/ui/PixelText.tsx`**
- Text component that enforces the game's pixel font styling
- Props: `size: 'sm' | 'md' | 'lg' | 'xl'`, `color`, `shadow` (text-shadow for readability)
- Sizes: sm=12px, md=16px, lg=20px, xl=28px

**Create `client/src/components/ui/SelectionList.tsx`**
- Generic keyboard-navigable list component
- Props: `items: { label, value, disabled?, description? }[]`, `columns: 1 | 2`, `onSelect`, `onCancel`, `selectedIndex`
- Arrow key navigation, Z/Enter to select, X/Escape to cancel
- Visual: highlighted row with cursor indicator (▶)
- Disabled items grayed out and skipped during navigation

**Create `client/src/components/ui/ProgressBar.tsx`**
- Animated bar for HP, XP, timers
- Props: `current`, `max`, `colorStops?: { threshold: number, color: string }[]`, `animationMs`, `height`, `showText`
- Default HP colors: green >50%, yellow 25-50%, red <25%
- CSS transition for smooth animation

**Create `client/src/components/ui/TypewriterText.tsx`**
- Typewriter text rendering for dialogue and battle log
- Props: `text`, `speed: 'slow' | 'medium' | 'fast' | 'instant'`, `onComplete`
- Speeds: slow=60ms, medium=30ms, fast=15ms, instant=0ms
- Press action key to instantly complete current text

**Create `client/src/components/ui/ConfirmDialog.tsx`**
- Yes/No prompt overlay
- Props: `message`, `onConfirm`, `onCancel`, `confirmLabel?`, `cancelLabel?`
- Keyboard: left/right to select, Z to confirm, X to cancel

**Create `client/src/components/ui/TabBar.tsx`**
- Horizontal tab navigation
- Props: `tabs: { label, key }[]`, `activeKey`, `onChange`
- Left/Right arrow keys to switch tabs, visual underline on active tab

**Create `client/src/components/ui/Notification.tsx`**
- Toast-style notification for "Item obtained!", "Game saved!", etc.
- Props: `message`, `duration`, `icon?`
- Slides in from top, auto-dismisses after duration

### Refactor Existing Components

**Refactor `client/src/components/battle/BattleHUD.tsx`**
- Use ProgressBar for HP and XP bars (replace any inline bar rendering)

**Refactor `client/src/components/battle/BattleMenu.tsx`**
- Use SelectionList with `columns: 2` for the 2×2 grid

**Refactor `client/src/components/battle/MoveSelect.tsx`**
- Use SelectionList with `columns: 2`, pass faction colors

**Refactor `client/src/components/battle/BattleLog.tsx`**
- Use TypewriterText for message rendering

**Refactor `client/src/components/menu/PauseMenu.tsx`**
- Use TabBar for tab navigation
- Use PixelBox for the menu container

**Refactor `client/src/components/menu/ShopScreen.tsx`**
- Use SelectionList for item lists, ConfirmDialog for purchase confirmation

**Refactor `client/src/components/menu/InventoryScreen.tsx`**
- Use TabBar for pocket tabs, SelectionList for item lists

### Styling

**Create `client/src/styles/theme.ts`**
- Centralized theme constants: colors (per faction, per UI variant), font sizes, spacing, border styles
- Faction colors: Retro=#c0392b, Techwear=#2980b9, Skate=#27ae60, HighFashion=#8e44ad, Normal=#95a5a6
- Export as typed object for use in styled components / inline styles

**Create `client/src/styles/global.css`** (or update existing)
- @font-face for pixel font (fallback to monospace)
- CSS reset for game UI (no default margins/padding inside game container)
- CSS variables matching theme.ts for use in plain CSS

### App.tsx Cleanup

**Modify `client/src/App.tsx`**
- Remove any remaining inline UI rendering (move to components)
- App.tsx should be a thin router: TitleScreen | Overworld+HUD | BattleScreen | PauseMenu overlay
- No direct DOM rendering of game UI elements — all delegated to components

## Tests Required

No Rust tests (React-only). Verification through:
1. `tsc --noEmit` passes
2. `vite build` succeeds
3. All existing functionality still works (battle, menus, overworld)

## Verification
```bash
./verify.sh
```

## Acceptance Criteria
- [ ] All UI primitives created and typed (PixelBox, PixelText, SelectionList, ProgressBar, TypewriterText, ConfirmDialog, TabBar, Notification)
- [ ] Theme constants centralized in theme.ts
- [ ] Battle components refactored to use shared primitives
- [ ] Menu components refactored to use shared primitives
- [ ] App.tsx is a thin router with no inline UI
- [ ] Keyboard navigation works consistently across all components
- [ ] `./verify.sh` exits 0
