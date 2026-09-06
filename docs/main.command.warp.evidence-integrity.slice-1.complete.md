# Slice 1: Unsupported suffix diagnostics

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 3 (high)
Contract: contract:main.command.warp.evidence-integrity.slice-1:v1
Dependencies: slice-0

## Desired result and acceptance

- An in-scope .verified.md artifact with no recognized state produces a diagnostic naming the file, effective recognized suffixes, and searched root.
- Distinguish an absent lane, invalid/wrong search directory, and unsupported suffix when evidence permits; do not guess that an absent lane exists elsewhere.
- Unknown suffixes are never silently promoted to completion. Proposed renames are non-mutating and flag destination conflicts.
- Cover default/custom suffixes, lexical lookalike lanes, and text/JSON diagnostics.

## Implementation and verification scope

src/main_command_warp_impl.rs; Warp status Julia tests and fixtures.

Required evidence gates:

- `suffix-diagnostics-tests`
- `nonmutating-diagnostics-tests`

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
defines: recur.warp.evidence.integrity.slice.1 Unsupported suffix diagnostics
consumes: recur.warp.evidence.integrity.slice.0 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Suffix regression moved from 4 failing assertions to 12/12 passing. Unknown suffix reports nearby file, root, policy and non-mutating conflict-aware guidance. Missing lane is distinguished without searching unrelated roots.
