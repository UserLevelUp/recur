# Command Tests Progress Snapshot

Status: `current`
Date: 2026-03-03

## Current Counts

- `trace-id`: `recur trace-id command (IMPROVEMENT8) |   13       6     19  0.5s`
- `callees`: `recur callees command |   24      10     34  0.5s`

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
