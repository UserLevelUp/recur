# Grounded Eventness and contract revision

artifact.type = contract
defines: main.improvement.34.implementation.eventness.contract
consumer: main.improvement.34.eventness.expansion
consumer: main.improvement.34.eventness.collapse
consumer: main.improvement.34.contract_revision
consumer: main.improvement.34.attention
consumer: main.improvement.34.skill_guidance
warp.id = main.improvement.34.implementation.eventness
warp.uuid = 01a1ba42-9db6-714c-98e7-8e8b4e43b57e
status = planned; no implementation or acceptance

## Initial Eventness

Existing Eventness and archive controls provide lifecycle context. Successor outlines need a grounded baseline; revision decisions and compact handoffs currently rely on author practice.

Inspect: Trace and workflow handoffs, existing recur-warp receipt/evolve/collapse commands and Eventness guidance. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: `main.improvement.34.implementation.trace`, `main.improvement.34.implementation.workflow` These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### revision — Record the contract revision decision

Checked test gate: `revision-julia` (no skipped checks). Exercise editorial clarification, new evidence under unchanged criteria, changed dependencies or limits and split responsibility. Preserve stable identities when appropriate and prior receipts always; avoid reusing historical acceptance under changed criteria.

### expand — Ground and expand a successor outline

Checked test gate: `expand-julia` (no skipped checks). Record existing artifacts and revisions, available predecessor inputs, deliberate unknowns, interface deltas and reassessment. Test stale or missing predecessor evidence and a review request bound to the exact expansion. Final enforcement belongs to review.

### handoff — Collapse attention into a recoverable handoff

Checked test gate: `handoff-julia` (no skipped checks). Keep exact sources, rationale, open findings and correction references where present. Verify fresh-root recovery, explicit unavailable/private evidence, and affected-only re-expansion. Compact summaries do not archive or rewrite receipts.

### verify — Verify a full expand-to-handoff lifecycle

Checked test gate: `verify-julia` (no skipped checks). Check immutable predecessor evidence and compatible archive behavior. Publish the lineage and review request contract for the evidence and review bubbles.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.eventness.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.eventness.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

A compact outline expands from real predecessor inputs, material changes get explicit revision decisions, and a portable handoff can recover exactly the referenced work. Independent enforcement is assigned to the review bubble.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
