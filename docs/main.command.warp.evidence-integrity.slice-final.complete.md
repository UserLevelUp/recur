# Slice final: Completed version and integrated acceptance

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: All seven issues integrated
Contract: contract:main.command.warp.evidence-integrity.slice-final:v1
Dependencies: slice-0, slice-1, slice-2, slice-3, slice-4, slice-5, slice-6, slice-7

## Desired result and acceptance

- All preceding slices have current accepted contracts and their required evidence; no pending, blocked, stale, conflicting, or failed mandatory gate remains.
- Replay one external-project scenario: inspect policy, diagnose .verified, create a compatible receipt, attach build/test evidence, validate the full map, and reconcile reveal.
- Run the matching negative scenarios: bare complete marker, zero tests, failed build/test, disallowed skips, missing result artifact, source drift, missing prerequisite, contract mismatch, and stale summary.
- Run cargo test and julia julia-tests/runtests.jl against the final source; report existing expected-broken/ignored cases explicitly and allow no new unexplained regression.
- Record exact commands, source binding, outcomes and artifact paths. A fresh session can rediscover the map, baseline, remaining/accepted work and evidence through reveal/tree/files/trace-id.
- Only after integrated verification, collapse this Warp's active markers, retain baseline/report/decisions, publish final completion evidence, and update reveal readiness.

## Implementation and verification scope

affected Rust/Julia suites; docs; versioned maps/layers and evidence fixtures.

Required evidence gates:

- `integrated-external-project-scenario`
- `cargo-regression`
- `julia-regression`
- `fresh-evidence-and-recovery`
- `eventness-closeout`

Each gate must link exact commands or inspection procedure, observed outcome,
source revision/fingerprint, and retained result artifacts. Use existing suites
and add meaningful cases for changed behavior. A documentation-only resolution
is valid only when existing behavior is demonstrated to satisfy the requirement.

## Observed state and next action

Implemented and regression-tested. The retained verification record below documents the completed result; no repair work remains in this bubble.

## Trace identities

```text
defines: recur.warp.evidence.integrity.slice.final Completed version and integrated acceptance
consumes: recur.warp.evidence.integrity.slice.0 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.1 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.2 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.3 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.4 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.5 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.6 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.slice.7 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

All seven repairs passed integrated and regression checks: 179 Rust tests and 2,400 Julia assertions; 73 expected broken Julia cases and seven ignored Rust documentation tests remain. Focused evidence-integrity suite: 108 passed. Exact commands, limitations and tested source SHA256 are retained in main.command.warp.evidence-integrity.verification.reference.md. Final acceptance uses declared references to these reviewed records, not a claim of machine-checked external receipts.
