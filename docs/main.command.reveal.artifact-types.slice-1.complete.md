# Classification

Status: complete, 2026-09-07. Classification checks passed; see
[verification](main.command.reveal.artifact-types.verification.md).

Contract: `contract:main.command.reveal.artifact-types.slice-1:v1`

Implement validated metadata and configured prefix resolution with custom types, untyped legacy behavior and conflict diagnostics. Pass classification acceptance cases.

See [scope](main.command.reveal.artifact-types.readme.md) and [acceptance plan](main.command.reveal.artifact-types.tests.md).

Implemented shared `recur::reveal_artifact` classification and validated
`reveal.types` configuration. Observed metadata/custom/untyped/conflict, longest
prefix, opt-out and invalid-policy cases in the 123-check CLI suite. Rust unit
tests additionally prove invalid/conflicting metadata cannot fall back to hints.
