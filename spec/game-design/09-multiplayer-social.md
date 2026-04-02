# Multiplayer & Social (Post-MVP)

> This document outlines features planned for after the single-player MVP launch.

## Trading System

### Direct Trading
- Two players connect via a room code
- Each selects up to 3 sneakers to offer
- Both must confirm before trade executes
- Traded sneakers retain original trainer name and caught location
- Cannot trade Key Items or Genesis Grails

### GTS (Global Trade Station)
- List a sneaker with a request ("Want: any Techwear, Lv.20+")
- Other players can browse and fulfill trades asynchronously
- Listings expire after 7 days

## PvP Battles

### Casual Battle
- Room code matchmaking
- Bring your story team as-is (any level, any items)
- No rewards, just bragging rights

### Ranked Battle
- Matchmaking based on ELO rating
- All sneakers auto-leveled to 50
- Standard competitive rules:
  - No duplicate sneakers
  - No duplicate held items
  - No Genesis Grails (legendary ban)
  - Species clause (one of each sneaker)
  - Sleep clause (only one opponent can be asleep at a time)
- Seasons last 1 month, rewards based on final rank

### Ranked Tiers
| Tier | ELO Range | Season Reward |
|------|-----------|---------------|
| Bronze | 0-999 | 1,000 $DD |
| Silver | 1000-1499 | 5,000 $DD + exclusive colorway |
| Gold | 1500-1899 | 10,000 $DD + exclusive sneaker |
| Diamond | 1900-2199 | 25,000 $DD + title + exclusive sneaker |
| Grail | 2200+ | 50,000 $DD + animated title + trophy |

## Leaderboards

- Top 100 Ranked players displayed globally
- Filterable by region/platform
- Show team composition (encourages meta diversity)

## Social Features

### Player Profile
- Display name, title, favorite sneaker, win/loss record
- Showcase: Pin 3 sneakers to show off on profile
- Achievement badges displayed

### Friend List
- Add friends via code
- See online status
- Quick-challenge to battles or trades

## Technical Considerations

- PvP requires WebSocket server (not WASM — server-authoritative)
- Battle state validation on server to prevent cheating
- Trade validation: ensure both parties have the offered sneakers
- Rate limiting on GTS to prevent spam
- Anti-cheat: server validates sneaker stats against legitimate ranges
