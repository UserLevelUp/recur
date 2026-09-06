# Slice 6: Whole-map contract and gate validation

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 5 (high)
Contract: contract:main.command.warp.evidence-integrity.slice-6:v1
Dependencies: slice-5

## Desired result and acceptance

- Extend/document existing map/merge validation for required receipts, dependency cycles/missing references, stale contracts, and contradictory results.
- A final receipt alone cannot cover missing prerequisite slices; report unresolved dependencies and evidence gates by slice.
- Bind mandatory gates to explicit outcomes from Slice 5, not merely nonempty reference strings.
- Document contract_hash semantics and versioning. Current v1 compares opaque strings; a sha256: prefix alone does not establish digest validation.
- If digest validation is introduced, define canonical bytes and algorithm with compatibility tests. This plan uses explicit opaque contract IDs.
- Historical Slice 0 evidence remains available after its baseline gate is accepted, without being counted as unfinished repair work.

## Implementation and verification scope

src/warp_bubble.rs; src/main_command_warp_impl.rs; src/recur_warp_main.rs; bubble/ring Julia suites.

Required evidence gates:

- `map-dependency-contract-tests`
- `map-evidence-outcome-tests`
- `baseline-preservation-tests`

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
defines: recur.warp.evidence.integrity.slice.6 Whole-map contract and gate validation
consumes: recur.warp.evidence.integrity.slice.5 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Map and companion share validation for dependencies/cycles/gate policies. Missing baseline blocks a final-only layer; changed contract explodes prior acceptance. Added checked evidence mode and per-slice gate outcomes. Whole-map regression 16/16; existing bubble 32/32 and companion 62/62 pass.
