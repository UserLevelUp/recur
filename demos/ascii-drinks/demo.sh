#!/usr/bin/env bash
# demo.sh - Quick demo of recur merge composability with ASCII drinks
#
# Shows how files from different naming conventions (dots vs underscores)
# are discovered and merged by recur to compose ASCII art drinks.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RECUR="${RECUR_BIN:-recur}"
TMPDIR="${TMPDIR:-/tmp}"

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

echo ""
echo "============================================"
echo "  recur merge demo: Hold My Beer Edition"
echo "============================================"
echo ""
echo "  Files use TWO naming conventions:"
echo "    Structure (dots):       drink.mug.1top.txt"
echo "    Contents (underscores): drink_mug_stout_2fill.txt"
echo ""
echo "  recur merge unifies them with --sep . --sep _"
echo ""
echo "--------------------------------------------"
echo "  1. recur tree (dot notation - structure)"
echo "--------------------------------------------"
"$RECUR" tree drink.mug -d "$SCRIPT_DIR" --sep .
echo ""

echo "--------------------------------------------"
echo "  2. recur tree (underscore notation - stout)"
echo "--------------------------------------------"
"$RECUR" tree drink_mug_stout -d "$SCRIPT_DIR" --sep _
echo ""

echo "--------------------------------------------"
echo "  3. recur merge (unified view!)"
echo "--------------------------------------------"
"$RECUR" tree drink.mug -d "$SCRIPT_DIR" --sep . > "$TMPDIR/drink_struct.json"
"$RECUR" tree drink_mug_stout -d "$SCRIPT_DIR" --sep _ > "$TMPDIR/drink_fill.json"
cat "$TMPDIR/drink_struct.json" "$TMPDIR/drink_fill.json" | \
    "$RECUR" merge --stdin --base drink --sep . --sep _ --show-sep
echo ""

echo "--------------------------------------------"
echo "  4. Composed ASCII art: Stout in a Mug"
echo "--------------------------------------------"
compose_drink mug stout
echo ""

echo "--------------------------------------------"
echo "  5. Same mug, different drink: Lager"
echo "--------------------------------------------"
compose_drink mug lager
echo ""

echo "--------------------------------------------"
echo "  6. Wine glass: Red Wine"
echo "--------------------------------------------"
compose_drink wine red
echo ""

echo "--------------------------------------------"
echo "  7. Same glass, different wine: Rose"
echo "--------------------------------------------"
compose_drink wine rose
echo ""

echo "============================================"
echo "  The container stays the same."
echo "  Only the contents change."
echo "  recur merge makes it all composable."
echo "============================================"
echo ""
