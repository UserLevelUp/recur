# Improvement 9: trace-id Deferred Scope

Status: `todo.future-plan`
Date: 2026-03-03

## Purpose

Track post-MVP `trace-id` work that is intentionally deferred while Improvement 8 delivers the build-now command.

## Deferred Items

1. Project-custom heuristic configuration:
   - `.recur/trace-id.toml`
   - load-order strategy (project-local, user-level, defaults)
2. Merge semantic edge typing:
   - `merge --edge-type`
   - lane-aware joins for call/route/config composition
3. Rich multi-lane composition workflow:
   - `trace` + `trace-id` + `flatten` merged into a single semantic graph view
4. Additional performance and false-positive hardening across large codebases.

## Hand-off from Improvement 8

Prerequisite:
- `docs/main.improvement.8.trace-id.complete.md` exists with MVP completion evidence.

Inputs from MVP:
- command JSON schema shape
- built-in heuristic behavior
- test fixtures and known edge cases

## Related

- `docs/main.improvement.8.trace-id.todo.current.md`
- `docs/main.improvement.8.todo.future-plan.md`
- `docs/main.improvement.9.todo.future-plan.md`
