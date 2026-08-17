# Warp Ring Topology Test Receipt

Status: `test.complete`
Date: 2026-08-15

## Verified

- `warp-ring-map-v1` freezes coordinator identity, nested domain roots, child
  Warp identity, public contract hashes, required child state, distinct parent
  acceptance Slices, bounded projection depth, and directional subscriptions.
- Coordinator, docs-monkey, and test-bird each complete an independent
  `recur init` and own a nearest `.recur/config.toml`.
- A coordinator-to-worker `recur-watch` delivers a task event asynchronously.
- A worker-to-coordinator `recur-watch` delivers a completion receipt
  asynchronously.
- `recur watch status` queries both accepted endpoint receipts without running
  the subscription loop itself.
- Missing parent acceptance remains distinguishable from required child state.

## Command

```powershell
julia julia-tests/main.command.warp.ring-topology.test.jl
```

Result: 29 passed, 1 intentionally broken, 30 total.

The broken assertion is the implementation handoff: `recur warp merge` does
not yet consume `warp-ring-map-v1`. It must become an ordinary passing
assertion when the recursive projection is implemented.

## Trace-id lines

```text
defines: main.command.warp.ring-topology.test.complete verified recursive-domain schema and asynchronous subscription harness
consumes: recur.warp.ring.schema warp-ring-map-v1 companion contract
consumes: main.command.watch.impl coordinator-to-worker and worker-to-coordinator active event delivery
consumes: main.command.watch.query.impl pure subscription ACK inspection
produces: recur.warp.ring.projection.red executable handoff to recursive recur warp merge composition
```
