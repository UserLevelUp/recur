# Slice 0: Current state and reproducible baseline

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: All seven report items
Contract: contract:main.command.warp.evidence-integrity.slice-0:v1
Dependencies: none

## Desired result and acceptance

- Preserve the supplied report and distinguish its external observations from locally reproduced behavior.
- Record executable versions, Git revision and dirty state, effective suffix policy, and exact reproduction commands/output.
- Audit existing status/config/map/merge, recur-warp complete, and recur-git test-receipt before labeling a capability missing.
- Build bounded fixtures for completion-only, unknown suffix, missing prerequisite, stale contract, and declared external test results. Record which expectations already pass and which require repair.
- Do not claim the external Visual Studio 50-test result was independently rerun here. Freeze the local behavior baseline and select the first failing acceptance case.

## Implementation and verification scope

src/main_command_warp_impl.rs; src/main_command_reveal_impl.rs; src/recur_warp_main.rs; src/recur_git_main.rs; existing Warp/Reveal Julia suites.

Required evidence gates:

- `baseline-capability-audit`
- `reported-cases-reproduced`

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
defines: recur.warp.evidence.integrity.slice.0 Current state and reproducible baseline

consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Baseline captured in main.command.warp.evidence-integrity.baseline.reference.md. Existing suites pass; focused baseline probe passes 7/7. Report and observed/source-audited limitations are preserved. Baseline capture is complete; repairs remain pending.
