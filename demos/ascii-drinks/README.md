# ASCII Drinks Demo - Hold My Beer Edition

Demonstrates `recur merge` composability using ASCII art drinks.

## The Concept

Container parts use **dot notation**: `drink.mug.1top.txt`, `drink.mug.3bottom.txt`
Drink contents use **underscore notation**: `drink_mug_stout_2fill.txt`, `drink_mug_lager_2fill.txt`

`recur merge` unifies both naming conventions into a single hierarchy:

```powershell
# Get tree outputs from each separator domain, then merge
$struct = recur tree drink.mug -d . --sep . | Out-String
$fill   = recur tree drink_mug_stout -d . --sep _ | Out-String
Write-Output $struct $fill | recur merge --stdin --base drink --sep . --sep _ --show-sep
```

```
drink
└── mug
    ├── 1top.txt [.]
    ├── 3bottom.txt [.]
    └── stout
        └── 2fill.txt [_]
```

## File Naming

Number prefixes control visual ordering:
- `1top` / `1rim` — top of the glass
- `2fill` — the drink contents (swappable!)
- `3bottom` / `3bowl` — bottom of the glass
- `4stem`, `5base` — wine glass stem and base

## Demo2: Block Layers + Transparency

`demo2.sh` levels this up using full-canvas text blocks:
- Dot separator (`.`): structure layers (`mask`, `cup`)
- Underscore separator (`_`): drink fill layers
- Hyphen separator (`-`): optional effect layers

Instead of stitching many small parts, each file is a full frame where a
transparent character (default `#`) means "let lower layer show through."
By default, transparency mask glyphs are hidden in the final render.
Effect layers are opt-in (`INCLUDE_EFFECTS=1`).
`recur merge` is used to unify layer discovery across separators before render.
When effects are enabled, water auto-cycles sparkle frames (`frame-01..frame-10`).

Example layer names:
- `demo2.scene.mug.layer.05.mask.txt`
- `demo2.scene.mug.layer.20.cup.txt`
- `demo2_scene_mug_stout_layer_10_fill.txt`
- `demo2-scene-mug-layer-30-sparkle-frame-01.txt`

## Demo3: Compile + Play Cache

`demo3.sh` shows a near-future pipeline shape:
- `recur tree` + `recur merge` build a merged layer manifest
- `jq` extracts layer/frame rows
- `gawk` composes frames once into cache
- playback is a tiny frame loop (`cat + sleep`)

This demonstrates the "selector + orchestration + minimal pipe" model with
faster replay after initial compile.

## Demo15: Simplicity Target (Future-State)

`demo15.sh` is intentionally a vision script for Improvement 15:
- one recur merge stage for layer selection
- one recur unflatten stage for render/materialization
- a tiny shell loop for playback only

It is expected to fail today because the required surface does not exist yet
(`merge --format flat` and `unflatten`). The point is to pin the desired UX
for where recur should go.

## Scripts

### PowerShell (Windows)

| Script | Description |
|--------|-------------|
| `demo.ps1` | Quick walkthrough showing recur merge in action |
| `pour.ps1 -Glass <glass> -Drink <drink>` | Pour a specific drink (e.g., `.\pour.ps1 -Glass mug -Drink stout`) |
| `tasting.ps1 [-Glass mug\|wine\|all]` | Animated tasting menu cycling through drinks |
| `bar.ps1` | Interactive bar — browse drinks with arrow keys, colorized ASCII art |

### Bash (Linux/macOS)

| Script | Description |
|--------|-------------|
| `demo.sh` | Quick walkthrough showing recur merge in action |
| `pour.sh <glass> <drink>` | Pour a specific drink (e.g., `./pour.sh mug stout`) |
| `tasting.sh [mug\|wine\|all]` | Animated tasting menu cycling through drinks |
| `demo2.sh [drink\|all] [delay]` | Block-layer compositor with transparency and a simple animation loop |
| `demo3.sh [run\|compile\|play\|clean] [drink\|all] [delay]` | Compile/play cached frames from recur-merged manifests |
| `demo15.sh [drink\|all]` | Future-state recur-native pipeline (expected to fail until Improvement 15) |

## Available Drinks

### Beer Mugs
`stout` `lager` `ipa` `water` `half` `empty`

### Wine Glasses
`red` `white` `rose` `empty`

## Quick Start

### PowerShell Setup (no admin required)

If scripts are blocked by execution policy:

```powershell
# Option 1: Set policy for current user only (persists)
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned

# Option 2: Set policy for this session only (temporary)
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

# Option 3: Run a single script without changing policy
powershell -ExecutionPolicy Bypass -File .\demo.ps1
```

Then run the demos:

```powershell
cd demos\ascii-drinks
.\demo.ps1                                  # walkthrough
.\pour.ps1 -Glass mug -Drink stout          # pour one drink
.\tasting.ps1 -Glass wine -Delay 2          # animated wine flight
.\bar.ps1                                   # interactive bar (see below)
```

### Bash
```bash
cd demos/ascii-drinks
chmod +x *.sh
./demo.sh
./pour.sh mug ipa
./tasting.sh wine
./demo2.sh
./demo2.sh stout
SHOW_MERGE=1 ./demo2.sh water 1.2
LOOP=1 ./demo2.sh all 0.7
SHOW_TRANSPARENT=1 ./demo2.sh stout
INCLUDE_EFFECTS=1 ./demo2.sh water
INCLUDE_EFFECTS=1 WATER_SPARKLE_DELAY=0.08 ./demo2.sh water
INCLUDE_EFFECTS=1 WATER_SPARKLE_CYCLE=0 ./demo2.sh water
./demo3.sh
./demo3.sh water
./demo3.sh compile all
./demo3.sh play all
INCLUDE_EFFECTS=1 WATER_SPARKLE_DELAY=0.05 ./demo3.sh run water
./demo3.sh clean
./demo15.sh water
```

## The Interactive Bar (`bar.ps1`)

`bar.ps1` is a full-screen interactive TUI with **colorized ASCII art**:

- **Arrow keys** (left/right) to browse through all 10 drinks
- **Number keys** (1–9, 0) to jump directly to a drink
- **Q** to quit
- Each drink renders with its own color (stout=dark red, lager=yellow, water=cyan, etc.)
- Shows which files make up the drink: structure `[.]` vs contents `[_]`
- Features witty quips from Skippy the Magnificent

```
  =======================================================
  Skippy & Joe's Bar & Merge
  Hold My Beer Edition -- powered by recur merge
  =======================================================

  === Irish Stout ===

    \~~~~~~~~~~~~/
    |##########|  |~~|       (dark red fill)
    |##########|  |  |
    |##########|  |  |
    |##########|  |__|
    \__________/

  "Dark as my commit history at 3am."
  -- Skippy the Magnificent

  Structure: drink.mug.* [.]
  Contents:  drink_mug_stout_* [_]

  [1/10]  Left/Right to browse, Q to quit
```

A static SVG preview is also included: [bar.svg](bar.svg)

## Sample Output

```
    \~~~~~~~~~~~~/              \_____/
    |##########|  |~~|          |#####|
    |##########|  |  |          |#####|
    |##########|  |  |          |#####|
    |##########|  |__|          \_____/
    \__________/                  | |
                                  | |
                               /~~~~~\
     Stout in a Mug           Red Wine
```
