# Corrections and evidence scope

artifact.type = contract
defines: main.improvement.34.implementation.evidence.contract
consumer: main.improvement.34.correction_records
consumer: main.improvement.34.evidence_scope
consumer: main.improvement.34.attention
consumer: main.improvement.34.compatibility
warp.id = main.improvement.34.implementation.evidence
warp.uuid = 01a1ba42-9ddd-727e-989d-f6794359f832
status = planned; no implementation or acceptance

## Initial Eventness

A legacy accepted layer can conflict with a new result hash. Existing refresh renews eligible freshness only. No general structured correction or qualified scope view is in use.

Inspect: Eventness handoff, src/warp_evidence.rs, src/warp_refresh.rs, existing layer and refresh formats. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: `main.improvement.34.implementation.eventness` These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### findings — Read scoped finding and correction sidecars

Checked test gate: `findings-julia` (no skipped checks). Bind the accepted artifact/layer digest, criterion, revision, counterevidence, optional replacement and provenance. Display original and correction together; report missing targets, cycles and competing replacements. A finding against R06 does not apply to R07.

### scope — Project separate evidence dimensions

Checked test gate: `scope-julia` (no skipped checks). Show accepted/declared/checked, freshness, method and envelope, review, and domain-specific validation independently. Distinguish stale, failed, unknown, unperformed and not applicable. A design-only display cannot waive a preexisting physical-test gate.

### verify — Verify the correction handoff without rewriting history

Checked test gate: `verify-julia` (no skipped checks). Round-trip a compact handoff with an open finding and correction to a fresh fixture. Assert original layer bytes, result hash conflicts, same-contract refresh restrictions and legacy views remain intact. Publication affecting effective acceptance is deferred until explicitly specified.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.evidence.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.evidence.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

An opt-in view links findings and corrections to exact historical claims and reports evidence scope separately. Accepted records and their original state remain intact.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
