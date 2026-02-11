#!/usr/bin/env bash
# pour.sh - Compose a drink using recur merge
#
# Usage:
#   ./pour.sh mug stout     # Pour a stout into a mug
#   ./pour.sh wine red      # Pour red wine into a glass

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RECUR="${RECUR_BIN:-recur}"
TMPDIR="${TMPDIR:-/tmp}"

GLASS="${1:-mug}"
DRINK="${2:-empty}"

echo ""
echo "  Pouring $DRINK into $GLASS..."
echo ""

# Show the recur merge view - structure [.] and contents [_] unified
echo "  [recur merge view]"
"$RECUR" tree "drink.$GLASS" -d "$SCRIPT_DIR" --sep . > "$TMPDIR/drink_struct.json"
"$RECUR" tree "drink_${GLASS}_${DRINK}" -d "$SCRIPT_DIR" --sep _ > "$TMPDIR/drink_fill.json"
cat "$TMPDIR/drink_struct.json" "$TMPDIR/drink_fill.json" | \
    "$RECUR" merge --stdin --base drink --sep . --sep _ --show-sep | sed 's/^/  /'
echo ""

# Compose ASCII art by sorting files by their number prefix
# 1top -> 2fill -> 3bottom (mug) or 1rim -> 2fill -> 3bowl -> 4stem -> 5base (wine)
SORT_FILE="$TMPDIR/drink_sort.txt"
> "$SORT_FILE"
for f in "$SCRIPT_DIR"/drink."$GLASS".*.txt "$SCRIPT_DIR"/drink_"${GLASS}"_"${DRINK}"_*.txt; do
    [ -f "$f" ] || continue
    num=$(basename "$f" | grep -oP '\d+(?=[a-z]+\.txt$)')
    echo "${num:-0} $f" >> "$SORT_FILE"
done

sort -n "$SORT_FILE" | cut -d' ' -f2- | xargs cat

echo ""
