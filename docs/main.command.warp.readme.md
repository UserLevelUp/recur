# recur warp

## Discover remaining bubbles

```powershell
recur warp -d docs
recur warp list -d docs
recur warp list --all -d docs --json
recur warp list --scan-all --all --json
```

Bare `recur warp` is equivalent to `recur warp list`. Both list remaining
declared bubbles and rings within project-aware scope beneath `-d` (the current
directory by default). `--all` includes completed projections. State and counts come
from the existing `merge` query, not filenames or authored readiness prose.
A ring and its coordinator bubble sharing an identity produce one ring row.

JSON uses `warp-list-v1`, with root, filter, discovered/listed/error counts,
and sorted entries containing identities, manifest paths, projected state,
counts, evidence/contract qualification where available, scope and diagnostic errors.
Each identity is qualified by its manifest directory. A ring and coordinator map
in the same directory form one entry; identical IDs in different directories are
independent entries, not duplicate errors. Invalid manifests stay visible as error rows; they are never
treated as completed. Per-entry errors do not fail the inventory command:
automation must inspect `errors` and each entry's state. Root/traversal failures
return a failing exit status. Ring evidence qualification is unassessed when
the existing ring projection does not expose it; use `merge` for details.

Discovery is read-only and includes `.recur`, including maps directly inside it.
It does not follow directory symlinks during traversal. No map means no bubble
entry: config, personas, watch status and stale `.current` notes are not bubbles.

The nearest `.recur/config.toml` can configure discovery; `recur init` writes:

```toml
[warp.discovery]
roots = ["."]
exclude_dirs = [".git", "target", "build", "dist", "node_modules", "fixtures"]
```

Existing configs without this section get these same defaults. Roots are existing
relative directories beneath the config's project root; exclusions are literal
directory names (case-insensitive), not globs. Configured arrays replace defaults.
`-d` narrows configured roots and never widens them. Overlapping roots are deduplicated.
The nearest config applies to the invocation; descendant configs are not merged
while walking. Invalid config/escaping roots fail clearly. An explicitly selected
scan root is entered even if its own name is excluded.

`--scan-all` bypasses discovery roots/exclusions under `-d` for diagnostics;
it does not imply `--all`, follow symlinks, or relax ring-domain containment.
JSON includes the effective `discovery_policy` and its source.

Inventory uses the manifest directory as its evidence root and reads that
directory's map/layers only. Ring children resolve their explicit `relative_root`
and use the same local rule. This prevents nested projects exchanging receipts.
Keep a bubble's map and layers together and evidence paths relative to that directory.
Existing explicit `map`, `merge` and lane queries retain their previous traversal
behavior; inventory scoping does not silently change those APIs. For legacy layouts
with layers spread across subdirectories, use the explicit merge at the intended
root; the inventory may conservatively show missing coverage.

## Lane queries

Status: `implemented` (status v1, compositional bubble v1, recursive ring v1,
and confirmed companion completion/evolution/collapse)
Date: 2026-09-04

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

## Recursive ring topology

For precise completion claims, policy-aware receipts, checked external evidence,
and reveal reconciliation, see [completion evidence](main.command.warp.evidence.readme.md).

When `<warp>.warp-ring.json` is present, the pure query surface projects a
coordinator and independently rooted child Warp domains:

```powershell
recur warp map coordinator.release -d planning/ --json
recur warp merge coordinator.release -d planning/ --json
recur warp status coordinator.release -d planning/ --json
```

The `warp-ring-map-v1` contract declares the coordinator, bounded projection
depth, domain-relative roots, required child states, distinct parent acceptance
Slices, public contract hashes, and directional watch subscriptions. Projection
rejects workspace escapes, cycles, exhausted depth, stale accepted public
contracts, and stale or rejected watcher receipts. A missing watcher receipt is
reported as `declared`; an observed stale/rejected receipt blocks convergence.

Parent acceptance is intentionally separate from child completion. A child can
be locally complete without being accepted into the coordinator's result.

## Confirmed evolution

`recur-warp evolve` only supersedes a bubble whose accepted layers prove it is
exploded by conflict or stale contract:

```powershell
recur-warp evolve demo.source candidate.target.json -d planning/ --json
recur-warp evolve demo.source candidate.target.json -d planning/ --json --confirm
```

The dry run identifies carried and invalidated Slices without writing. Confirmed
execution publishes the successor map, carries forward only single-result
accepted layers whose Slice and contract identities are unchanged, and writes a
`recur-warp-supersession-v1` ACK under `.recur/warp/`.

## Confirmed collapse

`recur warp collapse-plan` remains the read-only classification surface.
`recur-warp collapse` mirrors that plan and performs recoverable archival only
after confirmation:

```powershell
recur warp collapse-plan demo.lane -d planning/ --json
recur-warp collapse demo.lane -d planning/ --json
recur-warp collapse demo.lane -d planning/ --json --confirm
```

Known-complete evidence moves under `.recur/warp/archive/<lane>/`; interesting
evidence is preserved. Blocked or ambiguous/current evidence prevents mutation
until an operator resolves it. A successful collapse writes a
`recur-warp-collapse-receipt-v1` ACK.
