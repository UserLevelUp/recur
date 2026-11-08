# Opted-in expansion and acceptance review

artifact.type = contract
defines: main.improvement.34.implementation.review.contract
consumer: main.improvement.34.eventness.expansion
consumer: main.improvement.34.gate_review
consumer: main.improvement.34.evidence_scope
consumer: main.improvement.34.compatibility
warp.id = main.improvement.34.implementation.review
warp.uuid = 01a1ba42-9dff-764c-a83b-0725c3f64d13
status = planned; no implementation or acceptance

## Initial Eventness

Grounded expansion, exact correction targets and evidence scope must be available. Existing projects have no new review requirement unless they opt in.

Inspect: Eventness and evidence handoffs, companion completion paths and checked evidence rules. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: `main.improvement.34.implementation.eventness`, `main.improvement.34.implementation.evidence` These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### policy — Specify and assess review provenance

Checked test gate: `policy-julia` (no skipped checks). Define explicit opt-in criteria, target identity, exact input binding, author/reviewer provenance and resolved/open findings. Test self-review, stale inputs and changed obligations. Do not treat a second model name or agreement as independence.

### enforce — Apply policy to selected new gates

Checked test gate: `enforce-julia` (no skipped checks). Test expansion and selected acceptance: passing automated tests alone do not satisfy policy; counterexamples block the specific gate; a valid independent review can satisfy it. Preserve old gates and receipts; no automatic dispatch, approval request or global policy.

### verify — Verify adoption and compatibility

Checked test gate: `verify-julia` (no skipped checks). Run opted-in and legacy workflows with matching conditions. Include refusal without corrupting existing evidence, and verify handoff scope remains readable.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.review.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.review.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

Selected review-required expansion or acceptance gates assess independent, exact-input review under explicit policy. An unresolved counterexample stays visible and the associated new gate stays unsatisfied.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
