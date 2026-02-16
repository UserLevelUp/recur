# recur-git Checkpoint Config Bug Patch Complete

Status: `complete`
Date: 2026-02-15

## Completed

- `recur-git checkpoint` now discovers active current files from `.recur/config.toml` lane definitions.
- Checkpoint discovery now uses config values:
  - `[checkpoint].root_pattern`
  - `[status].current_suffix`
- `--append-parallel` now defaults to `[checkpoint].file` when `--file` is omitted.
- Wrapper script now uses `--file` when appending checkpoint logs.
- Wrapper script now prefers local `target/debug` / `target/release` `recur-git` builds before PATH.

## Evidence

- `cargo test --bin recur-git`
- `cargo run --quiet --bin recur-git -- checkpoint --snapshot`
- `cargo run --quiet --bin recur-git -- checkpoint --append-parallel --checkpoint-id ck-config-bug-smoke`
- `powershell -ExecutionPolicy Bypass -File scripts/dogfooding_checkpoint.ps1 -AppendParallelEntry -CheckpointId ck-script-file-smoke -ParallelLogPath .recur/checkpoints.script-smoke.md`

## Files

- `src/recur_git_main.rs`
- `scripts/dogfooding_checkpoint.ps1`
- `docs/main.command.checkpoint.readme.md`
- `docs/main.git.checkpoint.readme.md`
- `docs/main.recur-git.artifact.md`
