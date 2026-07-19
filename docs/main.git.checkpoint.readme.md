# main.git.checkpoint.readme

Purpose:
- Create consistent Git checkpoints between major state transitions.
- Keep progression visible in both file-leaf logs and commit history.
- Keep logging strictly opt-in (explicit shell command + flags).

## When To Checkpoint

Run this workflow at each major state change:
- before starting a new `todo.current` branch
- after completing an extraction/refactor unit
- before moving cursor from one branch to the next
- pair it with the active `todo.trigger.event` checklist (manual, no hidden automation)

## Checkpoint Commands (`recur-git` Extension)

```bash
# 1) Snapshot only (no append side effect)
recur-git checkpoint --snapshot

# 2) Snapshot + tests
recur-git checkpoint --snapshot --run-tests --run-julia-tests

# 3) Emit checkpoint entry to stdout
recur-git checkpoint --emit-parallel --checkpoint-id ck-children-01

# 4) Record a verified behavior as immutable test eventness
recur-git test-receipt main.command.tree.wildcard-current --julia-file julia-tests/main.command.tree.wildcard-current.test.jl
```

## Test Eventness Receipts

`recur-git test-receipt` is a bounded writer. It requires a clean worktree and
a committed `HEAD`, then runs exactly one selected target:

```bash
recur-git test-receipt <test-id> --cargo
recur-git test-receipt <test-id> --julia-full
recur-git test-receipt <test-id> --julia-file julia-tests/runtests.tree.jl
```

It writes an immutable local receipt under `.recur/tests/`:

```text
main.command.tree.wildcard-current.test.<head>.passed.complete.md
main.command.tree.wildcard-current.test.<head>.failed.strange.md
```

The receipt names the test identity, tested Git head, exact command, exit code,
and timestamp. `checkpoint --snapshot` and appended parallel checkpoints list
both passed and failed test receipts, making behavior changes visible between
Git snapshots without adding generated run evidence to source history.

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
recur-git checkpoint --emit-parallel --checkpoint-id ck-children-01

# append entry to docs/main.dogfooding.parallel.history.md
recur-git checkpoint --append-parallel --checkpoint-id ck-children-01
# explicit path override
recur-git checkpoint --append-parallel --checkpoint-id ck-children-01 -f docs/main.dogfooding.parallel.history.md
```

If `-f` is omitted, `recur-git` uses `[checkpoint].file` from `.recur/config.toml` when available.

## Optional PowerShell Helper Script

PowerShell helper:

```bash
powershell -ExecutionPolicy Bypass -File scripts/dogfooding_checkpoint.ps1 -RunTests
```

Purity rule:
- `recur` stays hierarchy-only.
- git/workflow integration lives in `recur-git` and shell scripts.

