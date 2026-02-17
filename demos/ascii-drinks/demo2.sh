#!/usr/bin/env bash
# demo2.sh - Block-layer transparency demo using recur + shell composition
#
# Usage:
#   ./demo2.sh                 # Animate stout -> lager -> water -> empty
#   ./demo2.sh stout           # Render one drink
#   ./demo2.sh all 0.7         # Animate all drinks with 0.7s delay
#
# Environment:
#   TRANSPARENT_CHAR           Character treated as transparent (default: '#')
#   SHOW_TRANSPARENT=1         Keep transparency mask glyphs visible (debug)
#   INCLUDE_EFFECTS=1          Include effect layers (default: off)
#   WATER_SPARKLE_CYCLE=1      When effects are on, animate water sparkle frames
#   WATER_SPARKLE_DELAY=0.12   Delay between water sparkle frames
#   LOOP=1                     Loop forever when selection is "all"
#   SHOW_MERGE=1               Print recur merge hierarchy before each frame
#   NO_CLEAR=1                 Do not clear terminal between frames

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RECUR="${RECUR_BIN:-}"
JQ_BIN="${JQ_BIN:-}"
TRANSPARENT="${TRANSPARENT_CHAR:-#}"
INCLUDE_EFFECTS="${INCLUDE_EFFECTS:-0}"
WATER_SPARKLE_CYCLE="${WATER_SPARKLE_CYCLE:-1}"
WATER_SPARKLE_DELAY="${WATER_SPARKLE_DELAY:-0.12}"
SELECTION="${1:-all}"
DELAY="${2:-1.0}"
TMPDIR="${TMPDIR:-/tmp}"
SEARCH_DIR="."

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Error: required command not found: $1" >&2
        exit 2
    fi
}

if [ -z "$RECUR" ]; then
    if command -v recur >/dev/null 2>&1; then
        RECUR="recur"
    elif command -v recur.exe >/dev/null 2>&1; then
        RECUR="recur.exe"
    else
        RECUR="recur"
    fi
fi

if [ -z "$JQ_BIN" ]; then
    if command -v jq >/dev/null 2>&1; then
        JQ_BIN="jq"
    elif command -v jq.exe >/dev/null 2>&1; then
        JQ_BIN="jq.exe"
    else
        JQ_BIN="jq"
    fi
fi

require_command "$RECUR"
require_command "$JQ_BIN"
require_command awk

cd "$SCRIPT_DIR"

extract_layer() {
    local file name
    file="$1"
    name="$(basename "$file")"
    if [[ "$name" =~ layer[._-]([0-9]+) ]]; then
        echo "${BASH_REMATCH[1]}"
    else
        echo "0"
    fi
}

extract_role() {
    local file name role
    file="$1"
    name="$(basename "$file")"
    if [[ "$name" =~ layer[._-][0-9]+[._-]([A-Za-z0-9]+) ]]; then
        role="${BASH_REMATCH[1]}"
        echo "${role,,}"
    else
        echo "layer"
    fi
}

extract_frame() {
    local file name
    file="$1"
    name="$(basename "$file")"
    if [[ "$name" =~ frame[._-]([0-9]+) ]]; then
        echo "${BASH_REMATCH[1]}"
    else
        echo "0"
    fi
}

resolve_merged_path() {
    local merged raw sep dir file stem ext
    merged="$1"

    if [[ "$merged" =~ ^(.*)[[:space:]]\[(.)\]$ ]]; then
        raw="${BASH_REMATCH[1]}"
        sep="${BASH_REMATCH[2]}"
    else
        raw="$merged"
        sep="."
    fi

    raw="${raw//\\//}"
    dir="$(dirname "$raw")"
    file="$(basename "$raw")"

    if [[ "$file" == *.* ]]; then
        stem="${file%.*}"
        ext=".${file##*.}"
    else
        stem="$file"
        ext=""
    fi

    if [ "$sep" != "." ]; then
        stem="${stem//./$sep}"
    fi

    if [ "$dir" = "." ]; then
        printf "%s%s\n" "$stem" "$ext"
    else
        printf "%s/%s%s\n" "$dir" "$stem" "$ext"
    fi
}

collect_layers() {
    local drink="$1"
    local struct_json fill_json fx_json merged_json

    struct_json="$("$RECUR" tree "demo2.scene.mug.layer" -d "$SEARCH_DIR" --sep .)"
    fill_json="$("$RECUR" tree "demo2_scene_mug_${drink}_layer" -d "$SEARCH_DIR" --sep _)"

    if [ "$INCLUDE_EFFECTS" = "1" ]; then
        fx_json="$("$RECUR" tree "demo2-scene-mug-layer" -d "$SEARCH_DIR" --sep -)"
        merged_json="$(printf "%s\n%s\n%s\n" "$struct_json" "$fill_json" "$fx_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --sep - --show-sep --json)"
    else
        merged_json="$(printf "%s\n%s\n" "$struct_json" "$fill_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --show-sep --json)"
    fi

    printf "%s\n" "$merged_json" | \
        "$JQ_BIN" -r '.. | objects | select(.path != null) | .path' | \
        tr -d '\r' | \
        while IFS= read -r merged_path; do
            [ -z "$merged_path" ] && continue
            real_path="$(resolve_merged_path "$merged_path")"
            frame="$(extract_frame "$real_path")"
            printf "%03d\t%s\t%s\t%s\n" \
                "$(extract_layer "$real_path")" \
                "$(extract_role "$real_path")" \
                "$real_path" \
                "$frame"
        done | while IFS=$'\t' read -r layer role path frame; do
            printf "%03d\t%s\t%03d\t%s\n" \
                "$layer" \
                "$role" \
                "$((10#$frame))" \
                "$path"
        done | sort -n -k1,1
}

list_sparkle_frames() {
    local drink="$1"
    collect_layers "$drink" | awk -F '\t' '$2=="sparkle" && $3+0>0 {print $3}' | awk '!seen[$0]++'
}

overlay_two_files() {
    local base_file="$1"
    local top_file="$2"
    awk -v T="$TRANSPARENT" '
        FNR == NR { base[FNR] = $0; if (FNR > base_rows) base_rows = FNR; next }
        { top[FNR] = $0; if (FNR > top_rows) top_rows = FNR }
        END {
            rows = (base_rows > top_rows) ? base_rows : top_rows
            for (r = 1; r <= rows; r++) {
                b = (r in base) ? base[r] : ""
                t = (r in top) ? top[r] : ""
                lb = length(b); lt = length(t)
                cols = (lb > lt) ? lb : lt
                out = ""
                for (c = 1; c <= cols; c++) {
                    tc = (c <= lt) ? substr(t, c, 1) : " "
                    bc = (c <= lb) ? substr(b, c, 1) : " "
                    out = out ((tc == T) ? bc : tc)
                }
                print out
            }
        }
    ' "$base_file" "$top_file"
}

apply_mask_to_fill() {
    local fill_file="$1"
    local mask_file="$2"
    awk -v T="$TRANSPARENT" '
        FNR == NR { mask[FNR] = $0; if (FNR > mask_rows) mask_rows = FNR; next }
        { fill[FNR] = $0; if (FNR > fill_rows) fill_rows = FNR }
        END {
            rows = (mask_rows > fill_rows) ? mask_rows : fill_rows
            for (r = 1; r <= rows; r++) {
                m = (r in mask) ? mask[r] : ""
                f = (r in fill) ? fill[r] : ""
                lm = length(m); lf = length(f)
                cols = (lm > lf) ? lm : lf
                out = ""
                for (c = 1; c <= cols; c++) {
                    mc = (c <= lm) ? substr(m, c, 1) : T
                    fc = (c <= lf) ? substr(f, c, 1) : T
                    out = out ((mc == T) ? T : fc)
                }
                print out
            }
        }
    ' "$mask_file" "$fill_file"
}

strip_transparency_for_display() {
    awk -v T="$TRANSPARENT" '
        {
            line = $0
            out = ""
            for (i = 1; i <= length(line); i++) {
                ch = substr(line, i, 1)
                out = out ((ch == T) ? " " : ch)
            }
            print out
        }
    '
}

compose_drink() {
    local drink="$1"
    local effect_frame="${2:-0}"
    local -a layer_rows render_layers temp_files
    local work_base work_next mask_file layer rest role frame path masked_file selected_sparkle

    mapfile -t layer_rows < <(collect_layers "$drink")
    if [ "${#layer_rows[@]}" -eq 0 ]; then
        echo "No layers found for drink '$drink'" >&2
        return 1
    fi

    if ! [[ "$effect_frame" =~ ^[0-9]+$ ]]; then
        effect_frame="0"
    fi

    mask_file=""
    for row in "${layer_rows[@]}"; do
        layer="${row%%$'\t'*}"
        rest="${row#*$'\t'}"
        role="${rest%%$'\t'*}"
        rest="${rest#*$'\t'}"
        frame="${rest%%$'\t'*}"
        path="${rest#*$'\t'}"
        if [ "$role" = "mask" ]; then
            mask_file="$path"
            break
        fi
    done

    render_layers=()
    temp_files=()
    selected_sparkle="0"
    for row in "${layer_rows[@]}"; do
        layer="${row%%$'\t'*}"
        rest="${row#*$'\t'}"
        role="${rest%%$'\t'*}"
        rest="${rest#*$'\t'}"
        frame="${rest%%$'\t'*}"
        path="${rest#*$'\t'}"

        if [ "$role" = "mask" ]; then
            continue
        fi

        if [ "$role" = "sparkle" ]; then
            if [ "$effect_frame" != "0" ]; then
                if [ "$((10#$frame))" -ne "$((10#$effect_frame))" ]; then
                    continue
                fi
            else
                if [ "$selected_sparkle" = "1" ]; then
                    continue
                fi
                selected_sparkle="1"
            fi
        fi

        if [ "$role" = "fill" ] && [ -n "$mask_file" ]; then
            masked_file="$TMPDIR/demo2.${drink}.$$.masked.${layer}.txt"
            apply_mask_to_fill "$path" "$mask_file" > "$masked_file"
            render_layers+=("$masked_file")
            temp_files+=("$masked_file")
        else
            render_layers+=("$path")
        fi
    done

    if [ "${#render_layers[@]}" -eq 0 ]; then
        echo "No renderable layers found for drink '$drink'" >&2
        return 1
    fi

    work_base="$TMPDIR/demo2.${drink}.$$.base.txt"
    work_next="$TMPDIR/demo2.${drink}.$$.next.txt"
    cp "${render_layers[0]}" "$work_base"

    for ((i = 1; i < ${#render_layers[@]}; i++)); do
        overlay_two_files "$work_base" "${render_layers[$i]}" > "$work_next"
        mv "$work_next" "$work_base"
    done

    if [ "${SHOW_TRANSPARENT:-0}" = "1" ]; then
        cat "$work_base"
    else
        strip_transparency_for_display < "$work_base"
    fi
    rm -f "$work_base" "$work_next" "${temp_files[@]:-}"
}

show_merge_view() {
    local drink="$1"
    local struct_json fill_json fx_json

    struct_json="$("$RECUR" tree "demo2.scene.mug.layer" -d "$SEARCH_DIR" --sep .)"
    fill_json="$("$RECUR" tree "demo2_scene_mug_${drink}_layer" -d "$SEARCH_DIR" --sep _)"

    if [ "$INCLUDE_EFFECTS" = "1" ]; then
        fx_json="$("$RECUR" tree "demo2-scene-mug-layer" -d "$SEARCH_DIR" --sep -)"
        printf "%s\n%s\n%s\n" "$struct_json" "$fill_json" "$fx_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --sep - --show-sep
    else
        printf "%s\n%s\n" "$struct_json" "$fill_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --show-sep
    fi
}

draw_frame() {
    local drink="$1"
    local effect_frame="${2:-0}"

    if [ "${NO_CLEAR:-0}" != "1" ]; then
        printf '\033[2J\033[H'
    fi

    echo ""
    echo "  demo2: block-layer composition ($drink)"
    if [ "$INCLUDE_EFFECTS" = "1" ]; then
        echo "  transparent='$TRANSPARENT'  separators='.,_,-'"
    else
        echo "  transparent='$TRANSPARENT'  separators='.,_'"
    fi
    if [ "$effect_frame" != "0" ]; then
        echo "  sparkle-frame='$effect_frame'"
    fi
    echo ""

    if [ "${SHOW_MERGE:-0}" = "1" ]; then
        show_merge_view "$drink"
        echo ""
    fi

    compose_drink "$drink" "$effect_frame" | sed 's/^/  /'
    echo ""
}

if [ "$SELECTION" = "all" ]; then
    drinks=(stout lager water empty)
else
    drinks=("$SELECTION")
fi

while true; do
    for drink in "${drinks[@]}"; do
        if [ "$INCLUDE_EFFECTS" = "1" ] && [ "$WATER_SPARKLE_CYCLE" = "1" ] && [ "$drink" = "water" ]; then
            mapfile -t sparkle_frames < <(list_sparkle_frames "$drink")
            if [ "${#sparkle_frames[@]}" -gt 0 ]; then
                for frame in "${sparkle_frames[@]}"; do
                    draw_frame "$drink" "$frame"
                    sleep "$WATER_SPARKLE_DELAY"
                done
                continue
            fi
        fi

        draw_frame "$drink"
        sleep "$DELAY"
    done

    if [ "${LOOP:-0}" != "1" ] || [ "$SELECTION" != "all" ]; then
        break
    fi
done

if [ "${NO_CLEAR:-0}" != "1" ]; then
    printf '\033[2J\033[H'
fi
echo ""
echo "  demo2 complete."
echo ""
