# trace-stats: Circular Distinct Pattern Count

Status: `complete`
Date: 2026-03-08

## Goal

Upgrade the `circular` metric in `recur trace-stats` from a binary presence flag
to a count of distinct circular cycle patterns through a function.

## What Landed

`visit_key` uses `function:path:line` so each distinct back-edge through a root
function is counted separately. No additional Rust change was required — the
existing traversal logic already produced distinct counts once the key was verified.

Fixture `DistinctCycleService.cs` (two distinct back-edges → `circular = 2`) confirmed
passing. Test upgraded from `@test_broken` → `@test`.

## References

- `julia-tests/runtests.trace-stats.jl` — "count distinct circular patterns"
- `julia-tests/runtests.setup.jl` — DistinctCycleService fixture
- `src/main_command_trace_stats_impl.rs` — traversal implementation
- `docs/main.version.a.0.2.8.complete.md` — version record
