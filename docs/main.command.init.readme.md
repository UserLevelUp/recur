# main.command.init.readme

Command overview for `init`.

Purpose:
- create `.recur/config.toml` from detected project lanes and separators
- analyze an existing project (`--analyze`) and suggest config updates

## Quick start

```bash
recur init
```

Creates:
- `.recur/config.toml` with detected lanes (for example `src/`, `docs/`, `julia-tests/`)
- `.recur/checkpoints.md` (if missing)

## Analyze mode

```bash
recur init --analyze
recur init --analyze --json
```

Reports:
- detected lanes and their suggested separators
- additions missing from config
- separator updates for configured lanes
- missing directories referenced by config

## Overwrite behavior

`recur init` is safe by default. If `.recur/config.toml` already exists, it exits with code 2 and does not overwrite.

Use:

```bash
recur init --force
```

only when you intentionally want to regenerate config.

## Why this matters

After `recur init`, commands that take `-d` can auto-resolve separator by directory from `.recur/config.toml`.

Example:

```bash
recur files "main_command_**" -d src/ --count
```

works without explicitly passing `--sep _` when `src/` is mapped to `_` in config.
