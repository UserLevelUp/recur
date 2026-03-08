# trace-stats: Circular Distinct Pattern Count

Status: `todo.current` (active — test written, Rust pending)
Date: 2026-03-08

## Goal

Upgrade the `circular` metric in `recur trace-stats` from a binary presence flag
to a count of distinct circular cycle patterns through a function.

## Current Behaviour

`circular` is 1 if ANY cycle is detected through the function, 0 otherwise.
Two distinct cycles (A→B→A and A→C→A) both report `circular = 1`.

## Target Behaviour

`circular` counts distinct back-edge patterns:
- A→B→A and A→C→A → `circular = 2`
- A→B→A only → `circular = 1`
- No cycle → `circular = 0`

## Test

`julia-tests/runtests.trace-stats.jl` — "count distinct circular patterns"

Fixture: `DistinctCycleService.cs` (added to `runtests.setup.jl`)
```csharp
public class DistinctCycleService {
    public void CycleRoot() { PathA(); PathB(); }
    public void PathA() { CycleRoot(); }  // pattern 1
    public void PathB() { CycleRoot(); }  // pattern 2
}
```

Assertion (currently `@test_broken`):
```julia
@test_broken length(cycle_root) == 1 && Int(cycle_root[1]["circular"]) == 2
```

## Rust Change Required

File: `src/main_command_trace_stats_impl.rs`

Current: cycle detection returns bool (any cycle found)
Target: count distinct back-edges / cycle entry points per function

## Close-out Criteria

1. `@test_broken` flips to passing
2. No regressions in existing circular-only filter tests
3. Delete this `.current.md`

## References

- `julia-tests/runtests.trace-stats.jl` — test at "count distinct circular patterns"
- `julia-tests/runtests.setup.jl` — DistinctCycleService fixture
- `src/main_command_trace_stats_impl.rs` — implementation target
- `docs/main.improvement.7.phase3.todo.current.md` — phase context
