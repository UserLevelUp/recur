# Slice 3: Supported policy-aware receipt workflow

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 2 (high)
Contract: contract:main.command.warp.evidence-integrity.slice-3:v1
Dependencies: slice-2

## Desired result and acceptance

- Audit and extend/document the existing companion workflow, with templates naming slice, contract reference, dependencies, gates, and evidence.
- Use the effective completion suffix for lifecycle artifacts and explicitly distinguish those artifacts from warp-slice-layer-v1 JSON acceptance layers.
- New confirmed receipts are discoverable by warp status at the documented root under default and custom policy.
- Dry runs do not write; confirmed writes avoid overwriting existing evidence and diagnose duplicates/conflicting state artifacts.
- Do not mark a template or manual evidence declaration as checked verification.

## Implementation and verification scope

src/recur_warp_main.rs; shared policy helpers; companion Julia tests; Warp readme.

Required evidence gates:

- `receipt-discovery-tests`
- `receipt-conflict-tests`
- `receipt-workflow-docs`

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
defines: recur.warp.evidence.integrity.slice.3 Supported policy-aware receipt workflow
consumes: recur.warp.evidence.integrity.slice.2 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

recur-warp receipt previews policy-aware templates and confirms declarations without overwriting attempts. New custom-suffix receipt is found by status. Focused receipt workflow 12/12; existing policy/diagnostics remain green. Required gate and contract fields are included, with evidence explicitly declared.
