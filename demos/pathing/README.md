# Recur Lang Pathing Demo

Status: red-first design fixture

`main.lang.pathing.recur` specifies a deterministic Pac-Man-like path
generator. It is intentionally ahead of the current Recur Lang parsers and
runtime.

The demo exercises two different parallel forms:

```text
one MazeGraph
  -> distance + dead-end + symmetry + coverage scorers

one PowerPlan
  -> one route and breadcrumb branch per power node
```

The formal source separates:

- `header`: contracts, function ports, aliases, fan-out, scatter, and joins;
- `body`: algorithm bindings and non-executable memos;
- `footer`: static checks, reports, Eventness, and the desired Warp.

## Current Test Posture

Run the isolated executable specification:

```powershell
julia --startup-file=no julia-tests/main.lang.pathing.test.jl
```

Source-shape tests pass now. Unsupported parser, graph, local-view, and
execution contracts use Julia's `@test_broken` convention. They remain visible
roadmap debt without making the established regression suite falsely red.

When a capability lands, its `@test_broken` assertion should become a normal
`@test`. An unexpected pass is therefore a signal to rotate that contract
forward rather than leave it marked as missing.

## Intended Implementation Order

1. Parse the 0.3 contracts, reusable scopes, and graph block.
2. Freeze broadcast, execution-mode, scatter, qualified-instance, and join IR.
3. Produce static and one-function-local graph views.
4. Bind deterministic Julia pathing algorithms.
5. Let `recur-lang` coordinate execution only after the read-only contracts are
   stable.

## Discovery

```powershell
recur files "main.lang.pathing" -d demos/pathing/
recur trace-id "recur.lang.pathing.graph" --scope "**" -d .
recur trace-id "demo.pathing.tests" --scope "**" -d .
```

## Trace-Id Lines

```text
defines: demo.pathing red-first Recur Lang fan-out scatter join and graph fixture
defines: recur.lang.pathing.graph contract-addressed parallel Pac-Man-like path generation
consumes: recur.lang.concurrent.ir.v1 exact typed message and await conventions
produces: demo.pathing.tests Julia executable specification for future parser graph and runtime behavior
triggers: recur.lang.static.graph.report.v1 directed and local graph projections
```
