#!/usr/bin/env bash
# tasting.sh - Animated drink tasting using recur
#
# Usage:
#   ./tasting.sh           # Full tasting
#   ./tasting.sh mug       # Beer only
#   ./tasting.sh wine      # Wine only

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TMPDIR="${TMPDIR:-/tmp}"
DELAY="${2:-2}"

compose_drink() {
    local glass="$1"
    local drink="$2"
    local sf="$TMPDIR/drink_sort.txt"

    > "$sf"
    for f in "$SCRIPT_DIR"/drink."$glass".*.txt "$SCRIPT_DIR"/drink_"${glass}"_"${drink}"_*.txt; do
        [ -f "$f" ] || continue
        num=$(basename "$f" | grep -oP '\d+(?=[a-z]+\.txt$)')
        echo "${num:-0} $f" >> "$sf"
    done

    sort -n "$sf" | cut -d' ' -f2- | xargs cat
}

show_drink() {
    printf '\033[2J\033[H'
    echo ""
    echo "  === $3 ==="
    echo ""
    compose_drink "$1" "$2"
    echo ""
    sleep "$DELAY"
}

echo ""
echo "  Hold my beer... recur is about to show off."
echo ""
sleep 1

case "${1:-all}" in
    mug)
        show_drink mug empty  "Empty Mug"
        show_drink mug half   "Half Pint"
        show_drink mug stout  "Irish Stout"
        show_drink mug lager  "Golden Lager"
        show_drink mug ipa    "Hoppy IPA"
        show_drink mug water  "Just Water"
        ;;
    wine)
        show_drink wine empty "Empty Glass"
        show_drink wine white "Chardonnay"
        show_drink wine rose  "Rose"
        show_drink wine red   "Cabernet"
        ;;
    all)
        show_drink mug empty  "Empty Mug"
        show_drink mug stout  "Irish Stout"
        show_drink mug lager  "Golden Lager"
        show_drink mug ipa    "Hoppy IPA"
        show_drink wine empty "Empty Glass"
        show_drink wine white "Chardonnay"
        show_drink wine rose  "Rose"
        show_drink wine red   "Cabernet"
        ;;
esac

printf '\033[2J\033[H'
echo ""
echo "  Cheers! That's recur merge - different naming conventions, one view."
echo ""
