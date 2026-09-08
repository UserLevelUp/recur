# Regression and acceptance

Status: complete, 2026-09-07. Full regressions passed; see
[verification](main.command.reveal.artifact-types.verification.md).

Contract: `contract:main.command.reveal.artifact-types.slice-final:v1`

Integrate green acceptance tests; run cargo test --locked and Julia full regressions using the documented working flags. Record actual results and source-bound evidence before completion.

See [scope](main.command.reveal.artifact-types.readme.md) and [acceptance plan](main.command.reveal.artifact-types.tests.md).

The new suite is integrated into the normal Julia runner. Cargo passed 188 tests
with 7 existing ignored doc tests. Full Julia passed 3707 with 73 existing
known-broken, 0 failures/errors on the installed Julia 1.12.4 using
`--startup-file=no -C generic` and normal optimization. Earlier internal Julia
failures and exact working-file fingerprints are retained in the verification
record. Local release-safe binaries were rebuilt; no global installation occurred.
