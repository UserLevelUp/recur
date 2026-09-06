# Slice 1: shared-policy-parity

Status: complete
Contract: contract:main.command.warp.companion-policy.slice-1:v1

Use shared policy for companion collapse; preserve explicit confirmation and conservative refusal.
Acceptance: shared-policy-parity. Follow the readme's test matrix, compatibility and safety boundaries.
Complete; actual evidence is recorded in the verified outcome below.


## Verified slice outcome

Collapse delegates to shared WarpPolicy loader and state/group matching. Duplicate parser removed; invalid/unreadable evidence errors before archive mutation. Existing confirmation, schemas, scope and defaults preserved. All 81 expanded subprocess assertions pass, including no-mutation byte snapshots and successful bounded archival.
