# Command: trace-id JSON Pipeline Composition

Status: `todo`
Date: 2026-04-05

## Goal

Track cross-command JSON composition contracts that feed recur analysis outputs into `merge --stdin`.

## Contract Targets

1. `trace --json` piped to `merge --stdin`.
2. `callers --json` piped to `merge --stdin`.
3. `callees --json` piped to `merge --stdin`.
4. `trace-id --json` piped to `merge --stdin`.

## Current State

- `trace-id --json | merge --stdin --json` is active and passing via
  `docs/main.improvement.9.trace-id.complete.md`.
- The remaining edge-metadata placeholders for `trace`, `callers`, and `callees` stay
  `@test_skip` because those commands do not emit `edge_type`.
- Saved-run persistence coverage now exists separately in Phase 4b; this lane is
  merge-composition only.

## Completion Criteria

1. Keep trace-id composition covered as an active assertion.
2. Treat `trace` / `callers` / `callees` edge-metadata placeholders as permanently
   descoped unless those commands gain `edge_type`.
3. Keep command docs examples aligned with the passing pipeline behavior.

## Related

- `docs/main.command.trace-id.run.todo.current.md`
- `docs/main.command.trace-id.test.todo.current.md`
- `docs/main.improvement.8.trace-id.todo.current.md`
- `docs/main.improvement.9.trace-id.todo.future-plan.md`
- `docs/main.improvement.9.trace-id.complete.md`
