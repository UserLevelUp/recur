# main.git.checkpoint.readme

Purpose:
- Create consistent Git checkpoints between major state transitions.
- Keep progression visible in both file-leaf logs and commit history.
- Keep logging strictly opt-in (explicit command + flags).

## When To Checkpoint

Run this workflow at each major state change:
- before starting a new `todo.current` branch
- after completing an extraction/refactor unit
- before moving cursor from one branch to the next

## Checkpoint Commands

```bash
# 1) Snapshot only (no logging side effects)
recur checkpoint --snapshot

# 2) Snapshot + tests
recur checkpoint --snapshot --run-tests
```

## Commit Convention

Use a commit message that includes state movement:

```bash
git add -A
git commit -m "dogfooding: complete <branch>; cursor <from> -> <to>"
```

Examples:
- `dogfooding: complete main.command.files; cursor files -> children`
- `dogfooding: separator policy update; cursor unchanged`

## History Sync Rule

After commit:
- append entry to `docs/main.dogfooding.history.md`
- include separator changes in `docs/main.separator.history.md` when relevant
- include commit hash in the history entry once known
- append/emit a parallel-lane checkpoint in `docs/main.dogfooding.parallel.history.md`

## Parallel Lane Checkpoint

Use a checkpoint ID to bind state + git + separator in one record:

```bash
# emit entry to terminal
recur checkpoint --emit-parallel --checkpoint-id ck-children-01

# append entry to docs/main.dogfooding.parallel.history.md
recur checkpoint --append-parallel --checkpoint-id ck-children-01
```

## Optional PowerShell Helper Script

PowerShell helper:

```bash
powershell -ExecutionPolicy Bypass -File scripts/dogfooding_checkpoint.ps1 -RunTests
```

