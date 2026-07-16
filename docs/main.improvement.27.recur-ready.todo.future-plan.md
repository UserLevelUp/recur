# main.improvement.27.recur-ready.todo.future-plan

Status: `future-plan`
Date: 2026-05-23

## Purpose

Make the warp proposal implementation-ready in the recur way: discoverable by
hierarchy, bounded by command surface, and testable before it returns to `src/`.

This file does not mean `recur warp` is active or shipped. It is a readiness
gate for a later implementation pass, likely after the current packaging lane.

## Current State

- Root proposal exists: `README.CORE.IMPROVEMENT27.md`
- Docs bridge exists: `docs/main.improvement.27.todo.future-plan.md`
- Capability card exists: `.recur-warp`
- Command implementation is parked and should not appear in `recur --help`
- Release horizon is later maturity work, likely around `0.2.20`

## Recur-Ready Criteria

Before implementation resumes, the lane should have:

- a frozen-ish JSON contract for `warp-status-v1`
- synthetic fixture names for optimum, sub_optimum, and blocked lanes
- a clear rule for what counts as evidence vs inference
- a stable scoring vocabulary with default weights
- a future-state convergence vocabulary for target states, semi-states, and
  supersystem residuals
- a temporal projection vocabulary for eventness membranes, horizons, and
  frame-by-frame residual changes
- an eventness epic and milestone vocabulary for expert-authored complete,
  pending, research, and blocked buckets, with epoch reserved for long horizons
- a documented split between `recur warp` and any future `recur-warp`
- explicit non-goals for write-side collapse, persona behavior, and active
  watching
- a root `.recur-warp` card that explains the implemented/proposed split
- one doc-only dry run over existing eventness lanes using current primitives

## Dry-Run With Existing Commands

The first dry run should use only existing commands:

```powershell
recur files "main.improvement.27.**" -d docs/
recur tree "main.improvement.27" -d docs/
recur trace-id "recur.warp.status" --scope "main.improvement.27.**" --dir docs --ext md --json
recur files "**.current" -d docs/
recur files "**.complete" -d docs/
```

The goal is to write down what those commands reveal and only then decide what
`recur warp status` would add.

## Implementation Return Gate

Do not reintroduce Rust code until these docs are good enough that a new agent
can implement from the eventness tree alone:

```powershell
recur tree "main.improvement.27" -d docs/
recur files "main.improvement.27.**.todo.future-plan" -d docs/
```

## Trace-Id Lines

```text
defines: main.improvement.27.recur-ready future readiness gates for making recur warp implementation-ready without shipping it
consumes: main.improvement.27 recur warp and project-control command proposal for eventness optimality scoring
consumes: recur.files existing file enumeration primitive for dry-run evidence
consumes: recur.tree existing hierarchy shape primitive for dry-run evidence
consumes: recur.trace-id existing role-classification primitive for dry-run evidence
consumes: recur.warp.status future read-only status command as the implementation target
consumes: recur.warp.future.state.convergence intended future eventness state and intermediate semi-state framing
consumes: recur.warp.temporal.frame.projection bounded projection over now day month year and decade eventness frames
consumes: recur.warp.eventness.epic expert-authored horizon target complete pending and research milestone frame
consumes: main.improvement.28 recur capability-card query surface for root .recur-* files
produces: recur.warp.readiness implementation return gate based on contract fixtures command boundary and dry-run evidence
triggers: main.improvement.27.contract.warp-status-v1 define JSON contract before code returns
triggers: main.improvement.27.command-boundary keep core query and companion automation separated
triggers: main.improvement.27.epic.milestone keep epic milestone planning separate from first command implementation
```
