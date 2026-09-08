# Baseline and frozen contracts

Status: complete, 2026-09-07. Accepted baseline evidence is recorded in
[baseline](main.command.reveal.artifact-types.baseline.md) and the slice-0 layer.

Contract: `contract:main.command.reveal.artifact-types.slice-0:v1`

Inspect reveal/config and legacy tests; freeze type metadata, prefix policy, filter syntax, JSON schema and explicit-root behavior. Create the standalone red suite from the acceptance plan and record actual legacy baseline results.

See [scope](main.command.reveal.artifact-types.readme.md) and [acceptance plan](main.command.reveal.artifact-types.tests.md).

Observed: 31 legacy reveal checks passed, 33 legacy init checks passed, and the
initial standalone contract suite reported 37 expected failures with no errors.
The exact type, selection, output and scope contract is now frozen.
