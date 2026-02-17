# Improvement 15: Unflatten and Recomposition Profiles

Status: `todo.future-plan` (long-distance backlog, contracts frozen, implementation deferred)

## Lane Policy

Improvement 15 is parked by design.

- Do not open a `todo.current` lane for Improvement 15 yet.
- Keep work limited to contract/docs maintenance.
- Active implementation priority remains Improvement 7 Phase 3.

## Objective

Freeze the minimum contracts now so implementation can happen later without design drift:

1. canonical flat record schema (`path`, `value`, `kind`)
2. `merge --format flat` contract
3. `unflatten` command contract (MVP)

## Current Eventness Snapshot (2026-02-16)

Completed now:

- Phase A contract freeze is complete.
- Frozen v1 contracts are documented and cross-linked.
- Future-state demo scaffold exists: `demos/ascii-drinks/demo15.sh`.
- Contract tests were added as intentionally broken:
  - `julia-tests/main.command.unflatten.test.jl`
  - `julia-tests/runtests.unflatten.jl`

Current state:

- Implementation remains deferred.
- Contracts are now the source of truth for future work.
- This branch is explicitly parked as long-distance.

## Frozen v1 Contract Set

- `docs/main.improvement.15.contract.flat-record.v1.md`
- `docs/main.command.merge.flat-format.contract.v1.md`
- `docs/main.command.unflatten.contract.v1.md`

## Why Freeze Now

- Keeps future implementation aligned across branches and sessions.
- Lets demo specs (for example `demos/ascii-drinks/demo15.sh`) target a stable CLI/data shape.
- Reduces rework when `unflatten` is finally implemented.

## Implementation Order (When Work Starts)

| Phase | Name | Outcome | Status |
|------|------|---------|--------|
| A | Contract Freeze | v1 docs locked and referenced | **complete** |
| B | Merge Flat Output | `recur merge --format flat` preserves flat records | planned |
| C | Unflatten MVP | `recur unflatten` for `text|json` with deterministic conflict handling | planned |
| D | Profile Layering | Base/variant profile inheritance wired to `unflatten` | planned |
| E | Frame Rendering | `--frames`/`--frame-key` for animation-style output | planned |
| F | Full Round-trip | `flatten -> merge(flat) -> unflatten` validated with fixtures | planned |

## Exit Criteria by Phase

### Phase B

- `merge --format flat` accepted and documented.
- path/value/kind retained (not path-only projection).
- input source precedence is deterministic.

### Phase C

- `unflatten` command added to CLI.
- supports stdin and file input.
- supports `--format text|json`.
- supports `--on-conflict` with deterministic defaults.

### Phase C Scoped Checklist (Do Not Start Yet)

- add `Unflatten` variant to `Commands` in `src/main.rs`
- add `src/main_command_unflatten_impl.rs` execution module
- parse flat-record JSON array input from file/stdin
- implement conflict handling: `error|last-wins|first-wins|array`
- implement deterministic sort: `path|input`
- implement `--format json` materialization path
- implement `--format text` deterministic `path = value` output
- add CLI/help and readme docs for command surface
- convert core `@test_broken` unflatten tests to passing

### Phase D

- multiple `--profile` files allowed.
- predictable override order (later profile wins).
- unknown profile keys can be rejected in strict mode.

### Phase E

- frame grouping works from flat records.
- frame ordering is deterministic.
- demo loop can consume output with minimal shell glue.

### Phase F

- round-trip fixtures pass for standard cases.
- docs updated to show native recur pipeline.
- `demo15.sh` moves from expected-fail to runnable.

## Discovery

```bash
recur files "main.improvement.15.**" -d docs/
recur files "main.command.unflatten.**" -d docs/
recur files "main.command.merge.flat-format.**" -d docs/
recur files "README.CORE.IMPROVEMENT15" -d ./
```

## Related

- `README.CORE.IMPROVEMENT15.md`
- `README.CORE.IMPROVEMENT12.md`
- `demos/ascii-drinks/demo15.sh`
