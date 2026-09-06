# Slice 4: Recorded completion and evidence semantics

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 1 (high)
Contract: contract:main.command.warp.evidence-integrity.slice-4:v1
Dependencies: slice-3

## Desired result and acceptance

- Expose recorded state, evidence status (absent/declared/checked/stale/failed), and contract satisfaction as distinct facts.
- A bare completion file is recorded completion, never checked success; zero trace-role sites neither certify nor automatically disprove external tests.
- Failed or unresolved mandatory gates remain visible despite a completion suffix.
- Define checked as validated evidence under an explicit validation method; distinguish artifact validation from independently rerunning the producer.
- Specify additive compatibility or a versioned output migration for existing verdict consumers, with text/JSON parity tests.

## Implementation and verification scope

src/main_command_warp_impl.rs; shared Warp contracts; status fixtures and tests.

Required evidence gates:

- `status-evidence-state-tests`
- `mandatory-gate-visibility-tests`

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
defines: recur.warp.evidence.integrity.slice.4 Recorded completion and evidence semantics
consumes: recur.warp.evidence.integrity.slice.3 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Status now qualifies recorded completion, evidence and contract states without breaking legacy verdict consumers. Integrated tests prove failed mandatory gates stay blocked even beside a completion file. Bare completion remains evidence absent. Focused integrity suite 74 assertions passes.
