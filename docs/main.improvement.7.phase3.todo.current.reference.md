# Reference: Improvement 7 Phase 3 (trace-stats)

## Primary Spec

- `README.CORE.IMPROVEMENT7.md` - full command contract and implementation outline

## Existing Implementation References

- `src/main_command_trace_impl.rs` - trace behavior and call graph traversal patterns
- `src/search.rs` - recursion/stop-reason engine (depth, width, cycle)
- `src/main_command_callers_impl.rs` - caller analysis patterns
- `src/main_command_callees_impl.rs` - callee analysis patterns
- `src/output.rs` - table/json output shaping
- `src/main.rs` - CLI wiring for trace flags and command dispatch
- `src/main_command_init_impl.rs` - existing `--force` pass-through pattern
- `src/project_config.rs` - existing `if exists && !force` guard gate
- `julia-tests/runtests.trace-stats.jl` - trace-stats test placeholder
- `julia-tests/runtests.trace.jl` - `trace --force` placeholder tests

## How to Study

```bash
cat README.CORE.IMPROVEMENT7.md
cat src/main.rs
cat src/main_command_trace_impl.rs
cat src/search.rs
cat src/output.rs
cat julia-tests/runtests.trace-stats.jl
cat julia-tests/runtests.trace.jl
```

## Recommended Approach

1. Define minimal `trace-stats` CLI + handler wiring first.
2. Reuse existing trace traversal logic before introducing new graph machinery.
3. Reuse the existing `--force` wiring model from `init` for trace safety bypass.
4. Add deterministic stats structs + sorting/filtering with focused unit tests.
5. Add output mode coverage (table/json/csv) after metrics are stable.
