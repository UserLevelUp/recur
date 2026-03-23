# Dogfooding Lane: trace-id on recur itself

## Two-Layer Model (discovered 2026-03-13)

recur's discoverability stack has two distinct layers. Each layer uses different commands:

| Layer | What it traces | Right tool |
|-------|---------------|------------|
| **Code layer** | Function symbols, call graphs | `callers`, `callees`, `trace` |
| **Eventness layer** | Dot-path identifiers in content | `trace-id` |

`trace-id` requires dot-separated identifiers appearing as text in files.
It does NOT find Rust function names — those are code-layer symbols.

## Dogfooding trace-id on the Eventness Layer — TESTED 2026-03-13

**Result: not viable in current form.**

```bash
recur trace-id "main.command.trace-id" --scope "main.**" -d docs/
# → define: [], produce: [], consume: [], trigger: []
```

**Why it returns empty:** The docs file *names* follow the hierarchy (`main.command.trace-id.todo.current.md`)
but the file *content* does not embed cross-references as dot-path identifier strings.
`trace-id` requires the identifier to appear literally in file content — the filename
hierarchy alone is not enough.

**What would make it work:** Doc files would need to reference each other by full
dot-path identifier in their content, e.g.:
```
This lane tracks main.command.trace-id development.
Depends on: main.improvement.8
Triggers: main.command.trace-id.test
```
That is a deliberate authoring convention, not the current practice.

## Right Tool for Docs Gap Analysis

`trace-id` is the wrong tool for docs cross-referencing. The right tools are:
```bash
# Gap: commands with impl but no test
recur files "main_command_*_impl" -d src/ --sep _       # what exists in code
recur files "main.command.*.test" -d julia-tests/       # what has tests
# absence = gap

# Gap: commands with impl but no readme
recur files "main.command.*.readme" -d docs/
```

`recur files` + absence detection IS the dogfooding pattern for docs. `trace-id`
is for tracing dot-path event identifiers through content — it belongs at the
Sudoku/application layer, not the meta/docs layer.

## Status

- [x] Validated: `recur trace-id` on docs/ returns empty — not viable without content changes
- [ ] Decision: adopt explicit cross-reference convention in docs content? (future consideration)
- No further action needed on this lane until that decision is made.
