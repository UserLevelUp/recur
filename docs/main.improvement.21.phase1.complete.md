# Improvement 21 Phase 1: lane scaffold complete

Status: `complete`
Date: 2026-07-15

## Differential slice

```text
E0 = lane contract absent; 21 lane assertions failing
Δ  = scaffold policy + pure recur lane command + scoped capsule generation
E1 = named lane roots are discoverable, idempotent, and revealable
```

Implemented:

- `recur init` emits the `[lanes]` scaffold policy.
- `recur lane <name>` creates a scoped sub-root, nested config, and capsule.
- `recur lane` lists known lane roots and supports JSON output.
- `recur psyche` now reports stale current work, missing last-run receipts
  after a stopped thrust, and orphan work records.

Verification:

- `julia julia-tests/main.command.lane.test.jl` — 38 passed.
- `julia julia-tests/main.recur.psyche.test.jl` — 27 passed.
- `julia julia-tests/runtests.jl` — 1069 passed, 49 expected-broken, 0 failed.

Phase 2 directory-to-prefix projection remains future work.
