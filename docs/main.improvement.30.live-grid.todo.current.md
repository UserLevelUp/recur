# Improvement 30: Living Master Work Report

Status: `todo.current`
Priority: `important`
Date: 2026-07-24
Parent: `README.CORE.IMPROVEMENT30.md`
Contract: `docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`
Fixture: `demos/main.lang/main.lang.skippy-watch-coordination.recur`

## Objective

Make the block-level coordination grid a first-class Recur Lang projection so
a human, coordinator, or intelligence can see parallel work change as it
happens and then retain the same information as the completed master work
report.

```text
Lane             Current block       State       Evidence
------------------------------------------------------------
csharp-monkey    csharp.f(a)          WORK        2 files
web-monkey       web.f(a)             PRODUCED    npm.test ACK
test-bird        test.f(a)            BLOCKED      question.002
review-bird      review.i(a)          WAIT 2/3     -
git-monkey       git.i(a)             WATCH        -
------------------------------------------------------------
Overall          implementation       ACTIVE       merge-ready: false
```

## Why This Is Current

The grid turns contracts, parallel lanes, watcher state, WorkOrders, receipts,
and Eventness into one immediately readable control-room view. It also provides
the missing bridge between live coordination and a durable explanation of how
the final branch or commit set was produced.

This is important enough to track now, but it should be built in bounded
slices. The continuous display must not become an independent dashboard state
store.

## Governing Invariant

```text
live grid == snapshot grid == completed report
```

All three are projections over the same canonical AST and durable Eventness.
Restarting `recur-lang` must reconstruct the same cell states without relying
on process-local memory.

## Command Boundary

Pure snapshot:

```powershell
recur lang grid solution
recur lang grid solution --json
```

Live companion:

```powershell
recur-lang coordinate demos/main.lang/main.lang.skippy-watch-coordination.recur --view grid
```

Subscription and watcher status:

```text
recur-watch = active blocking filesystem subscription
recur watch = pure watcher-state query and exit
```

The master grid belongs to `recur-lang`, not `recur-watch`, because it needs
contracts, joins, attempts, blocks, and receipts rather than raw file activity.

## Grid Cell Contract

Each lane cell should contain:

```text
lane
persona
host / CLI window
current compact block
watching or working mode
Eventness state
round and attempt
watcher state and ACK
dependency readiness
evidence receipts
blockers
last event time / progress age
```

The overall row should contain:

```text
run and source hash
current phase
ready / active / blocked / complete lane counts
missing joins
cycle and deadlock status
scope conflicts
required evidence completeness
verification
merge-ready
```

## Progressive Drill-Down

```text
grid
  -> lane
    -> contract
      -> current WorkOrder
        -> changed files or artifacts
          -> external tool receipts
            -> exact Eventness timeline
```

Text and JSON must expose the same underlying fields. An IDE, terminal art
view, or web view may render them differently without changing their meaning.

## Event-Driven Refresh

Relevant events include:

- watcher accepted, rejected, stale, or stopped;
- WorkOrder published or acknowledged;
- lane claimed, working, blocked, or awaiting guidance;
- question or NAK published;
- receipt produced;
- dependency join satisfied;
- review accepted or rejected;
- integration receipt accepted;
- session completed.

Raw watcher notifications are wake-up signals. The renderer rereads and
validates durable state before changing a cell.

## Eventness Lifecycle

```text
main.improvement.30.live-grid.todo.current
  -> main.improvement.30.live-grid.contract.complete
  -> main.improvement.30.live-grid.snapshot.complete
  -> main.improvement.30.live-grid.live-view.complete
```

For each coordinated solution:

```text
solution.coordination.current
  -> solution.coordination.complete
```

The completed report preserves final cells, transitions, attempts, questions,
receipts, commits, verification evidence, and the merge decision.

## First Bounded Slice

Freeze and test `grid-report-v0` without starting a live renderer:

1. define a versioned Julia/JSON shape for the overall report and lane cells;
2. derive a snapshot from a fixed coordination fixture;
3. render deterministic text and JSON;
4. prove blocked, waiting, working, produced, and complete cells;
5. prove a restart produces byte-equivalent normalized JSON;
6. expose no mutation or watch loop from the pure query path.

Only after this contract is stable should a companion subscribe and refresh
the grid continuously.

## Acceptance Criteria

- Parallel lanes occupy independent rows and update independently.
- A join displays both its required and received exact receipt references.
- Watcher health is distinct from lane work state.
- Duplicate file notifications do not create duplicate transitions.
- A blocked lane exposes its question and the coordinator it awaits.
- A stale lane is visible without being silently reassigned.
- Every cell drills down to source-backed evidence.
- Text and JSON snapshots agree.
- Restart reconstruction does not require hidden memory.
- The final `coordination.complete` report derives from the same event history.
- Color may assist rendering but is never the only state indicator.

## Non-Goals For The First Slice

- no full-screen TUI;
- no browser dashboard;
- no arbitrary command execution;
- no automatic merge or approval;
- no target-language compiler integration;
- no process-local state that cannot be rebuilt;
- no separate dashboard database.

## Warp

```text
E0(main.improvement.30.live-grid.todo.current)
  -> dE(grid-report-v0 contract and pure snapshot)
  -> Ef(main.improvement.30.live-grid.contract.complete)
```

## Discovery

```powershell
recur tree "main.improvement.30.live-grid" -d docs/
recur files "main.improvement.30.**.todo.current" -d docs/
recur files "main.lang.skippy-watch-coordination" -d demos/main.lang/
recur trace-id "recur.lang.master.work.report" --scope "**" --ext ".md" -d .
```

## Trace-Id Lines

```text
defines: main.improvement.30.live-grid current important lane for the living master work report
defines: recur.lang.master.work.report dynamic lane grid and durable completed coordination audit
defines: recur.lang.grid.report.v0 future versioned snapshot and JSON contract
consumes: main.improvement.30 Recur Lang coordination control-plane proposal
consumes: main.improvement.30.contract.watch-coordination-v0 formal watch work receipt and join contract
consumes: main.command.watch recur-watch active subscription and pure watcher state query
produces: solution.coordination.current live report reconstructed from coordination Eventness
triggers: main.improvement.30.live-grid.contract freeze grid-report-v0 before continuous rendering
```
