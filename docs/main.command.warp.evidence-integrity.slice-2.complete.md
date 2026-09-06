# Slice 2: Reveal exposes effective state policy

Status: complete
Warp: main.command.warp.evidence-integrity
Reported scope: Issue 4 (medium)
Contract: contract:main.command.warp.evidence-integrity.slice-2:v1
Dependencies: slice-1

## Desired result and acceptance

- Warp-oriented reveal entries expose effective state suffixes and provenance: defaults versus the actual configuration path.
- Text and JSON provide equivalent policy information before a caller creates a receipt.
- Reuse policy resolution shared with warp config; test defaults, nearest project configuration, and custom suffixes.
- Keep reveal read-only and preserve authored persona fields. Freeze a bounded explicit convention for identifying Warp-oriented entries.

## Implementation and verification scope

src/main_command_reveal_impl.rs; project configuration helpers; Reveal Julia tests.

Required evidence gates:

- `reveal-policy-tests`
- `policy-source-parity-tests`

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
defines: recur.warp.evidence.integrity.slice.2 Reveal exposes effective state policy
consumes: recur.warp.evidence.integrity.slice.1 accepted prerequisite evidence
consumes: recur.warp.evidence.integrity.contract declared acceptance and evidence policy
```


## Verified slice outcome

Shared nearest-policy resolver and longest suffix matching implemented. Reveal includes effective values and per-field provenance. Focused policy 9/9, Reveal regressions 25/25, Warp structure 28/28; shared Rust policy test passes.
