# Command: trace-stats Metrics Pipeline

Status: `todo.current` (active)
Date: 2026-02-16

## Goal

Implement the Phase 3 metrics engine behind `recur trace-stats`:

- direct callees
- transitive callees
- circular pattern count
- max depth
- risk classification

## Scope

1. Discover analyzable functions within scope/ext/stdin filters.
2. Reuse trace traversal to compute per-function stats.
3. Add sorting/filtering/top-N on computed metrics.
4. Replace bootstrap placeholder outputs with real table/json/csv data.
5. Expand Julia tests from contract-only to metric assertions.

## References

- `README.CORE.IMPROVEMENT7.md`
- `src/main_command_trace_stats_impl.rs`
- `src/search.rs`
- `src/output.rs`
- `julia-tests/runtests.trace-stats.jl`

## Discovery

```bash
recur files "main.command.trace-stats.**" -d docs/
recur tree "main.command.trace-stats" -d docs/
```
