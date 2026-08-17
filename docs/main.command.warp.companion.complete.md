# `recur-warp` companion boundary

Status: `complete` for Slice completion in Recur v0.2.8
Date: 2026-08-15

Core `recur warp` remains a pure query and projection surface. The separate
`recur-warp` executable implements one finite write action:

```text
recur-warp complete <warp> <slice>
```

The command is a dry run unless the operator supplies `--confirm`. It accepts
no arbitrary shell command, broad mutation glob, implicit approval from
`warp next`, or background execution.

## Completion contract

The writer requires:

- one Warp and Slice declared by a unique `warp-bubble-map-v1` file;
- a stable attempt identity and nonblank result hash;
- exactly the evidence gates declared by that Slice contract; and
- explicit `--confirm` before persistence.

The contract hash is derived from the map rather than supplied by the caller.
The writer rejects an accepted result that conflicts with an existing result
for the same qualified Warp, Slice, and contract.

## Receipt behavior

An ACK is the atomically persisted `warp-slice-layer-v1` completion layer.
Replaying identical attempt content is idempotent. A confirmed attempt refused
after validation leaves a JSON NAK receipt under:

```text
.recur/warp/recur-warp.<warp>.<slice>.<attempt>.status.nak.json
```

The pure merge excludes `.recur`, so a NAK never contributes accepted
coverage. It remains private audit evidence for diagnosis and retry.

Warp evolution is not implemented by this slice. A future `recur-warp evolve`
must preserve the same confirmation, exact-identity, evidence, and receipt
boundaries.

Trace-id lines:

```text
defines: recur.warp.companion.boundary confirmation-gated finite Slice completion writer
defines: recur.warp.companion.dry-run default non-mutating completion proposal
consumes: recur.warp.bubble.map exact Warp Slice contract and evidence gates
produces: recur.warp.slice.completion.layer atomic accepted evidence layer
produces: recur.warp.companion.nak durable private refusal receipt for a confirmed invalid attempt
triggers: recur.warp.merge automatic pure recomposition after a valid accepted layer appears
```
