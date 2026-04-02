#!/bin/bash
# Wizsneakers — Master Verification Script
# Run this after each PRD phase to confirm everything works.
# Exit code 0 = all good, non-zero = something failed.

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0

check() {
    local name="$1"
    shift
    echo -n "  [$name] "
    if "$@" > /tmp/verify_output.txt 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        PASS=$((PASS + 1))
    else
        echo -e "${RED}FAIL${NC}"
        cat /tmp/verify_output.txt | tail -20
        FAIL=$((FAIL + 1))
    fi
}

echo ""
echo "========================================="
echo "  WIZSNEAKERS VERIFICATION"
echo "========================================="
echo ""

# --- Stage 1: Rust Engine ---
echo "▸ Rust Engine"
check "cargo check" cargo check --manifest-path engine/Cargo.toml
check "cargo test" cargo test --manifest-path engine/Cargo.toml
check "wasm-pack build" wasm-pack build engine --target web --out-dir ../client/src/wasm

# --- Stage 2: TypeScript ---
echo ""
echo "▸ TypeScript Client"
check "tsc type-check" bash -c "cd client && npx tsc --noEmit"

# --- Stage 3: Production Build ---
echo ""
echo "▸ Production Build"
check "vite build" bash -c "cd client && npx vite build"

# --- Stage 4: Bundle Size ---
echo ""
echo "▸ Bundle Size Checks"
WASM_SIZE=$(wc -c < client/src/wasm/wizsneakers_engine_bg.wasm 2>/dev/null || echo "0")
WASM_GZ=$(gzip -c client/src/wasm/wizsneakers_engine_bg.wasm 2>/dev/null | wc -c || echo "0")
echo "  WASM raw: ${WASM_SIZE} bytes, gzipped: ${WASM_GZ} bytes"
if [ "$WASM_GZ" -lt 204800 ]; then
    echo -e "  [WASM <200KB gzip] ${GREEN}PASS${NC} ($(( WASM_GZ / 1024 ))KB)"
    PASS=$((PASS + 1))
else
    echo -e "  [WASM <200KB gzip] ${RED}FAIL${NC} ($(( WASM_GZ / 1024 ))KB)"
    FAIL=$((FAIL + 1))
fi

# --- Summary ---
echo ""
echo "========================================="
TOTAL=$((PASS + FAIL))
if [ "$FAIL" -eq 0 ]; then
    echo -e "  ${GREEN}ALL $TOTAL CHECKS PASSED${NC}"
else
    echo -e "  ${RED}$FAIL/$TOTAL CHECKS FAILED${NC}"
fi
echo "========================================="
echo ""

exit $FAIL
