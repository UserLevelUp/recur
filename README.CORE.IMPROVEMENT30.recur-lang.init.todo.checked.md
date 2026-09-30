# Completed focus: implement recur-lang init

```text
defines: recur.lang.work.remaining.init focused companion-policy initialization work
consumes: recur.lang Lang design context
consumes: recur.lang.companion opinionated companion implementation context
consumes: recur.lang.work.remaining parent remaining-work index
consumes: recur.lang.init.v1 exact initialization acceptance boundary
consumes: recur.eventness.todo stable todo identity and attention convention
triggers: main.lang.init.plan implement the selected bounded companion increment
```

Status: checked, 2026-09-30. Parent: [remaining Lang work](README.CORE.IMPROVEMENT30.recur-lang.todo.md).
Warp: [main.lang.init](warps/main.lang.init.readme.md).
Exact acceptance: [v1 contract](warps/main.lang.init.contract.md).

Deliver `recur-lang init [-d ROOT] [--dry-run] [--json]`, adding editable
`[recur-lang]` preferences to the nearest project `.recur/config.toml`.
Reuse the current companion patterns rather than adding another config loader
or precedence scheme. Default target is `unspecified`; preferences are inert
until a subsequent planner consumes them.

- [x] Review the frozen contract and [expected-red baseline](warps/main.lang.init.baseline.md).
  The observed 57 pass / 44 fail result establishes the missing command, not
  implemented initialization. All four init slices now have accepted layers;
  the map retains its declared-evidence policy.

  defines: recur.lang.work.remaining.init.baseline contract review and intentionally failing tests

- [x] Implement preview and additive installation, including nearest-root
  discovery, exact defaults, empty/inline tables and explicit false settings.

  defines: recur.lang.work.remaining.init.configuration nearest-project defaults and preview

- [x] Preserve unrelated configuration, user comments, unknown keys and repeat
  byte identity. Reserve the policy table so it cannot become a file lane.

  defines: recur.lang.work.remaining.init.configuration.preservation explicit choices and byte identity

- [x] Reject malformed types/versions, invalid roots and escaping paths without
  mutation. Add deterministic Cargo tests for concurrent edits and publication
  failures, including Windows sharing-lock rollback.

  defines: recur.lang.work.remaining.init.validation malformed inputs and path boundaries
  defines: recur.lang.work.remaining.init.publication concurrent changes and atomic-write failure

- [x] Make `julia-tests/main.command.recur-lang.init.test.jl` pass, then add it to
  the normal runner. Run relevant legacy tests and record process faults as
  failed validation attempts rather than acceptance.

  defines: recur.lang.work.remaining.init.tests green focused and legacy acceptance checks

- [x] Update CLI/discovery documentation and applicable expert guidance; verify
  the intended installed candidate and satisfy the Warp's evidence gates.

  defines: recur.lang.work.remaining.init.integration docs, discovery, candidate and Warp evidence

Implementation was verified on top of `b4ade84`; the source revision is the
commit containing this completed todo. [Evidence and gate mapping](warps/main.lang.init.verification.md)
record 257 Cargo passes (7 ignored doctests), 5,458 Julia passes (73 existing
expected-broken), all 58 verification cases and 211 installed init assertions.
All seven installed binaries match the candidate hashes. Immutable structured
receipts bind 104 source/artifact inputs; legacy full-suite results remain
explicit observations with their ignored/broken counts.

All four `main.lang.init` slices have `implemented-20260930` acceptance layers.
Full-root projection reports `declared-gates-satisfied`; configuration gates also
have checked structured evidence. The broader Lang todo and verification Warp
remain open. This file was renamed after the actual tests and gate publication.

Keep these trace IDs unchanged when the filename becomes `todo.checked.md`.
For this focus and its nested concerns:

```powershell
recur trace-id 'recur.lang.work.remaining.init.**' --scope 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --format full
```
