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
- default `[reveal]` sections for lane-local `*.recur.md` ignition capsules
- core workflow skill registrations: expert, Warp, Lang, Watch, trace-id and demo tests
- all default `[traits.*]` sections used by `recur trait`
- capability-trait preference/notes sections for warp, watch, merge, unmerge and git
- `[warp.discovery]` roots and directory exclusions for project-aware Warp inventory

Generated lane names are normalized to lowercase kebab-case section names.
If two directories collapse to the same normalized lane name, `recur init`
appends a numeric suffix so the generated TOML stays valid.

Example:

```toml
[test-quick]
dir = "test-quick/"
sep = "-"

[test-quick-2]
dir = "test_quick/"
sep = "."
```

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

When `.recur/config.toml` already exists, `recur init` preserves project settings
and adds only missing Reveal registration defaults. Custom skill paths, persona
associations and empty opt-outs remain authoritative. Repeated runs become a
no-op; `--analyze` remains a read-only lane/separator report. Skill registration
does not install bodies or execute agents. See
[core skill initialization](main.command.reveal.core-skills.readme.md) for scope.

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

## Testing

- `julia-tests/main.command.init.test.jl` - command-level wrapper for init CLI coverage
- `julia-tests/runtests.init.jl` - init command contract, analyze mode, and collision tests

## Related

- `docs/main.command.config.readme.md`
- `docs/main.command.init.lane-name-collision.complete.md`
