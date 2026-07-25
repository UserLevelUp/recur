# Improvement 30: Recur Lang Coordination Contracts

Status: `todo.future-plan` (umbrella proposal; static-graph sub-lane is current)
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
- `docs/main.improvement.30.static-graph.todo.current.md`
- `docs/main.improvement.30.live-grid.todo.tracking.md`

## Current Posture

- The Julia `main.lang` implementation is a language and algorithm spike.
- `recur-lang-warp-ir-v1` is complete for one receipt-backed 0.1 Warp.
- `recur-lang-concurrent-ir-v1` is complete for the read-only Level 0/1 lane,
  message, fork, and await boundary in the 0.2 design fixture.
- The source format demonstrates explicit input/output roles, exact shared
  contracts, compact/expanded functions, async lanes, Eventness, and Warp.
- The canonical proposal now defines hierarchical subsystem contraction:
  an accepted child becomes one versioned public block, while child readiness
  and parent integration remain separate Eventness facts.
- The Skippy watch-coordination fixture and formal v0 contract now capture the
  intended compact flow, watcher topology, lane state machine, work orders,
  receipts, joins, and bounded repair loop.
- It does not yet provide production lane coordination, durable multi-lane
  receipts, target-language compilation, or static circular-reference reports.
- The umbrella remains incremental. The static graph report is the active
  focused cursor. The living master work report remains tracked as the product
  destination without activating its snapshot or live view prematurely.

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

The single current implementation update is:

- `docs/main.improvement.30.static-graph.todo.current.md`

Freeze `SGR1` as one deterministic, read-only dependency and wait report over
the accepted concurrent IR. It must expose shared soundness facts without
reparsing source, scheduling lanes, or advancing Eventness.

The downstream product destination remains:

- `docs/main.improvement.30.live-grid.todo.tracking.md`

The grid returns to `todo.current` only after `SGR1` and the shared pure query
projection exist.

## Foundational Static Pull

Do not start with a compiler backend or a coordinator loop. The active focused
contract will:

1. consume `recur-lang-concurrent-ir-v1` without reparsing source;
2. freeze a small versioned graph/report representation;
3. detect dependency cycles, wait cycles, unreachable lanes, and unsatisfied
   joins;
4. generate a deterministic source-hashed JSON report;
5. preserve exact nodes, messages, waits, and downstream consumers;
6. prove accepted and rejected graphs with focused fixtures.

Subsystem imports are not present in `CIR1` and remain a later bounded
extension rather than being smuggled into this slice.

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
- `docs/main.improvement.30.static-graph.todo.current.md`
- `docs/main.improvement.30.live-grid.todo.tracking.md`
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
produces: main.improvement.30.discovery Recur queries for the active static-graph cursor and tracked live-grid destination
triggers: main.improvement.30.contract future versioned coordination IR and JSON schema
triggers: main.improvement.30.static-graph.todo.current cycle reachability join and wait report over CIR1
triggers: main.improvement.30.dogfooding future Recur Rust algorithm validation lane
```
