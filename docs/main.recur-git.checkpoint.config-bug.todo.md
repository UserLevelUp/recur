# recur-git Checkpoint Config Bug Patch

Status: `complete` (historical todo record; see `docs/main.recur-git.checkpoint.config-bug.complete.md`)

## Problem

`recur-git checkpoint` still had hardcoded lane/pattern assumptions for current-file discovery and required `--file` even when `.recur/config.toml` already defines `[checkpoint].file`.

## Scope

- make checkpoint lane discovery config-driven (`lanes`, `checkpoint.root_pattern`, `status.current_suffix`)
- keep fallback behavior when config is missing
- use config checkpoint file as default for `--append-parallel` when `--file` is not provided
- align wrapper script flag usage with `recur-git checkpoint --file`

## Files

- `src/recur_git_main.rs`
- `scripts/dogfooding_checkpoint.ps1`
