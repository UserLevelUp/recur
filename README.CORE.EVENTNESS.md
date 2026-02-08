# Core Eventness Model
### Prefix/Base Routing + Suffix Interest

## Status

Concept draft aligned to IMPROVEMENT6/9 direction.

---

## Purpose

Define a practical naming and scoring model for "what is interesting now" across:
- code work
- docs and tests
- TODO lanes
- triggers and checkpoints
- human and LLM workflows

This document replaces abstract operator language with an operational model for this repo and similar systems.

---

## Core Thesis

Use hierarchical IDs in this shape:

`<prefix>.<base>.<suffix>[.<qualifier>]`

Where:
- `prefix` routes context (domain/lane/root)
- `base` identifies the stable work unit
- `suffix` encodes interest (state, trigger, risk, priority)

Short version:
- routing is prefix/base-driven
- attention is suffix-driven

---

## Meaning of Each Part

### 1) Prefix (Context Router)

Examples:
- `main`
- `ops`
- `release`
- `product`
- `research`

Use `prefix` to segment search space and ownership boundaries.

### 2) Base (Stable Subject)

Examples:
- `command.tree`
- `command.checkpoint`
- `auth.session`
- `search.ux`

`base` should change slowly. It is the durable unit for history and metrics.

### 3) Suffix (Interest Signal)

Examples:
- `todo`
- `todo.current`
- `todo.next`
- `todo.tracking`
- `todo.priority`
- `todo.trigger.event`
- `risk.high`

`suffix` is what makes an item interesting right now.

---

## Parsing Rule (Important)

To keep naming flexible, parse from right to left:

1. Match the longest known suffix pattern.
2. Remaining left side: first segment is prefix.
3. Remaining middle segments are base.

Example:
- ID: `main.command.tree.todo.current`
- suffix: `todo.current`
- prefix: `main`
- base: `command.tree`

This avoids hardcoding base depth.

---

## Eventness Score

Eventness is a ranking score for "what should get attention now."

Suggested formula:

`eventness = suffix_weight + prefix_prior + freshness + impact + anomaly_bonus`

Where:
- `suffix_weight`: strongest signal (blocked > current > todo > complete)
- `prefix_prior`: domain urgency prior (`ops` may outrank `research`)
- `freshness`: time decay / staleness
- `impact`: downstream references, dependency fan-out
- `anomaly_bonus`: unknown patterns, drift, or surprising changes

Key policy:
- prefix guides search scope
- suffix dominates ranking

---

## Detecting the Unpredicted

You can detect useful surprises even when not explicitly planned.

Examples:
- new suffix pattern appears (`todo.blocker`, `risk.critical`) not in policy
- missing expected suffix chain (`todo` exists, `todo.current` missing)
- sudden rise in refs/callers around one base
- new prefix/base combination with no historical pattern

Treat these as anomaly candidates and raise eventness automatically.

---

## File Layer vs In-File Layer

This model supports both layers from IMPROVEMENT9:

1. File layer (`recur files/tree/stats`)
- selects where to look
- separator may vary by folder (`--sep _` for `src`, `.` for docs/tests)

2. In-file layer (`recur in id/refs/trace/gaps`)
- extracts and ranks interest IDs inside selected files
- canonical semantic IDs remain dot-based

---

## Example IDs

- `main.command.tree.todo.current`
- `main.command.tree.todo.trigger.event`
- `main.command.checkpoint.todo.next`
- `ops.incident.auth.outage.todo.blocker`
- `release.v2_3.rc1.risk.high`

Any suffix can represent interest as long as it is recorded consistently.

---

## Example Queries

### Current lane discovery (today)
```bash
recur files "main.command.*.todo.current" -d docs/
recur files "main_command_*_todo_current" -d src/ --sep _
```

### Trigger visibility (today)
```bash
recur files "main.command.*.todo.trigger.event" -d docs/
recur files "main_command_*_todo_trigger_event" -d src/ --sep _
```

### Interest extraction (IMPROVEMENT9 target)
```bash
recur files "main.command.**" -d docs/ \
  | recur in id "*.todo.*" --stdin

recur files "main_command_*" -d src/ --sep _ \
  | recur in refs "*.todo.trigger.event" --stdin
```

---

## Tracking Format for Existing Repos

For legacy codebases, start with one plain text list:

- `docs/main.semantic.names.txt`

Record one semantic ID per line:
- `prefix.base.suffix[.qualifier]`
- blank lines and `#` comments are ignored
- no NDJSON or embedded metadata schema required

Example:

```text
main.command.tree.todo.current
main.command.tree.todo.trigger.event
main.command.checkpoint.todo.current
```

Then resolve IDs to files with existing file-layer commands and pull detailed context from those files on demand.

---

## Human + LLM Operating Loop

1. Select scope by prefix/base.
2. Read candidate semantic IDs from the text list.
3. Resolve selected IDs to files (`recur files ...`) and extract suffix-bearing IDs (interest signals).
4. Rank by eventness.
5. Execute required triggers.
6. Run recurring completion checklist items (update docs, commit, push).
7. Rotate current lane.

Humans and LLMs should run the same loop over the same semantic list plus resolved file context.

---

## Minimum Governance

Keep this lightweight:

1. Maintain a suffix policy map (suffix -> weight/severity).
2. Enforce one active `*.todo.current` per lane.
3. Require `*.todo.trigger.event` for recurring start/complete workflows.
4. Run drift checks (missing suffix chains, unresolved refs).

---

## One-Line Summary

Prefix and base tell you where work lives; suffix tells you why it is interesting now.
