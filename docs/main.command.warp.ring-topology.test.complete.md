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

Result after implementation: 44 passed, 0 broken, 44 total.

The former broken assertion is now green: `recur warp map`, `merge`, and
`status` consume `warp-ring-map-v1`, compose complete child bubbles, and keep
parent acceptance distinct from child completion.

## Trace-id lines

```text
defines: main.command.warp.ring-topology.test.complete verified recursive-domain schema and asynchronous subscription harness
consumes: recur.warp.ring.schema warp-ring-map-v1 companion contract
consumes: main.command.watch.impl coordinator-to-worker and worker-to-coordinator active event delivery
consumes: main.command.watch.query.impl pure subscription ACK inspection
produces: recur.warp.ring.projection.green verified recursive recur warp merge composition
```
