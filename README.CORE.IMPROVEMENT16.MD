# Core Improvement 16: Eventness Realm Traversal and Drift Guardrails

**Status:** Future Vision (Long-Distance Backlog, Not Active)  
**Priority:** High  
**Category:** Eventness Operations, Cross-Realm Traversal, Drift Detection

## Why This Exists

Improvement 16 was initially conceived in a project context where only the `recur` and `recur-git` binaries (plus `--help`) were available.

This document translates that early idea into repo-native, actionable language so activation later is straightforward and low-risk.

## Execution Posture

- Improvement 16 is not active now.
- Keep it in future-plan posture until explicitly activated.
- No implementation work should start from this document alone.
- Current purpose: clarify terminology, define activation gates, and predefine the operator workflow.

## Current Capability Baseline (Reality Check)

Today, recur is strongest in cross-file hierarchy traversal:

- between files with similar names
- across one or more separators (`.`, `_`, `-`, `:`)
- from a chosen base directory/lane

In-file hierarchy support exists only in limited form right now and is not yet the primary operational layer.

Improvement 16 assumes this baseline and does not pretend in-file traversal is fully mature yet.

## Core Goal (Future)

Enable reliable traversal across eventness realms:

1. file hierarchy realm (`docs/`, `src/`, lane separators)
2. git realm (changed/staged/history file sets via stdin pipelines)
3. in-file realm (symbols/anchors/sections when capabilities mature)
4. composed realm (multi-stage piped recur queries that progressively filter eventness)

The target is not just "find files" but "move between realms without losing context."

## Problem Statement

As eventness grows across `docs/`, `src/`, and `.recur/`, it becomes easier to lose operational clarity:

- which lanes are active now
- what was recently completed
- where stale overlap exists (`.todo` + `.complete` on same base)
- where lane/separator drift is happening in `.recur/config.toml`
- whether users are running the intended binary (repo build vs global install)
- how to connect "what changed in git" with "what matters in hierarchy" and eventually "what matters inside files"

Improvement 16 focuses on operational visibility plus cross-realm traversal guardrails, not a brand-new parser.

## Terminology (Canonical)

- **Lane**: a configured project segment with a directory and separator (for example `docs/.`, `src/_`).
- **Base**: durable work subject (`main.command.trace-stats`, `main.improvement.7.phase3`).
- **Suffix**: eventness state signal (`todo.current`, `todo.trigger.event`, `complete`).
- **Eventness signal**: any suffix-bearing artifact indicating attention priority.
- **Drift**: mismatch between expected and current structure/state (separator drift, stale files, missing trigger chains).
- **Snapshot**: point-in-time summary of eventness and drift metrics.
- **Operator loop**: repeated discovery -> verify -> act -> record flow.
- **Realm**: a query domain with different source and granularity (files, git, in-file anchors, composed pipeline).
- **Traversal**: moving a query/result set from one realm into another without manual re-discovery.

## Scope (When Activated)

Improvement 16 is a query-first operations layer built on existing commands:

1. Eventness discovery from existing hierarchical artifacts.
2. Drift checks for lane config and suffix consistency.
3. Cross-realm traversal workflows (files <-> git <-> in-file <-> composed pipelines).
4. Optional snapshot/report artifacts under `.recur/`.

It does not require a new core parser or recursive model.

## Proposed Future Eventness Suffixes

These are future suffixes for clarity when Improvement 16 activates:

- `todo.current` (active lane)
- `todo.trigger.event` (runnable operator commands)
- `todo.audit.event` (scheduled health-check actions)
- `todo.drift.current` (active drift resolution work)
- `risk.high` / `risk.medium` / `risk.low`
- `anomaly.detected`
- `complete`
- `complete.reviewed`

Guideline: keep suffixes composable and right-to-left parseable per `README.CORE.EVENTNESS.md`.

Optional realm-oriented qualifiers (future):

- `todo.current.realm.files`
- `todo.current.realm.git`
- `todo.current.realm.in-file`
- `todo.current.realm.composed`

## Activation Gates

Only activate Improvement 16 when all are true:

1. Current active lane is stable enough to accept ops hardening work.
2. Team agrees on suffix policy map and severity meanings.
3. `.recur/config.toml` lane model is accepted as source of truth.
4. Baseline test posture is known and recorded at activation time.

## Operator Workflow (Actionable, Future)

### Step 1: Discover Current Work

```bash
recur files "**.current" -d docs/
recur files "**.trigger.event" -d docs/
recur tree "main" --sep . --sep _ --show-sep
```

### Step 2: Discover Recent Completion

```powershell
recur files "**.complete" -d docs/ |
  ConvertFrom-Json |
  ForEach-Object { Get-Item $_ } |
  Sort-Object LastWriteTime -Descending |
  Select-Object LastWriteTime, Name
```

### Step 3: Detect Stale Overlap (`todo` + `complete`)

```powershell
$todo = recur files "**.todo" -d docs/ | ConvertFrom-Json | ForEach-Object { $_ -replace '\.todo\.md$','' }
$done = recur files "**.complete" -d docs/ | ConvertFrom-Json | ForEach-Object { $_ -replace '\.complete\.md$','' }
$todo | Where-Object { $done -contains $_ } | Sort-Object -Unique
```

### Step 4: Run Lane Drift Check

```bash
recur init --analyze --json
```

Review at least:

- `additions`
- `separator_updates`
- `missing_directories`

### Step 5: Confirm Runtime Binary Is Intended

```powershell
Get-Command recur | Select-Object Source
target/debug/recur.exe --version
```

If sources differ, record a drift/anomaly event before continuing.

## Realm Traversal Workflow (Actionable, Future)

This is the specific Improvement 16 path.

### Realm A -> Realm B: Git to File Hierarchy

```bash
git diff --name-only | recur files "**" --stdin
git diff --name-only | recur files "main.command.**" --stdin --sep . --sep _
```

### Realm B -> Realm C: File Hierarchy to In-File (when mature)

```bash
recur files "main.command.trace-stats.**" -d src/ --sep _ \
  | recur id "risk.**" --stdin
```

### Realm C -> Realm D: Compose Filters for High-Signal Eventness

```bash
recur files "**.current" -d docs/ \
  | recur files "**.trace-stats.**" --stdin \
  | recur files "**.risk.high.**" --stdin
```

Note: final command shapes may change as in-file capabilities evolve.

## Proposed `.recur` Artifacts (Future, Optional)

```text
.recur/
  eventness/
    snapshots/
      2026-03-01T12-00-00Z.status.json
    reports/
      2026-03-01.status.md
    policy/
      suffix-policy.toml
```

Notes:

- These are metadata artifacts only.
- Do not duplicate source code or prose bodies.
- Keep records small and queryable with existing `recur` commands.

## Snapshot Contract (Draft)

```json
{
  "ts": "2026-03-01T12:00:00Z",
  "active_current_count": 3,
  "complete_count": 27,
  "todo_count": 12,
  "trigger_event_count": 1,
  "stale_overlap_bases": [
    "docs/main.command.merge.stdin.windows.bom"
  ],
  "drift": {
    "lane_additions": 1,
    "separator_updates": 1,
    "missing_directories": 0
  },
  "binary_source": "C:\\src\\recur\\target\\debug\\recur.exe"
}
```

## Success Criteria

- A newcomer can identify active lanes and recent completions in under 5 minutes.
- Drift is visible through one repeatable command sequence.
- Cross-realm traversal works as a repeatable operator loop (git -> files -> in-file/composed).
- Snapshot output is deterministic and script-friendly.
- No additional memory burden is required to recover working context.

## Non-Goals

- Replacing issue trackers or PR review systems.
- Storing full code or prose payloads in `.recur` artifacts.
- Introducing complex autonomous scheduling in v1.
- Claiming full in-file semantic traversal before supporting capabilities are production-ready.

## Dependencies and Alignment

- Aligns with `README.CORE.EVENTNESS.md` (prefix/base/suffix model).
- Complements Improvement 14 (mirror eventness) without requiring full mirror implementation first.
- Coexists with Improvement 15 future recomposition work.

## First Activation Checklist (Future)

1. Create `docs/main.improvement.16.todo.current.md`.
2. Create `docs/main.improvement.16.todo.trigger.event.md` with the operator workflow commands.
3. Record baseline counts (`current`, `complete`, `todo`, `trigger.event`).
4. Record lane drift output from `recur init --analyze --json`.
5. Create first `.recur/eventness/snapshots/*.status.json`.
6. Define `suffix-policy.toml` with explicit severity weights.
7. Run full validation loop and capture outcomes in `docs/main.improvement.16.phase1.status.md`.

## One-Line Summary

Improvement 16 is the future cross-realm operator layer for traversing file, git, in-file, and composed eventness views with consistent, query-first guardrails.
