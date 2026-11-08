# Trace discovery, declarations and pinned revisions

artifact.type = contract
defines: main.improvement.34.implementation.trace.contract
consumer: main.improvement.34.discovery
consumer: main.improvement.34.declarations
consumer: main.improvement.34.revision_dependencies
consumer: main.improvement.34.compatibility
warp.id = main.improvement.34.implementation.trace
warp.uuid = 01a1ba42-9d49-7012-b9c1-7b956e58aa54
status = planned; no implementation or acceptance

## Initial Eventness

Today trace-id requires scope and reports heuristic roles. The Julia baseline exposes incidental producer and descendant consumer hits without treating them as required behavior. There is no strict declaration or revision expectation evaluator.

Inspect: src/main_command_trace_id_impl.rs, src/trait/trace_id.rs, src/main_command_version_impl.rs and julia-tests/main.improvement.34.baseline.test.jl. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: None beyond reviewing the baseline. These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### discover — Find paths without requiring scope

Checked test gate: `discover-julia` (no skipped checks). Unscoped lookup lists deduplicated matching files and parent folders in stable order. No lines, roles or expansion. Explicit scope (including `**`) preserves detailed legacy output. Exercise narrow roots, extension exclusions, empty matches, discovery limits and detail-only flags.

### declare — Parse exact opt-in declarations

Checked test gate: `declare-julia` (no skipped checks). Choose one delimited Markdown form and define exact identity, case, roles, spans and policy authority before implementation. Prose, fenced examples and a generator string remain references; longer IDs stay distinct. Diagnose malformed characters and duplicate authority in context. Retain legacy JSON.

### pin — Assess the first revision expectation

Checked test gate: `pin-julia` (no skipped checks). Bind explicit family, expected revision, actual revision, artifact reference and byte digest independently. Both R04 and R06 hashes match; wrong R04 binding for an R06 pin fails revision, intentional R04 history passes, byte drift fails integrity, and unavailable targets are unknown.

### verify — Verify the smallest end-to-end relationship

Checked test gate: `verify-julia` (no skipped checks). Run the discovery-to-scope-to-pin case on synthetic artifacts; compare existing JSON outputs and verify no query mutation. Publish the declaration format and pin result contract for downstream bubbles.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.trace.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.trace.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

A path-only hunt leads to a scoped inspection. One structured relationship distinguishes matching bytes from an incorrect revision; intentional historical pins pass. Existing scoped clients retain their interface.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
