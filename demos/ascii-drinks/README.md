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
