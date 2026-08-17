# recur warp

Status: `implemented` (read-only status v1 and compositional bubble v1)
Date: 2026-08-15

`recur warp status <lane>` reads a bounded lane membrane and returns an
evidence-backed `optimum`, `sub_optimum`, or `blocked` verdict. It does not
rename files, collapse eventness, write receipts, start watchers, or approve
anything.

```powershell
recur warp status demo.project.good -d julia-tests/fixtures/warp-status-v1/optimum
recur warp status demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum --json
recur warp explain demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum
recur warp next demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum --json
recur warp collapse-plan demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum --json
recur warp config -d julia-tests/fixtures/warp-status-v1/config-override --json
```

The `warp-status-v1` response contains concrete files, suffix and state-group
counts, trace-id role counts, signals, residual pressures, and suggested next
actions. `objective` is the sum of residual weights; signals explain evidence
but do not make residual pressure negative.

Default suffix groups are `current` → active, `complete` → complete, `strange`
→ interesting, and `blocked` → blocked. A project may override the non-active
groups with `[warp.suffixes]` in `.recur/config.toml`.

The command treats a `blocker` or `operator approval` marker as an external
blocker and reports it rather than attempting to resolve it.

`recur warp explain` renders the same status evidence with signals and residual
paths. `recur warp next` emits only the suggested actions. Both are read-only;
their suggestions are not commands to execute and do not start `recur-warp`.

`recur warp collapse-plan` is a read-only partition of the same lane evidence:
complete/verified files are `collapse_known`, interesting files remain
`preserve_interesting`, blocked files or blocker-marked files are `blockers`,
and active current files remain `ambiguous`. It does not rename or collapse any
file. `recur warp config` reports the active suffix mapping used by every Warp
projection, including defaults when no project policy exists.

The dot-separated fixture strategy and full read-only command matrix are in
`main.command.warp.test-structure`.

## Compositional Warp bubbles

Recur v0.2.8 adds a declared final bubble map and accepted Slice completion
layers. The canonical files are:

```text
<warp>.warp-map.json
<warp>.<slice>.<attempt>.warp-layer.json
```

The map uses schema `warp-bubble-map-v1` and declares `warp_id` plus
`required_slices`. Each required Slice declares `slice_id`, `contract_hash`,
optional `depends_on`, and optional `evidence_gates`.

An accepted layer uses schema `warp-slice-layer-v1` and records the exact Warp,
Slice, contract hash, attempt identity, result hash, and evidence references.

```powershell
recur warp map demo.release -d planning/ --json
recur warp merge demo.release -d planning/ --json
recur warp status demo.release -d planning/ --json
```

`map` emits `warp-bubble-map-view-v1`. `merge` emits the deterministic
`warp-bubble-projection-v1` with covered, pending, blocked, stale-contract,
conflicting, and total counts. The projection state is `complete`,
`incomplete`, `blocked`, or `exploded`.

Independent accepted layers compose without completion-order significance.
Identical attempt replay is idempotent. Different accepted result hashes for
one qualified Slice conflict. Accepted layers with an obsolete contract hash
are stale. Both conflict and stale-contract evidence visibly explode the
current bubble instead of selecting an implicit winner.

When a matching map exists, `warp status` includes the bubble projection and
turns incomplete coverage into a residual, blocked coverage into a blocker,
and exploded composition into an evolution pressure. The map, layers, and
receipts are evidence; the merged bubble is a rebuildable projection.

## Confirmed completion

The write-side companion derives contract and evidence requirements from the
map:

```powershell
recur-warp complete demo.release alpha `
  --attempt-id attempt-alpha-1 `
  --result-hash sha256-result-alpha `
  --evidence tests=receipts/alpha-tests.json `
  -d planning/

# Repeat with explicit authorization to persist the layer
recur-warp complete demo.release alpha `
  --attempt-id attempt-alpha-1 `
  --result-hash sha256-result-alpha `
  --evidence tests=receipts/alpha-tests.json `
  -d planning/ --confirm
```

Without `--confirm`, it prints a plan and writes nothing. With confirmation it
atomically persists one accepted layer. Identical retries report `idempotent`;
conflicting results are refused and leave a NAK receipt under `.recur/warp/`.
Core `recur warp` never performs these writes.
