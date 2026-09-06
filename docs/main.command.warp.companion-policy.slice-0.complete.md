# Slice 0: baseline-and-red-tests

Status: complete
Contract: contract:main.command.warp.companion-policy.slice-0:v1

Freeze current behavior and reproduce policy gaps with no-mutation tests.
Acceptance: baseline-and-red-tests. Follow the readme's test matrix, compatibility and safety boundaries.
Complete; actual evidence is recorded in the verified outcome below.


## Verified slice outcome

Baseline 17626c8. Actual old release-safe binary produced 44 passes/19 failures with no errors/broken cases. Test fixtures expose inherited/compound/normalized policy mismatch and invalid policy acceptance; only temporary trees used. See companion-policy.verification.md for red/green trajectory.
