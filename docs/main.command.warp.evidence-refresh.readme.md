# Same-contract evidence refresh

`recur-warp refresh` renews one source-stale external gate reference while
preserving its accepted layer and historical manifests/results. It does not run
tests, change contracts, choose work, or perform an Eventness transition.
Use a local binary whose help includes refresh; installed binaries may lag.

```powershell
.\target\debug\recur-warp.exe refresh demo.warp-map.json demo.s0.first.warp-layer.json `
  --gate tests --from evidence:old.json --to evidence:new.json `
  --refresh-id rerun-1 --reason "Observed rerun after implementation change" -d PROJECT --json
```

This is a preview. After the external runner has produced truthful current
results, inspect the preview and add `--confirm` to publish the refresh record.
Core `recur warp show/merge/list` and ring projections resolve its current leaf
and report original reference plus refresh IDs. The old standalone
`recur warp evidence old.json` still reports the old evidence's own freshness.

The replacement must pass the same gate rule, retain every predecessor input
path, and have the same kind/project. The old result must remain unchanged and
passing; only source drift is refreshable. Missing/failed/changed results,
weakened scope, changed contracts, forks, cycles and orphan refreshes fail closed.
Fresh evidence for one reference never hides a different failing reference,
conflicting layer or missing dependency. This does not authenticate a producer
or prove that its declared test scope is sufficient.

Each immutable `warp-evidence-refresh-v1` record lives beside the map under
`WARP.evidence-refresh/ID.json`, binding the map identity, full slice policy,
accepted layer bytes, gate and predecessor/replacement manifest bytes. Further
drift requires a new ID and `--from` equal to the current leaf. Preserve all old
records. Changing the contract requires a separately supported evolution path;
refresh does not bypass evolve's explosion condition.

Reads are bounded: 2 MiB JSON, 32 MiB other inputs, 128 canonical files and
64 MiB total per assessment; the local receipt directory has at most 64 entries.
Publication reserves room for staging. Root-relative forward-slash paths and
contained files are required; symlink/reparse ancestors are refused. The writer
revalidates bytes and directory inventory, stages with create-new/sync and
publishes without overwriting. An identical confirmed retry can recover matching
staging; torn/conflicting staging names a bounded blocker. Exclusive operator
ownership during the short write is assumed; no universal power-loss guarantee.

This is generic Warp evidence maintenance owned by recur-warp. Lang's exact
contracts, body aliases and dependency edges, intended Eventness, recorded state,
static findings and external observations keep their existing meanings. Neither
Lang status files nor lifecycle Markdown receipts gain refresh semantics.
Snapshot prompt packets still refuse transitive checked evidence; use live
full-root Warp queries. No new grammar, runtime or live coordinator is added.

See [the acceptance contract](../warps/main.command.warp.evidence-refresh.contract.md)
and `tests/warp_refresh.rs` / `tests/warp_refresh_bounds.rs` for executable cases.

defines: recur.warp.evidence.refresh.usage immutable same-contract renewal
