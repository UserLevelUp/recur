# Reference: Improvement 7 Phase 3 (trace-stats)

## Primary Spec

- `README.CORE.IMPROVEMENT7.md` - full command contract and implementation outline

## Existing Implementation References

- `src/main_command_trace_impl.rs` - trace behavior and call graph traversal patterns
- `src/main_command_callers_impl.rs` - caller analysis patterns
- `src/main_command_callees_impl.rs` - callee analysis patterns
- `src/output.rs` - table/json output shaping
- `julia-tests/runtests.trace-stats.jl` - trace-stats test placeholder

## How to Study

```bash
cat README.CORE.IMPROVEMENT7.md
cat src/main_command_trace_impl.rs
cat src/output.rs
cat julia-tests/runtests.trace-stats.jl
```

## Recommended Approach

1. Define minimal `trace-stats` CLI + handler wiring first.
2. Reuse existing trace traversal logic before introducing new graph machinery.
3. Add deterministic stats structs + sorting/filtering with focused unit tests.
4. Add output mode coverage (table/json/csv) after metrics are stable.
