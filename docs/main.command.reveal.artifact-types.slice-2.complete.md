# Query integration

Status: complete, 2026-09-07. Query integration checks passed; see
[verification](main.command.reveal.artifact-types.verification.md).

Contract: `contract:main.command.reveal.artifact-types.slice-2:v1`

Implement list/show/type filtering, stable identity and ambiguity handling. Pass configured hierarchy and explicit-root boundary acceptance cases.

See [scope](main.command.reveal.artifact-types.readme.md) and [acceptance plan](main.command.reveal.artifact-types.tests.md).

Observed list/show type provenance, exact-path identity, filtered ambiguity and
type mismatch, custom suffixes and separators, nested config, explicit hidden
roots and sibling exclusion in the 123-check CLI suite. Rust tests exercise
Windows junction escape/cycle exclusion. Explicit -d behavior is documented as
an intentional change from config-root broadening.
