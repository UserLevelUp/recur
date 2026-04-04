# Command: init lane-name collision handling - Complete

Status: `complete`
Date: 2026-04-04

## What Landed

`recur init` now deduplicates generated lane section names when two directories
normalize to the same config key.

Example collision:

- `test-quick/`
- `test_quick/`

Both normalize to `test-quick`. Before this fix, `recur init --force` could emit
duplicate TOML sections and leave `.recur/config.toml` unparsable. Now the later
lane is suffixed (`test-quick-2`, `test-quick-3`, and so on) so the generated
config remains valid and `recur trait` can read it immediately.

## Docs + Test Surface

- `docs/main.command.init.readme.md` updated with lane-name collision behavior
- `docs/main.command.config.readme.md` updated to reflect the full trait surface
- `julia-tests/main.command.init.test.jl` added as the command-level wrapper
- `julia-tests/runtests.init.jl` added for init CLI, analyze-mode, and collision coverage

## Evidence

- `cargo test project_config -- --nocapture`
- `julia julia-tests/main.command.init.test.jl`

## Files

- `src/project_config.rs`
- `docs/main.command.init.readme.md`
- `docs/main.command.config.readme.md`
- `julia-tests/main.command.init.test.jl`
- `julia-tests/runtests.init.jl`
