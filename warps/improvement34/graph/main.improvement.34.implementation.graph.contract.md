# Contextual revisions and bounded relationship graph

artifact.type = contract
defines: main.improvement.34.implementation.graph.contract
consumer: main.improvement.34.revision_dependencies
consumer: main.improvement.34.relationship_graph
consumer: main.improvement.34.compatibility
warp.id = main.improvement.34.implementation.graph
warp.uuid = 01a1ba42-9d93-7bcf-8355-d586dcae1eaa
status = planned; no implementation or acceptance

## Initial Eventness

The first declared relationship and pin must be accepted before adding selection and traversal. The version manifest is a source of identity, not a consumer-constraint graph.

Inspect: Trace bubble handoff, src/main_command_version_impl.rs and docs/main.command.version.readme.md. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: `main.improvement.34.implementation.trace` These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### resolve — Resolve applicable revisions with explicit context

Checked test gate: `resolve-julia` (no skipped checks). Scope current selectors to project, variant and release lane. Test forked revisions, multiple applicable successors, missing targets and supersession cycles. Never use order or timestamps to break a tie.

### traverse — Project and traverse declared relationships

Checked test gate: `traverse-julia` (no skipped checks). Keep semantic IDs, artifacts and revisions distinct. Provide inbound/outbound traversal, stable node and edge identities, source spans and separate discovery/traversal bounds. Test diamond sharing, cycles, unresolved edges, and cross-prefix consumption without a Warp map.

### verify — Verify equivalent graph projections

Checked test gate: `verify-julia` (no skipped checks). Check CLI and JSON against the same declared graph and scope, including truncation and legacy scoped compatibility. Do not turn heuristic occurrences into edges.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.graph.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.graph.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

Contextual dependencies resolve explicitly, and a bounded graph shows their provenance, direction and gaps with equivalent text and JSON semantics.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
