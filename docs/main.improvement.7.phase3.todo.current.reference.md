# Reference: Improvement 7 Phase 3 (trace-stats)

## Primary Spec

- `README.CORE.IMPROVEMENT7.md` - full command contract and implementation outline

## Existing Implementation References

- `src/main_command_trace_stats_impl.rs` - phase 3 command surface + validation bootstrap
- `src/main_command_trace_impl.rs` - trace behavior and call graph traversal patterns
- `src/search.rs` - recursion/stop-reason engine (depth, width, cycle)
- `src/main_command_callers_impl.rs` - caller analysis patterns
- `src/main_command_callees_impl.rs` - callee analysis patterns
- `src/output.rs` - table/json output shaping
- `src/main.rs` - CLI wiring for trace flags and command dispatch
- `julia-tests/runtests.trace-stats.jl` - trace-stats tests (contract active, metrics pending)
- `docs/main.command.trace-stats.cli-surface.complete.md` - phase 3 step 1 completion evidence
- `docs/main.command.trace-stats.metrics.todo.current.md` - active metrics lane

## How to Study

```bash
cat README.CORE.IMPROVEMENT7.md
cat src/main.rs
cat src/main_command_trace_stats_impl.rs
cat src/main_command_trace_impl.rs
cat src/search.rs
cat src/output.rs
cat julia-tests/runtests.trace-stats.jl
```

## Recommended Approach

1. Keep command surface stable while replacing bootstrap payload with computed metrics.
2. Reuse existing trace traversal logic before introducing new graph machinery.
3. Add deterministic stats structs + sorting/filtering with focused unit tests.
4. Add output mode coverage (table/json/csv) after metrics are stable.
