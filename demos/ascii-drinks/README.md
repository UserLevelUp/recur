# ASCII Drinks Demo - Hold My Beer Edition

Demonstrates `recur merge` composability using ASCII art drinks.

## The Concept

Container parts use **dot notation**: `drink.mug.1top.txt`, `drink.mug.3bottom.txt`
Drink contents use **underscore notation**: `drink_mug_stout_2fill.txt`, `drink_mug_lager_2fill.txt`

`recur merge` unifies both naming conventions into a single hierarchy:

```bash
recur tree drink.mug -d . --sep . --json | \
  recur merge --stdin drink_mug_stout_2fill.txt --sep . --sep _ --show-sep
```

## File Naming

Number prefixes control visual ordering:
- `1top` / `1rim` - top of the glass
- `2fill` - the drink contents (swappable!)
- `3bottom` / `3bowl` - bottom of the glass
- `4stem`, `5base` - wine glass stem and base

## Scripts

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

```bash
cd demos/ascii-drinks
chmod +x *.sh
./demo.sh
```
