# trace-id: edge_type Field in JSON Output

Status: `complete` (Layer 1 plus Layer 2 passthrough)
Date: 2026-04-05

## What Landed

`edge_type` is present on every site object in `recur trace-id --json` output, and the
same metadata now survives `trace-id --json | merge --stdin --json`.

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

- `TraceIdSite` includes `edge_type: String`
- `result_to_site()` accepts the role string explicitly
- merge JSON intake reads trace-id site objects and carries `edge_type`
- merge JSON output emits leaf `edge_type` arrays after path deduplication

### Tests

- Phase 3b (Layer 1): active and passing
- Phase 5 (Layer 2): active and passing for `trace-id -> merge`

## References

- `src/main_command_trace_id_impl.rs`
- `src/main_command_merge_impl.rs`
- `julia-tests/runtests.trace-id.jl`
- `docs/main.improvement.9.trace-id.complete.md`
- `docs/main.version.a.0.2.8.complete.md`
