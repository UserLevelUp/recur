# trace-id: edge_type Field in JSON Output

Status: `todo.current` (Layer 1 complete — Layer 2 deferred to Improvement 9)
Date: 2026-03-08

## Goal

Add `edge_type` field to each site object in `recur trace-id --json` output
so downstream tools (merge, callers, custom parsers) can read the role
of each occurrence without re-classifying it.

## Current JSON Shape (per site)

```json
{
  "path": "src/DotWatcherHostedService.cs",
  "line_number": 4,
  "line": "  registry.Register(... PublishAsync(DotControlTopics.OwnershipCreate));"
}
```

## Target JSON Shape

```json
{
  "path": "src/DotWatcherHostedService.cs",
  "line_number": 4,
  "line": "  registry.Register(... PublishAsync(DotControlTopics.OwnershipCreate));",
  "edge_type": "produce"
}
```

`edge_type` values: `"define"`, `"produce"`, `"consume"`, `"trigger"`

## Tests

Two layers of tests (both currently `@test_broken`):

### Layer 1: Direct — Phase 3b in `runtests.trace-id.jl`
Checks that `recur trace-id --json` output sites include `edge_type`:
```julia
@test_broken haskey(first_site, "edge_type")
@test_broken String(get(first_site, "edge_type", "")) == role
```

### Layer 2: Pipeline — Phase 5 in `runtests.trace-id.jl`
Checks that `trace-id --json | merge --stdin` output retains `edge_type`.
This is Improvement 9 scope — requires merge to pass through edge metadata.
Keep as `@test_broken` until merge edge-type support lands.

## Rust Change Required

File: `src/main_command_trace_id_impl.rs`

`TraceIdSite` struct: add `edge_type: String` field
`build_record()`: populate `edge_type` when classifying each site
JSON serialization: include `edge_type` in output

## Close-out Criteria (Layer 1 only — this lane)

1. Phase 3b `@test_broken` tests flip to passing
2. Phase 5 pipeline tests remain `@test_broken` (Improvement 9 scope)
3. No regressions in Phase 1-4 passing tests
4. Delete this `.current.md`

## References

- `julia-tests/runtests.trace-id.jl` — Phase 3b (Layer 1) and Phase 5 (Layer 2)
- `src/main_command_trace_id_impl.rs` — TraceIdSite struct + build_record()
- `docs/main.improvement.8.trace-id.todo.current.md` — improvement 8 context
- `docs/main.improvement.9.trace-id.todo.future-plan.md` — merge edge-type (Layer 2)
