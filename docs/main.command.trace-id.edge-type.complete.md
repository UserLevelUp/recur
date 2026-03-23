# trace-id: edge_type Field in JSON Output

Status: `complete` (Layer 1 — Layer 2 is Improvement 9)
Date: 2026-03-08

## What Landed

`edge_type` field added to every site object in `recur trace-id --json` output.
Values: `"define"`, `"produce"`, `"consume"`, `"trigger"`.

### JSON Shape (per site)

```json
{
  "path": "src/Publisher.cs",
  "line_number": 12,
  "line": "  await bus.PublishAsync(Topics.MyEvent);",
  "edge_type": "produce"
}
```

### Rust Changes

- `TraceIdSite` struct: added `edge_type: String` field
- `result_to_site()`: accepts `edge_type: &str` parameter
- `build_record()`: passes correct role string at each classification site
- Serde `Serialize` derive handles JSON output automatically

### Tests

- Phase 3b (Layer 1): 8 assertions passing — `@test_broken` → `@test`
- Phase 5 (Layer 2): remains `@test_broken` — requires `merge --edge-type` (Improvement 9)

## Deferred

Layer 2 pipeline passthrough (`trace-id --json | merge --stdin` retaining `edge_type`)
tracked in `docs/main.improvement.9.trace-id.todo.future-plan.md`.

## References

- `src/main_command_trace_id_impl.rs` — TraceIdSite struct + build_record()
- `julia-tests/runtests.trace-id.jl` — Phase 3b (Layer 1) and Phase 5 (Layer 2)
- `docs/main.improvement.9.trace-id.todo.future-plan.md` — merge edge-type (Layer 2)
- `docs/main.version.a.0.2.8.complete.md` — version record
