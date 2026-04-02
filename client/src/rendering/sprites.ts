// Procedural pixel art generation — Stardew Valley-inspired tiles + character sprites
// All art is drawn at 16px base resolution; the renderer scales up with imageSmoothingEnabled=false.

import { TILE_SIZE } from './camera';

type Ctx = CanvasRenderingContext2D;

// ────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────

function makeCanvas(w: number, h: number): [HTMLCanvasElement, Ctx] {
  const c = document.createElement('canvas');
  c.width = w;
  c.height = h;
  const ctx = c.getContext('2d')!;
  ctx.imageSmoothingEnabled = false;
  return [c, ctx];
}

function px(ctx: Ctx, x: number, y: number, c: string) {
  ctx.fillStyle = c;
  ctx.fillRect(x, y, 1, 1);
}

function rect(ctx: Ctx, x: number, y: number, w: number, h: number, c: string) {
  ctx.fillStyle = c;
  ctx.fillRect(x, y, w, h);
}

function hashXY(x: number, y: number): number {
  let h = (x * 374761393 + y * 668265263) | 0;
  h = ((h ^ (h >> 13)) * 1274126177) | 0;
  return (h ^ (h >> 16)) >>> 0;
}

// ────────────────────────────────────────────────────────────
// Tile Drawing (16×16 each)
// ────────────────────────────────────────────────────────────

const T = TILE_SIZE;

// -- Grass (4 variants) --
function drawGrass(ctx: Ctx, variant: number) {
  rect(ctx, 0, 0, T, T, '#5ea845');

  // Scattered light/dark pixels for texture
  const spots = [
    [1, 1], [3, 5], [5, 2], [7, 9], [9, 1], [11, 6], [13, 3], [15, 10],
    [2, 8], [4, 12], [6, 7], [8, 4], [10, 11], [12, 0], [14, 9], [0, 14],
    [3, 13], [7, 3], [11, 8], [15, 5], [1, 10], [5, 15], [9, 6], [13, 12],
  ];
  for (const [sx, sy] of spots) {
    px(ctx, sx, sy, (sx + sy) % 3 === 0 ? '#70bc55' : '#4c9635');
  }

  // Extra subtle texture
  px(ctx, 0, 0, '#52983d'); px(ctx, 8, 8, '#52983d');
  px(ctx, 4, 4, '#68b450'); px(ctx, 12, 12, '#68b450');

  if (variant === 1) {
    // Yellow flower
    px(ctx, 7, 5, '#f8e060'); px(ctx, 8, 5, '#f8d848');
    px(ctx, 7, 6, '#f8d848'); px(ctx, 8, 6, '#e8a030');
    px(ctx, 7, 4, '#70bc55'); // stem
  } else if (variant === 2) {
    // Pink flower
    px(ctx, 10, 8, '#f06888'); px(ctx, 11, 8, '#f06888');
    px(ctx, 10, 9, '#e05878'); px(ctx, 11, 9, '#f06888');
    px(ctx, 10, 7, '#70bc55');
  } else if (variant === 3) {
    // Purple flower
    px(ctx, 4, 10, '#9878e0'); px(ctx, 5, 10, '#8868d0');
    px(ctx, 4, 11, '#8868d0'); px(ctx, 5, 11, '#7858c0');
    px(ctx, 4, 9, '#70bc55');
  }
}

// -- Wall / Building (2 variants) --
function drawWall(ctx: Ctx, variant: number) {
  const mortar = variant === 0 ? '#c8c0b0' : '#c0b8a8';
  const brick = variant === 0 ? '#a08060' : '#988068';
  const brickLt = variant === 0 ? '#b89070' : '#a89078';
  const brickDk = variant === 0 ? '#8a6a4a' : '#806858';

  rect(ctx, 0, 0, T, T, mortar);

  for (let row = 0; row < 4; row++) {
    const by = row * 4;
    const offset = (row % 2) * 4;
    for (let bx = -4 + offset; bx < T; bx += 8) {
      const x1 = Math.max(0, bx);
      const x2 = Math.min(T, bx + 7);
      if (x1 >= x2) continue;
      rect(ctx, x1, by, x2 - x1, 3, brick);
      rect(ctx, x1, by, x2 - x1, 1, brickLt);
      rect(ctx, x1, by + 2, x2 - x1, 1, brickDk);
    }
  }
}

// -- Tall Grass (2 variants) --
function drawTallGrass(ctx: Ctx, variant: number) {
  // Ground base
  drawGrass(ctx, 0);

  const positions = variant === 0 ? [2, 5, 8, 11, 14] : [1, 4, 7, 10, 13];
  const colors = ['#3a8828', '#4a9838', '#5aac48', '#7acc68'];

  for (const bx of positions) {
    const height = 8 + (bx * 3) % 5;
    for (let y = T - 1; y >= T - height; y--) {
      const t = (T - 1 - y) / height;
      const ci = Math.min(3, Math.floor(t * 4));
      px(ctx, bx, y, colors[ci]);
      if (y > T - height * 0.6) px(ctx, bx + 1, y, colors[Math.max(0, ci - 1)]);
    }
    // Bright tip
    px(ctx, bx, T - height, '#8acc68');
  }
}

// -- Door --
function drawDoor(ctx: Ctx) {
  rect(ctx, 0, 0, T, T, '#3a2818');        // frame
  rect(ctx, 2, 1, 12, 14, '#8b6842');      // panel
  rect(ctx, 3, 2, 10, 5, '#7a5832');       // upper inset
  rect(ctx, 3, 2, 10, 1, '#9a7852');       // upper trim
  rect(ctx, 3, 9, 10, 5, '#7a5832');       // lower inset
  for (let y = 3; y < 14; y += 3) {        // wood grain
    rect(ctx, 3, y, 10, 1, '#9a7852');
  }
  rect(ctx, 10, 7, 2, 2, '#c8a860');       // handle
  px(ctx, 10, 7, '#e8c880');               // handle highlight
  rect(ctx, 0, 15, T, 1, '#6a5a4a');       // threshold
}

// ────────────────────────────────────────────────────────────
// Tile Atlas
// ────────────────────────────────────────────────────────────
//
// Row 0: grass_0  grass_1  grass_2  grass_3
// Row 1: wall_0   wall_1   tgrass_0 tgrass_1
// Row 2: door     (reserved)

export function generateTileAtlas(): HTMLCanvasElement {
  const cols = 4, rows = 3;
  const [canvas, ctx] = makeCanvas(cols * T, rows * T);

  const at = (c: number, r: number, fn: () => void) => {
    ctx.save(); ctx.translate(c * T, r * T); fn(); ctx.restore();
  };

  for (let i = 0; i < 4; i++) at(i, 0, () => drawGrass(ctx, i));
  at(0, 1, () => drawWall(ctx, 0));
  at(1, 1, () => drawWall(ctx, 1));
  at(2, 1, () => drawTallGrass(ctx, 0));
  at(3, 1, () => drawTallGrass(ctx, 1));
  at(0, 2, () => drawDoor(ctx));

  return canvas;
}

export function getTileSrc(
  tileType: number, x: number, y: number,
): { sx: number; sy: number } {
  const h = hashXY(x, y);
  switch (tileType) {
    case 0: return { sx: (h % 4) * T, sy: 0 };
    case 1: return { sx: (h % 2) * T, sy: T };
    case 2: return { sx: (2 + (h % 2)) * T, sy: T };
    case 3: return { sx: 0, sy: 2 * T };
    default: return { sx: 0, sy: 0 };
  }
}

// ────────────────────────────────────────────────────────────
// Character Spritesheet  (16 × 24 per frame)
// ────────────────────────────────────────────────────────────
//
// 3 cols (stand · walk1 · walk2) × 4 rows (down · up · left · right)

export const CHAR_W = 16;
export const CHAR_H = 24;

// Palette
const HAIR    = '#2a1a10';
const SKIN    = '#e8b888';
const EYE     = '#1a1a2a';
const MOUTH   = '#c88060';
const JACKET  = '#404050';
const JACK_LT = '#505060';
const PANTS   = '#384878';
const SHOE    = '#ee3322';
const SHOE_W  = '#ffffff';
const SOLE    = '#222222';

function drawShadow(ctx: Ctx) {
  const s = 'rgba(0,0,0,0.18)';
  rect(ctx, 4, 20, 8, 1, s);
  rect(ctx, 3, 21, 10, 1, s);
  rect(ctx, 4, 22, 8, 1, s);
}

// ── shared lower-body helper ──
function legs(
  ctx: Ctx, frame: number,
  lx: number, rx: number, legW: number, shoeW: number,
) {
  if (frame === 0) {
    rect(ctx, lx, 15, legW, 2, PANTS);
    rect(ctx, rx, 15, legW, 2, PANTS);
    rect(ctx, lx - 1, 17, shoeW, 2, SHOE);
    rect(ctx, rx, 17, shoeW, 2, SHOE);
    px(ctx, lx, 17, SHOE_W); px(ctx, rx + 1, 17, SHOE_W);
    rect(ctx, lx - 1, 19, shoeW, 1, SOLE);
    rect(ctx, rx, 19, shoeW, 1, SOLE);
  } else if (frame === 1) {
    // left leg forward (lower), right leg back (higher)
    rect(ctx, lx, 15, legW, 2, PANTS);
    rect(ctx, rx, 15, legW, 1, PANTS);
    rect(ctx, lx - 1, 17, shoeW, 2, SHOE);
    px(ctx, lx, 17, SHOE_W);
    rect(ctx, lx - 1, 19, shoeW, 1, SOLE);
    rect(ctx, rx, 16, shoeW, 2, SHOE);
    px(ctx, rx + 1, 16, SHOE_W);
    rect(ctx, rx, 18, shoeW, 1, SOLE);
  } else {
    // right leg forward, left leg back
    rect(ctx, lx, 15, legW, 1, PANTS);
    rect(ctx, rx, 15, legW, 2, PANTS);
    rect(ctx, lx - 1, 16, shoeW, 2, SHOE);
    px(ctx, lx, 16, SHOE_W);
    rect(ctx, lx - 1, 18, shoeW, 1, SOLE);
    rect(ctx, rx, 17, shoeW, 2, SHOE);
    px(ctx, rx + 1, 17, SHOE_W);
    rect(ctx, rx, 19, shoeW, 1, SOLE);
  }
}

// ── DOWN ──
function drawCharDown(ctx: Ctx, frame: number) {
  drawShadow(ctx);

  // Hair
  rect(ctx, 5, 2, 6, 1, HAIR);
  rect(ctx, 4, 3, 8, 2, HAIR);

  // Face
  rect(ctx, 5, 5, 6, 3, SKIN);
  px(ctx, 6, 6, EYE); px(ctx, 9, 6, EYE);         // eyes
  px(ctx, 7, 7, MOUTH); px(ctx, 8, 7, MOUTH);      // mouth
  rect(ctx, 6, 8, 4, 1, SKIN);                      // chin

  // Jacket
  rect(ctx, 5, 9, 6, 1, JACKET);
  rect(ctx, 4, 10, 8, 3, JACKET);
  rect(ctx, 5, 13, 6, 1, JACKET);
  rect(ctx, 7, 10, 2, 2, JACK_LT);                  // zip highlight
  px(ctx, 3, 11, SKIN); px(ctx, 12, 11, SKIN);      // arms
  px(ctx, 3, 12, SKIN); px(ctx, 12, 12, SKIN);

  // Pants waist
  rect(ctx, 5, 14, 6, 1, PANTS);
  legs(ctx, frame, 5, 9, 2, 3);
}

// ── UP ──
function drawCharUp(ctx: Ctx, frame: number) {
  drawShadow(ctx);

  // Hair (back view — larger)
  rect(ctx, 5, 2, 6, 1, HAIR);
  rect(ctx, 4, 3, 8, 4, HAIR);

  // Jacket back
  rect(ctx, 5, 7, 6, 1, JACKET);
  rect(ctx, 4, 8, 8, 3, JACKET);
  rect(ctx, 5, 11, 6, 2, JACKET);
  rect(ctx, 5, 13, 6, 1, JACKET);
  px(ctx, 3, 9, SKIN); px(ctx, 12, 9, SKIN);
  px(ctx, 3, 10, SKIN); px(ctx, 12, 10, SKIN);

  rect(ctx, 5, 14, 6, 1, PANTS);
  legs(ctx, frame, 5, 9, 2, 3);
}

// ── LEFT ──
function drawCharLeft(ctx: Ctx, frame: number) {
  drawShadow(ctx);

  // Hair (profile)
  rect(ctx, 4, 2, 6, 1, HAIR);
  rect(ctx, 3, 3, 8, 2, HAIR);

  // Face (profile — left-facing)
  rect(ctx, 5, 5, 5, 3, SKIN);
  px(ctx, 5, 6, EYE);                               // one eye
  px(ctx, 5, 7, MOUTH);
  rect(ctx, 6, 8, 3, 1, SKIN);

  // Jacket
  rect(ctx, 5, 9, 5, 1, JACKET);
  rect(ctx, 4, 10, 7, 3, JACKET);
  rect(ctx, 5, 13, 5, 1, JACKET);
  px(ctx, 3, 11, SKIN); px(ctx, 3, 12, SKIN);       // front arm

  rect(ctx, 5, 14, 5, 1, PANTS);

  if (frame === 0) {
    rect(ctx, 5, 15, 4, 2, PANTS);
    rect(ctx, 4, 17, 5, 2, SHOE);
    px(ctx, 5, 17, SHOE_W); px(ctx, 6, 17, SHOE_W);
    rect(ctx, 4, 19, 5, 1, SOLE);
  } else if (frame === 1) {
    rect(ctx, 4, 15, 3, 2, PANTS);
    rect(ctx, 7, 15, 2, 2, PANTS);
    rect(ctx, 3, 17, 3, 2, SHOE);
    rect(ctx, 7, 17, 3, 2, SHOE);
    px(ctx, 4, 17, SHOE_W); px(ctx, 8, 17, SHOE_W);
    rect(ctx, 3, 19, 3, 1, SOLE);
    rect(ctx, 7, 19, 3, 1, SOLE);
  } else {
    rect(ctx, 5, 15, 2, 2, PANTS);
    rect(ctx, 8, 15, 2, 1, PANTS);
    rect(ctx, 4, 17, 3, 2, SHOE);
    rect(ctx, 7, 16, 3, 2, SHOE);
    px(ctx, 5, 17, SHOE_W); px(ctx, 8, 16, SHOE_W);
    rect(ctx, 4, 19, 3, 1, SOLE);
    rect(ctx, 7, 18, 3, 1, SOLE);
  }
}

// ── RIGHT (horizontal flip of LEFT) ──
function drawCharRight(ctx: Ctx, frame: number) {
  const [tmp, tmpCtx] = makeCanvas(CHAR_W, CHAR_H);
  drawCharLeft(tmpCtx, frame);
  ctx.save();
  ctx.translate(CHAR_W, 0);
  ctx.scale(-1, 1);
  ctx.drawImage(tmp, 0, 0);
  ctx.restore();
}

// ────────────────────────────────────────────────────────────
// Spritesheet Generation
// ────────────────────────────────────────────────────────────

const DRAW_FNS = [drawCharDown, drawCharUp, drawCharLeft, drawCharRight];

export function generateCharSheet(): HTMLCanvasElement {
  const [canvas, ctx] = makeCanvas(3 * CHAR_W, 4 * CHAR_H);
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 3; col++) {
      ctx.save();
      ctx.translate(col * CHAR_W, row * CHAR_H);
      DRAW_FNS[row](ctx, col);
      ctx.restore();
    }
  }
  return canvas;
}

const FACING_ROW: Record<string, number> = {
  down: 0, up: 1, left: 2, right: 3,
};

export function getCharSrc(
  facing: string, frame: number,
): { sx: number; sy: number } {
  const row = FACING_ROW[facing] ?? 0;
  return { sx: Math.min(frame, 2) * CHAR_W, sy: row * CHAR_H };
}
