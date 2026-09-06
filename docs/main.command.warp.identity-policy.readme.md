# Warp: initialization, identities and optional preservation policy

Status: planned; Slice 0 baseline recorded, implementation not started.

A bubble has a permanent identity and an editable name. Its slices identify work
independently of its container. A successor gets a new bubble identity and explicit
lineage; moving or renaming an existing bubble does not create a new identity.
This Warp prepares that model; it does not implement semantic merge/split.

## Scope and defaults

- Add recur-warp init: explicit setup write, optional --dry-run, no extra confirmation.
- Install editable nearest-project configuration and a starter JSON template.
  Default output is warps/; comments illustrate docs/warps/ and .recur/warps/.
  Preserve existing fields, comments and user templates; repeated init is harmless.
- Creation assigns bubble_uuid and slice_uuid as UUIDv7, separately from readable
  warp_id and slice_id. Templates must not clone UUID identities. Validate identity
  fields, expose them to JSON consumers, and preserve them through existing writers.
- Legacy maps remain readable without invented identities or automatic rewrites.
  An explicit identity migration and full merge/split lineage are deferred.
- Add typed [warp.removal] configuration: require_confirmation=true,
  require_committed_snapshot=true, require_preservation_ref=true,
  require_pushed_ref=false. Explicit false values survive initialization.
  Configuration alone never authorizes or performs removal.

No removal command ships in this Warp. Git snapshot/ref verification is NOT
enforced merely because its configuration fields exist. Document that distinction
in help and examples. Closing remains different from removing. Combining/splitting
must never automatically delete source bubbles.

## Acceptance and follow-on tests

Slice 4 adds trait-style human list presentation: [warps."<readable-id>"]
sections with separate state, completed, required, pending, blocked, evidence and
scope fields. Retain manifest paths, conflict/stale-contract information, clear
per-entry errors and the inventory summary. Unknown counts remain unknown, not
zero; ring domain counts must not be mislabeled as completed slices. JSON-escape
names/values to prevent control characters corrupting output. Scope may use a
readable relative path when unambiguous; do not change actual scope resolution.
Bare warp and explicit list must match. --json stays warp-list-v1 unchanged.
The standalone main.command.warp.list-format.test.jl contract is intentionally red
until implemented; extend with ring/escaping/duplicate-scope fixtures at that slice.
The unaccepted final Slice contract advances to v2 because it now depends on Slice 4.
Previously accepted baseline receipts remain unchanged.

Compatibility is mandatory: readable filenames remain discoverable through recur
tree and recur files; explicit defines/consumes/produces/triggers markers remain
queryable through recur trace-id. UUID metadata supplements hierarchy, not replaces
it. A UUID field alone is not automatically a trace-id role declaration.
For hidden storage, generic queries may explicitly select -d .recur/warps;
do not silently change their existing traversal rules.

recur warp is the read-only query surface: list/find bubbles through inventory,
show progress/counts, and inspect slices/dependencies/readiness. recur-warp is
the configured write-side companion. Slice 10 is a slice identity, not a speed
measurement or proof that slices 0–9 passed. Counts remain evidence-derived.
The passing main.command.warp.query-compatibility.test.jl suite checks visible
and hidden placement, UUID-bearing maps, explicit trace markers, eleven slices,
inventory/progress and no query writes. Keep it in the normal regression suite.

Standalone executable contract: julia-tests/main.command.warp.identity-policy.test.jl.
Keep it deliberately red outside runtests.jl until implementation; then integrate
it as ordinary passing tests, without test_broken or weakened legacy assertions.
Extend it during implementation for partial configuration, nearest-project scope,
template identity injection, UUID preservation through completion and copy/rename,
malformed/duplicate identities, legacy maps, and failure-safe init publication.
Dry-run UUIDs are prospective, not persisted identity reservations.

Before a future removal API ships, executable temporary-Git tests must cover:
dirty/untracked bubble artifacts; exact snapshot mismatch; missing durable refs;
required push missing; all optional Git guards disabled in a non-Git project;
separate confirmation; a recovery record outside the removal target; bounded
artifact ownership; and refusal without partial deletion. Include receipts and
lineage, not only the map. Never use the live repository for deletion fixtures.

Final gate: Cargo and full Julia regressions plus new tests pass with existing
known-broken cases reported honestly. No automatic commit, push, migration or cleanup.
