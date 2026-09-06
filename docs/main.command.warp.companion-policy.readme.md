# Warp: companion configuration policy foundation

Created 2026-09-06, following completed docs reconciliation at 17626c8.
State: implemented and verified; three slices complete. See
[verification](main.command.warp.companion-policy.verification.md).
Scope: make collapse use the existing validated nearest-config suffix policy,
including longest compound suffix matching. No new arbitrary executor, scoring
configuration, transition DSL, permission grant, or autonomous actor is introduced.
This is the first bounded foundation for opinionated Warp actions, not the entire
future orchestration policy engine.

Slice 0 captures red-first executable safety evidence. Slice 1 implements shared
policy. Final verifies regressions and documents compatibility. Existing defaults,
JSON shapes, confirmation gate, archival destination and conservative handling of
unknown files remain. Invalid legacy collapse configs now fail closed consistently
with queries; nearest config and compound suffix handling intentionally change.
Query/actor bucket parity applies to recognized Markdown eventness in the same
scope; companion may conservatively refuse unknown files rather than ignore them.

Tests must compare file bytes before/after dry runs and refusals; cover inherited
and nearer config, normalization, compound suffixes, malformed/conflicting config,
default behavior, policy changes between preview and confirm, blockers and scope.
Run existing completion/evolution/receipt and full regression tests unchanged.
Use only temporary test trees for archive execution. Do not install binaries or
archive live project work. Completion layers are declared evidence, not test runs.

Broader configurable transitions, evidence/execution policies and shared conventions
for other trait apps remain future work after this bounded contract is demonstrated.

## Configuration and compatibility

The same nearest ancestor `.recur/config.toml` now governs query and companion
suffix interpretation. A nearer config replaces the ancestor configuration;
missing fields use defaults, not an ancestor merge. Example:

```toml
[warp.suffixes]
active = ["work.open"]
complete = ["test.accepted"]
interesting = ["needs.review"]
blocked = ["approval.wait"]
```

Suffixes are trimmed/lowercased and validated; duplicates, overlapping groups,
non-string entries and invalid components fail before mutation. Longest matching
compound suffix wins. Default complete/current/strange/blocked behavior and JSON
shapes are preserved. Previously tolerated malformed actor configs now error;
an inherited policy now applies even when invoked from a nested directory.

```powershell
recur warp config -d <lane-root> --json
recur warp collapse-plan demo.lane -d <lane-root> --json
recur-warp collapse demo.lane -d <lane-root> --json
# Only after inspecting the current dry run and obtaining authority:
recur-warp collapse demo.lane -d <lane-root> --json --confirm
```

Confirmation recomputes policy and classification, not a saved plan. Changed
policy, blockers, ambiguous files, invalid/unreadable evidence or existing archive
destinations refuse before archive writes in the tested cases. This is not a
transactional guarantee against concurrent filesystem changes or an immutable
preview token. Unknown files remain conservative companion blockers even if core
queries omit them. Arbitrary actions and automatic permission are not introduced.

Run `julia julia-tests/main.command.warp.companion-policy.test.jl` against the
repository release-safe binaries; tests are also in the full Julia suite. No
tests archive user work; each writer fixture is a temporary directory.

defines: recur.warp.companion-policy.contract shared validated suffix policy without increased authority
