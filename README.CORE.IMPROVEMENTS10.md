# Core Improvement 10: `recur-git` Extension

## Status

Implemented as a separate binary extension:
- `recur` stays focused on hierarchical semantics (`.` default, `--sep` override).
- `recur-git` handles workflow operations that need git/test orchestration.

---

## Thesis

Keep tool boundaries strict:

1. `recur`:
- file/content hierarchy semantics
- separator-aware matching
- no built-in git workflow logic

2. `recur-git`:
- workflow snapshot/checkpoint orchestration
- git state capture
- optional test execution
- parallel-lane checkpoint emission/appending

This keeps the core search engine pure and composable.

---

## Command Surface

`recur-git checkpoint` supports:
- `--snapshot`
- `--run-tests`
- `--run-julia-tests`
- `--emit-parallel`
- `--append-parallel`
- `--checkpoint-id <id>`
- `--parallel-log <path>` (default: `docs/main.dogfooding.parallel.history.md`)
- `--src-sep <char>` (default: `_`)

---

## Usage Examples

```bash
# Snapshot only
recur-git checkpoint --snapshot

# Snapshot + Rust/Julia tests
recur-git checkpoint --snapshot --run-tests --run-julia-tests

# Emit a checkpoint block
recur-git checkpoint --emit-parallel --checkpoint-id ck-children-01

# Append a checkpoint block to history
recur-git checkpoint --append-parallel --checkpoint-id ck-children-01
```

PowerShell wrapper (optional):

```bash
powershell -ExecutionPolicy Bypass -File scripts/dogfooding_checkpoint.ps1 -RunTests -RunJuliaTests -AppendParallelEntry -CheckpointId ck-children-01
```

---

## Data Captured

Checkpoint entries include:
- lane state (`docs` + `src` active `todo.current` leaves)
- git branch/head/worktree status
- separator policy (`.` for docs/tests, `--src-sep` for src)
- evidence commands for reproducing the lane snapshot

---

## Why This Matters

- Preserves purity of `recur` core semantics.
- Still supports practical parallel-lane coordination.
- Keeps composition-friendly workflows for Linux/PowerShell users.
- Reduces coupling between hierarchy parsing and external VCS behavior.
