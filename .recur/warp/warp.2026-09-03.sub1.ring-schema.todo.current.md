# Warp 2026-09-03 Sub-Slice 1: Ring Schema & Types

Status: `todo.current`
Date: 2026-09-03
Parent: `.recur/warp/warp.2026-09-03.completion-suite.warp-map.json`
Slice: `sub1-ring-schema`
Contract Hash: `sha256:01-ring-schema-v1-types-and-contracts`

## Intent

Define and freeze the Rust types and schema for `warp-ring-map-v1` in `src/warp_bubble.rs`:
- `WARP_RING_MAP_SCHEMA = "warp-ring-map-v1"`
- `WarpRingMap`: coordinator domain, domains list, subscriptions, projection depth.
- `WarpRingDomain`: domain identity, relative root, role, public contract hash, required state, parent acceptance.
- `WarpRingParentAcceptance`: slice identity, contract hash.
- `WarpRingSubscription`: subscription id, direction, source/target domains, filter, event contract, freshness.

## Evidence Gates

- [ ] `rust-lib-tests`: Unit tests verifying deserialization and validation of ring fixtures.
- [ ] `warp-ring-map-v1-structs`: Exported from `warp_bubble.rs` for use in `recur` and `recur-warp`.

## Trace-id Lines

```text
defines: warp.2026-09-03.sub1.ring-schema.todo.current first implementation slice of 2026-09-03 warp bubble
consumes: recur.warp.ring.schema warp-ring-map-v1 contract
produces: recur.warp.ring.types rust data structures in src/warp_bubble.rs
triggers: warp.2026-09-03.sub2.ring-query-merge recursive composition engine
```
