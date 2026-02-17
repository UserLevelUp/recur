#!/usr/bin/env bash
# demo3.sh - Compile/play animation pipeline driven by recur manifests
#
# Usage:
#   ./demo3.sh                    # run all drinks (compile on cache miss)
#   ./demo3.sh water              # run one drink
#   ./demo3.sh compile water      # compile cache only
#   ./demo3.sh play water         # play from cache only
#   ./demo3.sh clean              # remove cache
#
# Environment:
#   INCLUDE_EFFECTS=1             Include sparkle effect layers
#   WATER_SPARKLE_CYCLE=1         Animate water sparkle frames when available
#   WATER_SPARKLE_DELAY=0.08      Delay per water sparkle frame during playback
#   DELAY=0.35                    Delay per non-water frame during playback
#   TRANSPARENT_CHAR=#            Transparent symbol for composition
#   REBUILD=1                     Force recompilation even if cache exists
#   SHOW_MANIFEST=1               Print merged layer tree while compiling
#   NO_CLEAR=1                    Do not clear terminal between frames
#   LOOP=1                        Loop playback forever
#   CACHE_DIR=...                 Override cache directory

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RECUR="${RECUR_BIN:-}"
JQ_BIN="${JQ_BIN:-}"
AWK_BIN="${AWK_BIN:-}"

INCLUDE_EFFECTS="${INCLUDE_EFFECTS:-0}"
WATER_SPARKLE_CYCLE="${WATER_SPARKLE_CYCLE:-1}"
WATER_SPARKLE_DELAY="${WATER_SPARKLE_DELAY:-0.08}"
DELAY="${DELAY:-0.35}"
TRANSPARENT="${TRANSPARENT_CHAR:-#}"
REBUILD="${REBUILD:-0}"
SHOW_MANIFEST="${SHOW_MANIFEST:-0}"
CACHE_DIR="${CACHE_DIR:-$SCRIPT_DIR/.demo3-cache}"
SEARCH_DIR="."

MODE="run"
TARGET="all"
CUSTOM_DELAY=""

case "${1:-}" in
    "" ) ;;
    compile|play|run|clean)
        MODE="$1"
        TARGET="${2:-all}"
        CUSTOM_DELAY="${3:-}"
        ;;
    *)
        TARGET="$1"
        CUSTOM_DELAY="${2:-}"
        ;;
esac

if [ -n "$CUSTOM_DELAY" ]; then
    DELAY="$CUSTOM_DELAY"
fi

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

if [ -z "$AWK_BIN" ]; then
    if command -v gawk >/dev/null 2>&1; then
        AWK_BIN="gawk"
    else
        AWK_BIN="awk"
    fi
fi

require_command "$RECUR"
require_command "$JQ_BIN"
require_command "$AWK_BIN"

cd "$SCRIPT_DIR"

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

collect_layer_rows() {
    local drink="$1"
    local struct_json fill_json fx_json merged_json

    struct_json="$("$RECUR" tree "demo2.scene.mug.layer" -d "$SEARCH_DIR" --sep .)"
    fill_json="$("$RECUR" tree "demo2_scene_mug_${drink}_layer" -d "$SEARCH_DIR" --sep _)"

    if [ "$INCLUDE_EFFECTS" = "1" ]; then
        fx_json="$("$RECUR" tree "demo2-scene-mug-layer" -d "$SEARCH_DIR" --sep -)"
        if [ "$SHOW_MANIFEST" = "1" ]; then
            printf "%s\n%s\n%s\n" "$struct_json" "$fill_json" "$fx_json" | \
                "$RECUR" merge --stdin --base demo2 --sep . --sep _ --sep - --show-sep
            echo ""
        fi
        merged_json="$(printf "%s\n%s\n%s\n" "$struct_json" "$fill_json" "$fx_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --sep - --show-sep --json)"
    else
        if [ "$SHOW_MANIFEST" = "1" ]; then
            printf "%s\n%s\n" "$struct_json" "$fill_json" | \
                "$RECUR" merge --stdin --base demo2 --sep . --sep _ --show-sep
            echo ""
        fi
        merged_json="$(printf "%s\n%s\n" "$struct_json" "$fill_json" | \
            "$RECUR" merge --stdin --base demo2 --sep . --sep _ --show-sep --json)"
    fi

    printf "%s\n" "$merged_json" | \
        "$JQ_BIN" -r '.. | objects | select(.path != null) | .path' | \
        tr -d '\r' | \
        while IFS= read -r merged_path; do
            [ -z "$merged_path" ] && continue
            real_path="$(resolve_merged_path "$merged_path")"
            layer="$(extract_layer "$real_path")"
            role="$(extract_role "$real_path")"
            frame="$(extract_frame "$real_path")"
            printf "%03d\t%s\t%03d\t%s\n" \
                "$((10#$layer))" \
                "$role" \
                "$((10#$frame))" \
                "$real_path"
        done | sort -n -k1,1 -k3,3
}

overlay_two_files() {
    local base_file="$1"
    local top_file="$2"
    "$AWK_BIN" -v T="$TRANSPARENT" '
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
    "$AWK_BIN" -v T="$TRANSPARENT" '
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
    "$AWK_BIN" -v T="$TRANSPARENT" '
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

compose_from_current_rows() {
    local drink="$1"
    local effect_frame="${2:--1}"
    local -a render_layers temp_files
    local row layer role frame path rest mask_file
    local work_base work_next masked_file

    mask_file=""
    for row in "${CURRENT_ROWS[@]}"; do
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

    for row in "${CURRENT_ROWS[@]}"; do
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
            if [ "$effect_frame" -lt 0 ]; then
                continue
            fi
            if [ "$((10#$frame))" -ne "$((10#$effect_frame))" ]; then
                continue
            fi
        fi

        if [ "$role" = "fill" ] && [ -n "$mask_file" ]; then
            masked_file="$CACHE_DIR/.tmp.${drink}.$$.$layer.$frame.masked.txt"
            apply_mask_to_fill "$path" "$mask_file" > "$masked_file"
            render_layers+=("$masked_file")
            temp_files+=("$masked_file")
        else
            render_layers+=("$path")
        fi
    done

    if [ "${#render_layers[@]}" -eq 0 ]; then
        echo "No renderable layers for '$drink'" >&2
        return 1
    fi

    work_base="$CACHE_DIR/.tmp.${drink}.$$.base.txt"
    work_next="$CACHE_DIR/.tmp.${drink}.$$.next.txt"
    cp "${render_layers[0]}" "$work_base"

    for ((i = 1; i < ${#render_layers[@]}; i++)); do
        overlay_two_files "$work_base" "${render_layers[$i]}" > "$work_next"
        mv "$work_next" "$work_base"
    done

    strip_transparency_for_display < "$work_base"
    rm -f "$work_base" "$work_next" "${temp_files[@]:-}"
}

sparkle_frames_from_current_rows() {
    local row rest role frame
    for row in "${CURRENT_ROWS[@]}"; do
        rest="${row#*$'\t'}"
        role="${rest%%$'\t'*}"
        rest="${rest#*$'\t'}"
        frame="${rest%%$'\t'*}"
        if [ "$role" = "sparkle" ] && [ "$((10#$frame))" -gt 0 ]; then
            echo "$((10#$frame))"
        fi
    done | sort -n | uniq
}

compile_drink() {
    local drink="$1"
    local drink_dir="$CACHE_DIR/$drink"
    local -a sparkle_frames
    local idx frame

    mkdir -p "$drink_dir" "$CACHE_DIR"
    mapfile -t CURRENT_ROWS < <(collect_layer_rows "$drink")
    if [ "${#CURRENT_ROWS[@]}" -eq 0 ]; then
        echo "No layers discovered for '$drink'" >&2
        return 1
    fi

    printf "%s\n" "${CURRENT_ROWS[@]}" > "$drink_dir/manifest.tsv"
    rm -f "$drink_dir"/frame-*.txt "$drink_dir/base.txt"

    compose_from_current_rows "$drink" -1 > "$drink_dir/base.txt"

    if [ "$INCLUDE_EFFECTS" = "1" ] && [ "$drink" = "water" ] && [ "$WATER_SPARKLE_CYCLE" = "1" ]; then
        mapfile -t sparkle_frames < <(sparkle_frames_from_current_rows)
        if [ "${#sparkle_frames[@]}" -gt 0 ]; then
            idx=1
            for frame in "${sparkle_frames[@]}"; do
                compose_from_current_rows "$drink" "$frame" > "$drink_dir/frame-$(printf '%03d' "$idx").txt"
                idx=$((idx + 1))
            done
            return 0
        fi
    fi

    cp "$drink_dir/base.txt" "$drink_dir/frame-001.txt"
}

cache_ready() {
    local drink="$1"
    [ -f "$CACHE_DIR/$drink/frame-001.txt" ] && [ -f "$CACHE_DIR/$drink/manifest.tsv" ]
}

compile_target() {
    local drink
    mkdir -p "$CACHE_DIR"
    for drink in $(drinks_for_target "$TARGET"); do
        if [ "$REBUILD" = "1" ] || ! cache_ready "$drink"; then
            compile_drink "$drink"
        fi
    done
}

play_drink() {
    local drink="$1"
    local drink_dir="$CACHE_DIR/$drink"
    local -a frames
    local frame idx total pause

    if [ ! -d "$drink_dir" ]; then
        echo "Cache missing for '$drink' ($drink_dir). Run compile first." >&2
        exit 2
    fi

    mapfile -t frames < <(find "$drink_dir" -maxdepth 1 -type f -name 'frame-*.txt' | sort)
    if [ "${#frames[@]}" -eq 0 ]; then
        echo "No cached frames for '$drink'. Run compile first." >&2
        exit 2
    fi

    total="${#frames[@]}"
    idx=1
    for frame in "${frames[@]}"; do
        if [ "${NO_CLEAR:-0}" != "1" ]; then
            printf '\033[2J\033[H'
        fi
        echo ""
        echo "  demo3: cached playback ($drink) [$idx/$total]"
        if [ "$INCLUDE_EFFECTS" = "1" ] && [ "$drink" = "water" ] && [ "$total" -gt 1 ]; then
            echo "  mode: sparkle-cycle"
        else
            echo "  mode: static-frame"
        fi
        echo ""
        cat "$frame" | sed 's/^/  /'
        echo ""

        pause="$DELAY"
        if [ "$INCLUDE_EFFECTS" = "1" ] && [ "$drink" = "water" ] && [ "$total" -gt 1 ]; then
            pause="$WATER_SPARKLE_DELAY"
        fi
        sleep "$pause"
        idx=$((idx + 1))
    done
}

play_target() {
    local drink
    while true; do
        for drink in $(drinks_for_target "$TARGET"); do
            play_drink "$drink"
        done

        if [ "${LOOP:-0}" != "1" ]; then
            break
        fi
    done
}

clean_cache() {
    rm -rf "$CACHE_DIR"
    echo "Removed cache: $CACHE_DIR"
}

case "$MODE" in
    clean)
        clean_cache
        ;;
    compile)
        compile_target
        ;;
    play)
        play_target
        ;;
    run)
        compile_target
        play_target
        ;;
    *)
        echo "Error: unknown mode '$MODE'" >&2
        exit 2
        ;;
esac

if [ "$MODE" != "play" ] && [ "$MODE" != "run" ]; then
    exit 0
fi

if [ "${NO_CLEAR:-0}" != "1" ]; then
    printf '\033[2J\033[H'
fi
echo ""
echo "  demo3 complete."
echo ""
