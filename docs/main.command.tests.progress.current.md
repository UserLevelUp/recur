# Command Tests Progress Snapshot

Status: `current`
Date: 2026-04-06

## Current Counts

- `trace-id`: `recur trace-id command (IMPROVEMENT8) |   88       3     91  1.2s`
- `callees`: `recur callees command |   10      13      10     33  1.4s`

## Trace-id Notes

- Phase 4b now covers saved runs: `--save-run`, `--check-run`, `--reuse-if-fresh`
- Phase 4b now also pins the current persistence contract as `latest`-only
  (`manifest.toml` + `latest.json`, no `history/` directory yet)
- Phase 4c now proves a transition-audit pattern on plain-text eventness files:
  a saved run goes stale when rollback evidence appears, and a refreshed trace
  captures both forward and backward transition evidence
- Phase 5 `trace-id -> merge` now passes and retains `edge_type` through merge JSON
- Remaining `Broken = 3` entries are the permanent `@test_skip` placeholders for
  `trace`, `callers`, and `callees` edge-metadata composition
- Eventness lane for saved-run polish: `docs/main.command.trace-id.run.todo.current.md`
- Improvement close-out: `docs/main.improvement.9.trace-id.complete.md`

## Future Coverage To Add

- If saved runs gain timestamped history, add assertions for retained
  `.recur/trace-id/runs/<name>/history/` artifacts and documented retention policy
- If mirror or in-file eventness becomes canonical, add assertions that saved-run
  status is derived or validated against that canonical layer rather than treated
  as source truth by itself

## Callees Notes

- Re-ran `julia julia-tests/main.command.callees.test.jl` on 2026-04-06 and the
  suite is currently failing outside this change scope
- Failures are broad (`success` is false across basic search, scoped search,
  count, and JSON paths), so the older passing snapshot should not be treated as
  current branch truth

## Repro Commands

```powershell
julia julia-tests/main.command.trace-id.test.jl
julia julia-tests/main.command.callees.test.jl
```

## Merge View (docs + julia-tests)

```text
# recur merge .tmp/tree.docs.trace-id.json .tmp/tree.julia.trace-id.json --base "main.command.trace-id" --sep . --sep .
main.command.trace.id
```

```text
# recur merge .tmp/tree.docs.callees.json .tmp/tree.julia.callees.json --base "main.command.callees" --sep . --sep .
main.command.callees
├── readme.md
├── stdin
│   └── todo.md
└── test.jl
```

## Snapshot Inputs

```powershell
recur tree "main.command.trace-id" -d docs --json > .tmp/tree.docs.trace-id.json
recur tree "main.command.trace-id" -d julia-tests --json > .tmp/tree.julia.trace-id.json
recur tree "main.command.callees" -d docs --json > .tmp/tree.docs.callees.json
recur tree "main.command.callees" -d julia-tests --json > .tmp/tree.julia.callees.json
```

## Diff Next Checkpoint

```powershell
git diff -- docs/main.command.tests.progress.current.md
```
