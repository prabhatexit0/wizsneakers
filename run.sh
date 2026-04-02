#!/bin/bash
# Wizsneakers — Autonomous Build Runner
# Usage: ./run.sh
# This kicks off Claude Code to work through all PRDs sequentially.
# It commits and pushes after each completed PRD phase.
# Run it and walk away.

set -o pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

LOG_FILE="$PROJECT_DIR/build.log"
CURRENT_PRD_FILE="$PROJECT_DIR/.current_prd"

# PRDs in execution order
PRDS=(
  "01-engine-models"
  "02-game-data"
  "03-game-state"
  "04-map-engine"
  "05-camera-viewport"
  "06-smooth-movement"
  "07-battle-core"
  "08-battle-effects"
  "09-battle-progression"
  "10-battle-ui"
  "11-dialogue-npcs"
  "12-menus-save"
  "13-world-starter"
)

log() {
  echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

commit_and_push() {
  local msg="$1"
  cd "$PROJECT_DIR"

  # Stage all changes (except secrets/logs)
  git add -A
  git reset -- build.log .current_prd 2>/dev/null || true

  # Check if there's anything to commit
  if git diff --cached --quiet 2>/dev/null; then
    log "  No changes to commit."
    return 0
  fi

  git commit -m "$(cat <<EOF
$msg

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
EOF
  )" >> "$LOG_FILE" 2>&1

  git push origin main >> "$LOG_FILE" 2>&1
  if [ $? -eq 0 ]; then
    log "  Committed and pushed: $msg"
  else
    log "  Commit ok, push failed (will retry next cycle)"
  fi
}

verify() {
  log "  Running verification..."
  if ./verify.sh >> "$LOG_FILE" 2>&1; then
    log "  Verification PASSED"
    return 0
  else
    log "  Verification FAILED"
    return 1
  fi
}

# Determine where to resume from
get_start_index() {
  if [ -f "$CURRENT_PRD_FILE" ]; then
    local last_done
    last_done=$(cat "$CURRENT_PRD_FILE")
    for i in "${!PRDS[@]}"; do
      if [ "${PRDS[$i]}" = "$last_done" ]; then
        echo $((i + 1))
        return
      fi
    done
  fi
  echo 0
}

# ── Main ──

echo "" > "$LOG_FILE"
log "========================================="
log "  WIZSNEAKERS AUTONOMOUS BUILD"
log "  Starting at $(date)"
log "========================================="

START_INDEX=$(get_start_index)
if [ "$START_INDEX" -gt 0 ]; then
  log "Resuming from PRD #$((START_INDEX + 1)) (${PRDS[$START_INDEX]})"
  log "Last completed: ${PRDS[$((START_INDEX - 1))]}"
else
  log "Starting from the beginning (PRD 01)"
fi

TOTAL=${#PRDS[@]}

for i in $(seq "$START_INDEX" $((TOTAL - 1))); do
  PRD="${PRDS[$i]}"
  PRD_NUM=$((i + 1))
  PRD_FILE="prds/${PRD}.md"

  log ""
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  log "  PRD $PRD_NUM/$TOTAL: $PRD"
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  if [ ! -f "$PRD_FILE" ]; then
    log "  ERROR: $PRD_FILE not found, skipping"
    continue
  fi

  # Build the prompt for Claude Code
  PROMPT="You are executing an autonomous build. Read and implement the PRD at prds/${PRD}.md

CRITICAL RULES:
- Read the PRD file FIRST, then implement everything it specifies.
- Create all files listed in the PRD.
- Write all Rust tests specified in the PRD.
- After implementation, run: cd engine && cargo test && cd ..
- Then run: ./verify.sh
- If tests fail, fix the issues and re-run until they pass.
- Do NOT ask questions — make reasonable decisions and keep going.
- Do NOT skip any deliverables listed in the PRD.
- Reference spec/ files when you need exact values (stats, formulas, move data).
- Keep the existing client working — do not break what already renders.

When done, output EXACTLY this line on its own: PRD_COMPLETE"

  log "  Launching Claude Code..."

  # Run Claude Code with the PRD prompt
  # --max-turns high enough to finish a full PRD
  # --no-input so it doesn't block waiting for user
  claude --print \
    --max-turns 150 \
    --model claude-sonnet-4-6 \
    --allowedTools "Edit,Write,Read,Bash,Glob,Grep,Agent" \
    -p "$PROMPT" \
    >> "$LOG_FILE" 2>&1

  EXIT_CODE=$?

  if [ $EXIT_CODE -ne 0 ]; then
    log "  Claude Code exited with code $EXIT_CODE"
    # Still try to commit whatever was done
    commit_and_push "WIP: partial progress on PRD $PRD_NUM ($PRD)"
    log "  Retrying PRD $PRD_NUM in 30 seconds..."
    sleep 30
    # Retry once
    claude --print \
      --max-turns 100 \
      --model claude-sonnet-4-6 \
      --allowedTools "Edit,Write,Read,Bash,Glob,Grep,Agent" \
      -p "Continue implementing prds/${PRD}.md — pick up where you left off. Run ./verify.sh when done. Output PRD_COMPLETE when finished." \
      >> "$LOG_FILE" 2>&1
  fi

  # Verify
  if verify; then
    log "  PRD $PRD_NUM COMPLETE"
    echo "$PRD" > "$CURRENT_PRD_FILE"
    commit_and_push "feat: complete PRD $PRD_NUM — $PRD"
  else
    log "  PRD $PRD_NUM verification failed — committing WIP and moving on"
    commit_and_push "WIP: PRD $PRD_NUM ($PRD) — verification needs fixes"
    # Don't block the whole pipeline — later PRDs may still work
    echo "$PRD" > "$CURRENT_PRD_FILE"
  fi
done

log ""
log "========================================="
log "  BUILD COMPLETE at $(date)"
log "  Check build.log for full output"
log "========================================="

# Final push of any remaining changes
commit_and_push "chore: build runner complete"
