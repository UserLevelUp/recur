# Improvement 30: Recur Lang Coordination Contracts

Status: `todo.future-plan` (umbrella proposal; live-grid sub-lane is current)
Date: 2026-07-24

## Objective

Keep the Recur Lang coordination capability visible in the Eventness tree
without prematurely turning it into a universal compiler or workflow engine.

The capability should let Recur describe, inspect, and gradually coordinate
bounded lanes of human or machine intelligence:

```text
i(a) -> f(a) -> o(b)
```

It should make contracts, dependencies, waits, write scopes, evidence, cycles,
and integration state easy to query and difficult to accidentally ignore.

## Canonical Proposal

- `README.CORE.IMPROVEMENT30.md`
- `docs/main.improvement.30.live-grid.todo.current.md`

## Current Posture

- The Julia `main.lang` implementation is a language and algorithm spike.
- The source format demonstrates explicit input/output roles, exact shared
  contracts, compact/expanded functions, async lanes, Eventness, and Warp.
- The canonical proposal now defines hierarchical subsystem contraction:
  an accepted child becomes one versioned public block, while child readiness
  and parent integration remain separate Eventness facts.
- The Skippy watch-coordination fixture and formal v0 contract now capture the
  intended compact flow, watcher topology, lane state machine, work orders,
  receipts, joins, and bounded repair loop.
- It does not yet provide production lane coordination, durable receipts,
  target-language compilation, or static circular-reference reports.
- The umbrella remains incremental. The living master work report is now an
  active focused cursor so its schema and snapshot can be tackled without
  activating every future Recur Lang capability.

## Product Boundary

```text
recur lang   = pure query, diagram, static check, report, explanation
recur-lang   = coordination actor, lane state, receipt validation, ACK/NAK
workers      = humans or intelligences that write project code and artifacts
toolchains   = external compilers, test runners, linters, CI, and benchmarks
```

Recur Lang validates orchestration soundness. External evidence is required
for claims about C#, Rust, Angular, React, MVC, CSHTML, or other implementation
correctness.

## Active Focused Pull

The current important update is:

- `docs/main.improvement.30.live-grid.todo.current.md`

Freeze the master-report cell/schema and pure snapshot before building a
continuous display. The grid must reconstruct from durable Eventness and
collapse into the completed audit report without becoming a second state store.

## Foundational Static Pull

Do not start with a compiler backend. When this improvement becomes active,
the first focused contract should be:

1. freeze a small versioned graph/diagnostic representation;
2. detect accidental dependency and wait cycles;
3. generate a source-hashed report;
4. expose the result through a pure text and JSON query;
5. validate one exact versioned subsystem import without executing it;
6. prove it with focused Julia fixtures.

An example requested footer:

```text
footer {
  check circular_ref
  report coordination
}
```

An example generated result:

```text
report {
  orchestration_sound: true
  circular_ref: false
  circular_refs: []
}
```

## Dogfooding Direction

Use a later slice on one real Recur Rust algorithm or validation problem:

- one lane characterizes current behavior;
- one lane proposes the Rust change;
- one lane supplies boundary and property tests;
- Julia may serve as an independent reference;
- an external environment runs Cargo;
- Recur Lang consumes the receipts and coordinates independent verification.

That experiment should determine whether the formalism actually reduces drift
and integration cost.

## Discovery

```powershell
recur tree "main.improvement.30" -d docs/
recur files "README.CORE.IMPROVEMENT30" -d ./
recur files "main.lang.**" -d . --sep . --sep _
recur files "main.command.lang.**" -d docs/
```

## Related

- `README.CORE.IMPROVEMENT30.md`
- `recur_language_start.md`
- `docs/main.lang.readme.md`
- `docs/main.command.lang.readme.md`
- `README.CORE.EVENTNESS.md`
- `docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`
- `docs/main.improvement.30.live-grid.todo.current.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `demos/main.lang/main.lang.algorithm-lab.recur`
- `demos/main.lang/main.lang.skippy-watch-coordination.recur`
- `julia-tests/main.lang.test.jl`

## Trace-Id Lines

```text
defines: main.improvement.30.todo future-plan bridge for Recur Lang coordination contracts
defines: recur.lang.control-plane pure static orchestration and progressive disclosure model
defines: recur-lang.coordinator companion lane state receipt validation and ACK/NAK actor
defines: recur.lang.subsystem.composition accepted child models contract into versioned blocks with separate parent integration Eventness
consumes: README.CORE.IMPROVEMENT30 canonical Recur Lang coordination proposal
consumes: main.recur.purity.decision core recur query and companion actor split
produces: main.improvement.30.discovery Recur queries for the incremental capability and active live-grid cursor
triggers: main.improvement.30.contract future versioned coordination IR and JSON schema
triggers: main.improvement.30.static-analysis future cycle reachability join and lane-scope report
triggers: main.improvement.30.dogfooding future Recur Rust algorithm validation lane
```
