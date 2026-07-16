# main.improvement.pre27and28.todo.current

Status: `todo.current`
Date: 2026-05-23

## Purpose

Rank the work that should be handled before returning to Improvement 27 warp
implementation or expanding Improvement 28 beyond its seed query surface.

This lane separates true gates from old future-plan residue.

## Ranked Queue

### 0. Release And Package Hygiene

Lane: `docs/main.package.crates-io.todo.current.md`

Why first: public command surface should not drift while the crate package lane
is active. Package verification currently passes, but publish and fresh install
verification remain open.

Next actions:

- confirm version metadata and README install text
- decide whether this window actually publishes to crates.io
- if publishing, run fresh install verification afterward
- collapse the package lane into recurring or complete when done

### 1. Improvement 28 Docs Bridge

Lane: `docs/main.improvement.28.complete.md`

Why first: the seed `.recur-*` capability-card surface is already implemented,
tested, and useful, but it needed to appear in the docs-side improvement tree.

Status: done for the seed query surface.

Remaining future work:

- optional card authoring or repair command
- optional richer card schema

### 2. Improvement 26 Criteria Coverage

Lane: `docs/main.improvement.26.test.todo.criteria-coverage.md`

Why before 27: warp status wants to score governance, evidence, blockers, and
version-like state. Improvement 26 is the closest existing contract for those
rules. Synthetic coverage should be stronger before warp turns those ideas into
a verdict.

Next actions:

- create generic fixtures for versioned operational artifacts
- prove `recur version` and `recur-version` behavior without private data
- keep operator-confirmation and privacy behavior explicit

### 3. Trace-Id Saved-Run Policy

Lane: `docs/main.command.trace-id.run.todo.current.md`

Why before 27: warp will consume trace-id evidence. The first read-only warp
slice can work with current trace-id output, but saved-run freshness/history
policy should be settled before warp relies on cached evidence.

Next actions:

- decide `latest` only vs timestamped `history/`
- decide metadata freshness vs optional content hash
- collapse the lane into recurring or future-plan when policy is explicit

### 4. Improvement 25 Topology Bridge

Source: `README.CORE.IMPROVEMENT25.md`

Why before deeper 27: Improvement 27 consumes Improvement 25 as reveal-lane and
vault-topology context, but there is no docs-side `main.improvement.25` bridge.

Next action:

- add a small `docs/main.improvement.25.todo.future-plan.md` bridge if warp
  scoring starts using topology terms directly

### 5. Improvement 27 Contract Fixtures

Lane: `docs/main.improvement.27.contract.warp-status-v1.todo.future-plan.md`

Why last: this is not before 27; it is the first real 27 implementation gate.
Only start this after the package lane is calm and the 26/trace-id policy
questions are either closed or explicitly deferred.

Next actions:

- freeze `warp-status-v1`
- add synthetic optimum, sub_optimum, blocked, and config override fixtures
- implement read-only `recur warp status` only after fixtures are clear

## Not Blocking 27 Or 28

These lanes can stay visible without blocking the warp/capability path:

- `docs/main.command.watch.cli-art.todo.current.md`
- demo lanes under `docs/main.demo.*.todo.current.md`
- old future-plan lanes for Improvements 14, 15, 17, 18, 19, 20, 21, and 22

## Discovery

```powershell
recur files "**.current" -d docs/
recur tree "main.improvement" -d docs/
recur files "main.improvement.27.**" -d docs/
recur files "main.improvement.28.**" -d docs/
recur capability doctor -d .
```

## Trace-Id Lines

```text
defines: main.improvement.pre27and28 ranked active gate list before warp implementation and capability expansion
consumes: main.package.crates-io active release package lane for public install readiness
consumes: main.improvement.28 complete seed capability-card query surface for root .recur-* files
consumes: main.improvement.26 version governance and criteria coverage before warp scoring
consumes: recur.trace-id.saved-runs saved-run persistence and freshness policy before cached warp evidence
consumes: main.improvement.25 reveal lanes and topology over the vault as conceptual input
triggers: main.improvement.27.contract.warp-status-v1 freeze warp status contract after prerequisites settle
triggers: recur.warp.status.implementation return to Rust only after contract fixtures and evidence policy are stable
```
