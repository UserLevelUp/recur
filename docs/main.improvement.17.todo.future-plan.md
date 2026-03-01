# Improvement 17: Depth-Windowed Token Separator Pipelines

Status: `todo.future-plan` (long-distance backlog, implementation deferred)

## Lane Policy

Improvement 17 is parked by design.

- Do not open `todo.current` for Improvement 17 yet.
- Keep work as contracts/docs/investigation artifacts only.
- Active implementation priority remains current phase lanes.

## Objective

Prepare a future-safe design for:

1. token-aware flattening,
2. depth-window chunking/filtering of flat records,
3. token-aware unflatten round-trips.

## Current Snapshot (2026-03-01)

- Token separators validated in `tree/files/merge` paths.
- Flatten token parity is not complete yet.
- Unflatten remains future work (Improvement 15 scope).

## Planned Phases

| Phase | Name | Outcome | Status |
|------|------|---------|--------|
| A | Contract Freeze | token + depth-window contract defined | planned |
| B | Flatten Token Parity | flatten honors full separator tokens | planned |
| C | Unflatten Token Parity | unflatten consumes token paths correctly | planned |
| D | Chunk Ops | depth-window chunk/filter strategy | planned |
| E | Benchmarks | deep/wide performance and memory evidence | planned |

## Exit Criteria

### Phase A

- Explicit token separator contract published.
- Explicit depth-window contract published.
- Collision policy documented for token-in-key cases.

### Phase B/C

- Round-trip fixtures cover token separators and deep paths.
- No silent path collapse under token-aware pipelines.

### Phase D/E

- Chunked workflows are deterministic.
- Benchmarks show measurable operator/runtime benefit.

## Discovery

```bash
recur files "main.improvement.17.**" -d docs/
recur files "README.CORE.IMPROVEMENT17" -d ./
recur files "main.command.flatten.separator-token.investigation" -d docs/
```

## Related

- `README.CORE.IMPROVEMENT17.md`
- `README.CORE.IMPROVEMENT15.md`
- `docs/main.command.flatten.separator-token.investigation.md`
