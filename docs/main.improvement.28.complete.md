# main.improvement.28.complete

Status: `complete`
Date: 2026-05-23

## Purpose

Record the seed implementation of Improvement 28 in the docs-side improvement
tree.

## Canonical Proposal

- `README.CORE.IMPROVEMENT28.md`

## What Shipped

Improvement 28 introduces root-level `.recur-*` capability cards plus a
read-only query surface:

```text
recur capability list
recur capability explain <name>
recur capability doctor
```

Seed cards now exist for:

- `.recur-warp`
- `.recur-watch`
- `.recur-git`
- `.recur-trace-id`
- `.recur-reveal`

The important split is that `.recur-warp` can describe the warp capability
before any top-level `recur warp` command ships.

## Verification

```powershell
recur capability list -d .
recur capability doctor -d .
recur capability explain warp -d .
julia julia-tests/main.command.capability.test.jl
```

Observed verification on 2026-05-23:

- `recur capability doctor -d .` reports all seed cards present
- `julia julia-tests/main.command.capability.test.jl` passes 25/25

## Remaining Follow-Up

Future work can add card authoring or repair commands, but the seed query
surface is complete.

## Trace-Id Lines

```text
defines: main.improvement.28 complete seed capability-card query surface for root .recur-* files
defines: recur.capability.cards root-level self-describing .recur-* files for shared human agent and local tool understanding
defines: recur.capability.query-surface list explain and doctor commands for capability cards
consumes: README.CORE.IMPROVEMENT28 canonical capability-card proposal
consumes: main.improvement.27 future warp command proposal described by .recur-warp before implementation
produces: recur.warp.capability readable root card for future warp semantics without public command exposure
triggers: recur.capability.card-authoring future optional command for creating or repairing missing cards
```
