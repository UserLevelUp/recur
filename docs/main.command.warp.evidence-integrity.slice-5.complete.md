# Slice 5: Structured external evidence and freshness

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 6 (medium)
Contract: contract:main.command.warp.evidence-integrity.slice-5:v1
Dependencies: slice-4

## Desired result and acceptance

- Freeze a versioned evidence format with kind, producer, project/configuration/platform, timestamp, result path, outcomes, test counts, and source binding.
- Support clean revision and explicit dirty-tree content fingerprints; define what files/configuration the fingerprint covers and how it is recomputed.
- Attach external runner artifacts without invoking the runner automatically. Manual summaries remain declared evidence.
- Zero executed tests cannot satisfy all-tests-passing; represent discovered/executed/passed/failed/skipped totals and reject inconsistent counts.
- Make missing artifacts, explicit failures, disallowed skips, and changed source visible. Gate policy states when skips are allowed.
- Reuse Cargo/Julia and recur-git evidence where possible; use deterministic Visual Studio-like fixtures rather than inventing an external run.

## Implementation and verification scope

shared Warp evidence model; pure evidence readers; recur-git receipt integration; Rust/Julia fixtures.

Required evidence gates:

- `external-evidence-tests`
- `freshness-and-dirty-source-tests`
- `zero-tests-failure-skip-tests`

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
defines: recur.warp.evidence.integrity.slice.5 Structured external evidence and freshness
consumes: recur.warp.evidence.integrity.slice.4 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Versioned external manifests/results validate build/test/scan outcomes and exact scoped source/result byte fingerprints without invoking producers. Zero-test, failure, skips, missing results and source drift covered in 38 Julia integration assertions and Rust unit coverage. Dirty state is explicit; revision is provenance and the named file scope determines freshness.
