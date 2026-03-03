# Reference: Improvement 8 trace-id MVP

## Primary Inputs

- `ulu_docs/recur-agent.static-analysis.md`
- `ulu_docs/recur-agent.trace-id.proposal.md`

## Existing Implementation References

- `src/main_command_id_impl.rs` - identifier pattern scanning entrypoint
- `src/main_command_trace_impl.rs` - command shape, scope/ext/stdin flow, guardrails
- `src/main_command_trace_stats_impl.rs` - structured metrics pipeline patterns
- `src/search.rs` - reusable search primitives and trace graph data structures
- `src/output.rs` - JSON and terminal formatter patterns
- `src/main_command_merge_impl.rs` - current merge behavior and JSON input contracts
- `julia-tests/runtests.id.jl` - identifier command test style
- `julia-tests/runtests.trace.jl` - trace command test style
- `julia-tests/runtests.trace-stats.jl` - command contract and metrics assertions

## How to Study

```bash
cat src/main_command_id_impl.rs
cat src/main_command_trace_impl.rs
cat src/main_command_trace_stats_impl.rs
cat src/search.rs
cat src/output.rs
cat julia-tests/runtests.id.jl
cat julia-tests/runtests.trace.jl
cat julia-tests/runtests.trace-stats.jl
```

## Recommended Approach

1. Start with a standalone command module `main_command_trace_id_impl.rs` that mirrors `id` and `trace` command ergonomics.
2. Implement MVP heuristics in code first (no config file yet).
3. Emit an output shape that can later compose with merge (include node role metadata), but do not add `merge --edge-type` in this phase.
4. Keep the MVP narrow and test-backed; defer cross-lane semantic composition to Improvement 9.
