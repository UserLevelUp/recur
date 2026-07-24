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

Operational repo shape often looks like:

`<prefix>.<base>.<suffix>[.<eventness>][.<ext>]`

In practice:
- `suffix` is usually the durable artifact or lane anchor
- the extra right-hand eventness chain expands or collapses current interest
- `.ext` is optional; most files have one, some do not

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

## Operational Lifecycle

This repo uses eventness as a recurring pattern:

### 1) Expansion

When interest is live, expand around a stable subject:

`prefix.base.suffix[.expanding.eventness][.ext]`

Examples:
- `main.command.trace-id.test.todo.current.md`
- `main.command.trace-id.test.todo.current.reference.md`
- `main.command.trace-id.test.todo.trigger.event.md`

Expanded files can hold:
- resume context
- hypotheses
- exact `recur` commands to run next
- references to related files or lanes

### 2) Interest Discovery

During expansion, use `recur` commands to discover what is actually interesting now.
The files do not replace query-time discovery; they improve it.

Typical loop:
- `recur files "**.current" -d docs/`
- `recur files "**.trigger.event" -d docs/`
- `recur tree "main" --sep . --sep _ --show-sep`
- `recur find "<symbol-or-text>" --scope "**" -d src/`

This is why eventness works well with humans and LLMs: the repo stores both the current state and the next useful queries.

### 3) Collapse

Once the interest has paid off, collapse the expanded state:

`prefix.base.suffix[.collapsing.eventness][.ext]`

Typical outcomes:
- keep a stable completion record: `*.complete.*`
- keep a lower-intensity reminder: `*.future-plan.*`
- keep a durable rediscovery point: `*.recurring.*`
- remove the ephemeral file entirely when no residue is worth keeping

### 4) Recurring Rediscovery

`recurring` is apt when the value is not "this is active now" but "this workflow or pattern should be easy to find again later."

---

## Scoped Completion and Subsystem Integration

Eventness belongs to the stable subject named by its prefix and base. Interest
and completion do not automatically propagate from a child subject to a parent
subject.

For a Recur Lang subsystem, these are distinct facts:

```text
game.pathing.implementation.complete
game.pathing.verification.accepted
game.pathing.integration.ready
game.gameplay.pathing.integration.accepted
```

Their intended meanings are:

- `implementation.complete`: the child produced its declared artifacts;
- `verification.accepted`: the child's required evidence was accepted;
- `integration.ready`: the child published an accepted public contract and
  receipt for a parent to consume;
- `integration.accepted`: a particular parent accepted that exact child
  contract into its own coordination boundary.

Child completion never implies parent completion. A pathing subsystem can be
complete and integration-ready while gameplay is still working, blocked, or
has rejected the pathing result.

The readiness record should identify the public contract version or content
hash. A parent acceptance record should identify the same value. If the child
boundary changes, the earlier parent acceptance is stale and must not silently
apply to the new boundary.

These suffixes are an Improvement 30 policy proposal, not automatically
recognized core states. Register any adopted multi-segment suffixes in the
suffix policy map before depending on parsing, ranking, or transition checks.
The general Eventness rule is stable: observe and rank each subject at its own
scope, then connect child and parent facts with explicit evidence.
The formal subsystem contract and composition rules are proposed in
`README.CORE.IMPROVEMENT30.md`.

---

## Eventness Score

Eventness is a ranking score for "what should get attention now."

Suggested formula:

`eventness = suffix_weight + prefix_prior + freshness + impact + anomaly_bonus`

Where:
- `suffix_weight`: strongest signal (blocked > current > todo > recurring > complete)
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
- expanded eventness never collapsed after the interest window closed
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
- `main.command.trace-id.test.todo.current.reference`
- `main.command.checkpoint.todo.next`
- `main.package.crates-io.recurring`
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

### Recurring rediscovery (today)
```bash
recur files "**.recurring" -d docs/
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
6. Collapse the expanded eventness to the right durable residue.
7. Run recurring completion checklist items (update docs, commit, push).
8. Rotate current lane.

Humans and LLMs should run the same loop over the same semantic list plus resolved file context.

---

## Minimum Governance

Keep this lightweight:

1. Maintain a suffix policy map (suffix -> weight/severity).
2. Enforce one active `*.todo.current` per lane.
3. Require `*.todo.trigger.event` for recurring start/complete workflows.
4. Collapse stale expanded eventness once the interest window closes.
5. Run drift checks (missing suffix chains, unresolved refs).

---

## One-Line Summary

Prefix and base tell you where work lives; suffix tells you why it is interesting now.

For composed systems, child readiness and parent acceptance remain separate,
explicit Eventness facts.
