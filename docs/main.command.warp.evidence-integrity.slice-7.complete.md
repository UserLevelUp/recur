# Slice 7: Reconcile reveal summaries with receipts

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 7 (medium)
Contract: contract:main.command.warp.evidence-integrity.slice-7:v1
Dependencies: slice-6

## Desired result and acceptance

- Templates separate desired final behavior from observed results and identify authoritative receipt/map references.
- Warn when structured readiness still points to Slice 1 although all required slices are recorded complete; report referenced receipts and evidence state.
- Warn when a completion summary contradicts pending, stale, failed, or missing mandatory evidence.
- Freeze explicit structured fields for reliable checks; report unsupported prose as unassessed rather than pretending to understand all narrative.
- Propose edits or regenerate clearly owned fields only through an authorized writer. Read-only checks preserve human rationale and baseline history.

## Implementation and verification scope

Reveal/Warp pure query implementations; lane templates; Rust/Julia reconciliation fixtures.

Required evidence gates:

- `summary-reconciliation-tests`
- `authored-intent-preservation-tests`

Each gate must link exact commands or inspection procedure, observed outcome,
source revision/fingerprint, and retained result artifacts. Use existing suites
and add meaningful cases for changed behavior. A documentation-only resolution
is valid only when existing behavior is demonstrated to satisfy the requirement.

## Observed state and next action

This slice is implemented or baseline-audited. See the verified outcome below.
Acceptance criteria above are retained as the historical contract. Final suite
verification and integration closeout are recorded separately in Slice Final.

## Trace identities

```text
defines: recur.warp.evidence.integrity.slice.7 Reconcile reveal summaries with receipts
consumes: recur.warp.evidence.integrity.slice.6 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Explicit warp.id/warp.root, observed.state and readiness.slice enable read-only reveal reconciliation. Tests flag stale readiness and unsupported verified claims after source drift while preserving authored narrative bytes. Integrated evidence/reconciliation testset 44/44; broader integrity suite 96/96 passes. Final regression and closeout remain.
