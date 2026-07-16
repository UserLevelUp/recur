# main.improvement.27.command-boundary.todo.future-plan

Status: `future-plan`
Date: 2026-05-23

## Purpose

Keep the warp idea from leaking into the wrong command shape.

The immediate design choice is simple:

```text
recur warp      = future read-only query surface
recur-warp      = possible future companion for approval-gated automation
```

No `warp` command should ship until the read-only query surface is stable.

## Core Command Boundary

Future `recur warp` may:

- read files under a lane scope
- count state suffixes
- classify trace-id role lines
- load `.recur/config.toml` scoring defaults
- emit text and JSON
- suggest next actions

Future `recur warp` must not:

- rename files
- delete files
- collapse eventness
- stage or commit changes
- approve a transition
- encode private/project-specific personas or product rules
- start active watchers or daemons

## Companion Boundary

A future `recur-warp` companion may be considered only after the read-only query
surface is boring and tested.

Possible companion responsibilities:

- consume a collapse plan
- require explicit operator confirmation
- write ACK/NAK status under `.recur/warp/`
- preserve before/after manifests
- integrate with future version or lineage tools

That companion should not be part of the first implementation slice.

## Relationship To Existing Commands

`recur warp` should compose existing primitives rather than replacing them:

- `recur files` enumerates evidence
- `recur tree` explains shape
- `recur related` shows sibling pressure
- `recur reveal` restores lane-local context
- `recur trace-id` classifies role evidence
- `recur watch` reads watcher state
- `recur version` reads artifact version policy and history

## Release Boundary

Do not ship this in the current packaging lane.

Candidate release horizon:

```text
0.2.20 or later, after contract and fixture lanes stabilize
```

## Trace-Id Lines

```text
defines: main.improvement.27.command-boundary future split between recur warp read-only query and recur-warp companion automation
defines: recur.warp.core-boundary read-only files tree related reveal trace-id watch and version composition without writes
defines: recur-warp.companion-boundary possible future approval-gated write-side collapse automation
consumes: main.improvement.27 recur warp and project-control command proposal for eventness optimality scoring
produces: recur.warp.release-boundary do not ship in current packaging lane and consider 0.2.20 or later
triggers: main.improvement.27.contract.warp-status-v1 stabilize read-only status contract before any implementation
```
