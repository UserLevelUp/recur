# Focus: implement recur-lang init

Status: open. Parent: [remaining Lang work](README.CORE.IMPROVEMENT30.recur-lang.todo.md).
Warp: [main.lang.init](warps/main.lang.init.readme.md).
Exact acceptance: [v1 contract](warps/main.lang.init.contract.md).

Deliver `recur-lang init [-d ROOT] [--dry-run] [--json]`, adding editable
`[recur-lang]` preferences to the nearest project `.recur/config.toml`.
Reuse the current companion patterns rather than adding another config loader
or precedence scheme. Default target is `unspecified`; preferences are inert
until a subsequent planner consumes them.

- [ ] Review the frozen contract and [expected-red baseline](warps/main.lang.init.baseline.md).
  The observed 57 pass / 44 fail result establishes the missing command, not
  implemented initialization. No slice is currently accepted.
- [ ] Implement preview and additive installation, including nearest-root
  discovery, exact defaults, empty/inline tables and explicit false settings.
- [ ] Preserve unrelated configuration, user comments, unknown keys and repeat
  byte identity. Reserve the policy table so it cannot become a file lane.
- [ ] Reject malformed types/versions, invalid roots and escaping paths without
  mutation. Add deterministic Cargo tests for concurrent edits and publication
  failures, including Windows sharing-lock rollback.
- [ ] Make `julia-tests/main.command.recur-lang.init.test.jl` pass, then add it to
  the normal runner. Run relevant legacy tests and record process faults as
  failed validation attempts rather than acceptance.
- [ ] Update CLI/discovery documentation and applicable expert guidance; verify
  the intended installed candidate and satisfy the Warp's evidence gates.

After these criteria are verified, record the source revision and evidence here,
rename this file to `README.CORE.IMPROVEMENT30.recur-lang.init.todo.checked.md`,
and update the parent link. Leave the broader Lang todo open for subsequent work.

consumes: recur.lang.init.v1 exact initialization acceptance boundary
triggers: main.lang.init.plan implement the selected bounded companion increment
