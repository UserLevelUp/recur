# Improvement 30: Static Graph Report v1

Status: `todo.current`
Priority: `active bounded implementation slice`
Parent: `README.CORE.IMPROVEMENT30.md`
Consumes: `main.improvement.30.concurrent-ir.contract.complete`
Date: 2026-07-24
Fixture: `demos/main.lang/main.lang.skippy-watch-coordination.recur`

## Manual Warp

```text
E0(main.improvement.30.static-graph.todo.current)
  -> dE(freeze one deterministic graph report over CIR1)
  -> Ef(main.improvement.30.static-graph.contract.complete)
```

This Eventness transition remains manual. A parser, report, or passing test is
evidence for completion; none may rename its own lifecycle artifact.

## Starting Eventness

`WIR1` and `CIR1` are complete contracts. `CIR1` now provides source-hashed,
source-spanned, typed facts for five lanes, projected WorkOrders, WorkReceipt
ports, one fork, ordered awaits, and downstream consumers. Those facts are
validated locally, but there is no reusable graph report that explains:

```text
which node depends on which producer
which exact messages close each wait gate
whether dependency or wait cycles exist
which nodes are reachable
why the coordination is or is not statically sound
```

Later queries, diagrams, grids, and coordinators must not each answer those
questions differently.

## Goldilocks dE

Freeze one read-only report over an already parsed
`recur-lang-concurrent-ir-v1` value:

- deterministic lane and coordinator-scope nodes;
- exact typed message dependency edges;
- ordered wait gates with required ports and next consumer;
- dependency-cycle and wait-cycle findings;
- unreachable-lane and unsatisfied-join findings;
- source hash, consumed IR schema, and report schema;
- stable finding identities and source-backed evidence where CIR1 provides it;
- deterministic text-independent JSON suitable for later pure projections.

The candidate wire name is:

```text
recur-lang-static-graph-report-v1
```

It is not frozen until this Eventness reaches `contract.complete`.

## Symbolic Graph Vocabulary

These symbols are local mathematical notation. Serialized identities remain
the exact CIR1 names.

```text
G        complete static coordination graph
N_l(x)   lane node named x
N_s(x)   coordinator scope node named x
E_m(p,c) typed message dependency from producer port p to consumer c
W(k)     ordered wait gate k
R(k)     exact required message-port set for W(k)
C(k)     next consumer released by W(k)
D(G)     directed dependency projection
Q(G)     directed wait projection
Reach(G) nodes reachable from the compact flow entry
Sound(G) no blocking static finding in the bounded report
```

Canonical semantic names:

```text
SGR1                         document-local short symbol
recur.lang.static.graph.report.v1
main.improvement.30.static-graph.todo.current
```

The graph report must preserve authored vector order for explanation while
cycle and set comparisons use deterministic canonical ordering.

## Acceptance Criteria

- The analyzer consumes `ConcurrentIr`; it does not parse Recur Lang source.
- The Skippy fixture produces the exact five lane nodes in source order.
- Every lane input message becomes an exact producer-to-consumer dependency.
- The three awaits preserve their required typed ports and downstream
  consumers.
- Dependency and wait projections are deterministic across repeated runs.
- A valid fixture reports no cycle, unreachable lane, or unsatisfied join.
- Focused synthetic graphs prove dependency-cycle, wait-cycle, unreachable,
  and missing-join findings.
- JSON includes `schema`, `ir_schema`, `source_hash`, nodes, edges, waits,
  findings, and `orchestration_sound`.
- Text-independent normalized JSON is byte-equivalent across repeated runs.
- No worker is scheduled, no Eventness is advanced, and no project artifact is
  mutated by analysis.

## Non-Goals

- no new Recur Lang parsing;
- no unification of the 0.1 Warp and 0.2 concurrent grammars in this slice;
- no system, subsystem, import, adapter, watcher, channel, or feedback syntax;
- no write-scope conflict or stale subsystem-import analysis;
- no `recur lang` CLI surface yet;
- no grid snapshot or live renderer;
- no WorkOrder publication or receipt acceptance;
- no scheduler, coordinator loop, merge, or target-language execution.

## Downstream Red-First Fixture

`demos/pathing/main.lang.pathing.recur` and
`julia-tests/main.lang.pathing.test.jl` preserve the larger graph-language
destination: broadcast, explicit execution modes, dynamic scatter, qualified
instances, local graph views, and deterministic path generation.

The active source-shape tests protect that design. Its parser, graph, and
execution assertions are `@test_broken` because those capabilities exceed
`CIR1`. They do not expand this slice: `SGR1` still consumes the accepted
concurrent IR and adds no new parser.

## Final-State Alignment

The imagined final state remains achievable:

```text
one bounded source model
  -> one soundness analysis
    -> many pure explanations and views
      -> one receipt-backed stateful actor
        -> durable Eventness and completed work report
```

The work is on track because `WIR1` established receipt-bound Eventness and
`CIR1` established exact lane communication without prematurely scheduling
anything. `SGR1` is the correct active gate because it turns those facts into
one shared soundness answer.

One architectural risk is now explicit: `WIR1` and `CIR1` are sibling frozen
contracts, not yet one unified coordination AST. This slice must not create a
third parser. Before live coordination, a later contract must define how Warp
Eventness and concurrent lane state compose under one source identity. Keeping
that convergence gate visible is what preserves the final-state direction.

## Exit Evidence

Completion requires:

```text
focused red/green graph tests
full Rust regression suite
focused Julia fixture regression suite
deterministic JSON proof
format and diff hygiene
no new Clippy diagnostics in touched modules
implementation commit captured before manual Eventness rename
```

## Discovery

```powershell
recur tree "main.improvement.30.static-graph" -d docs/
recur files "main.improvement.30.**.todo.current" -d docs/
recur trace-id "recur.lang.static.graph.report.v1" --scope "**" --ext ".md" -d .
recur trace-id "recur.lang.concurrent.ir.v1" --scope "**" --ext ".md" -d .
```

## Trace-Id Lines

```text
defines: main.improvement.30.static-graph current bounded implementation lane for SGR1
defines: recur.lang.static.graph.report.v1 deterministic nodes message edges waits findings and soundness over CIR1
consumes: recur.lang.concurrent.ir.v1 exact typed lane communication graph
consumes: main.improvement.30.concurrent-ir.contract.complete accepted CIR1 Eventness
produces: main.improvement.30.static-graph.contract versioned read-only static graph report
triggers: recur.lang.query.surface pure show check report and diagram projections
triggers: recur.lang.grid.report.v0 pure snapshot over shared graph and durable Eventness
```
