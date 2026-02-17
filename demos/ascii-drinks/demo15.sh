#!/usr/bin/env bash
# demo15.sh - Future-state visualization (Improvement 15)
#
# This script intentionally targets a not-yet-implemented recur surface:
# - recur merge --format flat
# - recur unflatten
#
# Goal:
# - very small orchestration loop
# - almost all composition performed by recur itself
# - profile-driven output behavior
#
# Usage:
#   ./demo15.sh                 # all drinks (future)
#   ./demo15.sh water           # one drink (future)
#
# Environment:
#   INCLUDE_EFFECTS=1           Include sparkle layer domain (default: 1)
#   DELAY=0.12                  Delay between drinks
#   LOOP=1                      Loop forever
#   NO_CLEAR=1                  Do not clear terminal between frames

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RECUR="${RECUR_BIN:-recur}"
TARGET="${1:-all}"
INCLUDE_EFFECTS="${INCLUDE_EFFECTS:-1}"
DELAY="${DELAY:-0.12}"
LOOP="${LOOP:-0}"

drinks_for_target() {
    case "$1" in
        all) echo "stout lager water empty" ;;
        stout|lager|water|empty) echo "$1" ;;
        *)
            echo "Error: unsupported target '$1' (use all|stout|lager|water|empty)" >&2
            exit 2
            ;;
    esac
}

require_future_surface() {
    local missing=0
    local merge_help
    local recur_help

    merge_help="$("$RECUR" merge --help 2>/dev/null || true)"
    recur_help="$("$RECUR" --help 2>/dev/null || true)"

    if ! grep -q -- "--format" <<< "$merge_help"; then
        missing=1
        echo "demo15 gate: missing future capability 'recur merge --format flat'"
    fi

    if ! grep -qi "unflatten" <<< "$recur_help"; then
        missing=1
        echo "demo15 gate: missing future command 'recur unflatten'"
    fi

    if [ "$missing" -eq 1 ]; then
        echo ""
        echo "demo15 is a future-state visualization and is expected to fail today."
        echo "Desired ultra-simple pipeline:"
        echo "  recur merge ... --format flat | recur unflatten --frames --format text --profile ..."
        echo ""
        echo "Once Improvement 15 is implemented, this script should run unchanged."
        exit 3
    fi
}

render_drink_future() {
    local drink="$1"
    local -a merge_cmd
    local base_profile="$SCRIPT_DIR/demo15.profile.base.json"
    local drink_profile="$SCRIPT_DIR/demo15.profile.${drink}.json"

    merge_cmd=(
        "$RECUR" merge
        --pattern "demo2.scene.mug.layer" --sep .
        --pattern "demo2_scene_mug_${drink}_layer" --sep _
        --base demo2
        --format flat
        --json
    )

    if [ "$INCLUDE_EFFECTS" = "1" ]; then
        merge_cmd+=(--pattern "demo2-scene-mug-layer" --sep -)
    fi

    "${merge_cmd[@]}" | "$RECUR" unflatten \
        --stdin \
        --frames \
        --frame-key frame \
        --format text \
        --profile "$base_profile" \
        --profile "$drink_profile"
}

play_target() {
    local drink
    while true; do
        for drink in $(drinks_for_target "$TARGET"); do
            if [ "${NO_CLEAR:-0}" != "1" ]; then
                printf '\033[2J\033[H'
            fi
            echo ""
            echo "  demo15: recur-native future pipeline ($drink)"
            echo "  merge --format flat | unflatten --frames --profile"
            echo ""
            render_drink_future "$drink" | sed 's/^/  /'
            echo ""
            sleep "$DELAY"
        done

        if [ "$LOOP" != "1" ]; then
            break
        fi
    done
}

cd "$SCRIPT_DIR"
require_future_surface
play_target
