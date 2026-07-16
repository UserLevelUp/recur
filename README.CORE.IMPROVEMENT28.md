

RECUR IMPROVEMENT 28
Capability Cards and Explain Commands
=====================================
Date: May 23, 2026
Status: Implemented seed query surface
Author: Captured from a 2026-05-23 design discussion

INTENT
------
Make recur capability surfaces self-describing from project files. A user or
agent should be able to ask what a recur technology does before using it.

Improvement 28 introduces root-level capability cards and a read-only command
surface:

```text
recur capability list
recur capability explain warp
recur capability explain watch
recur capability explain git
recur capability explain trace-id
recur capability explain reveal
recur capability doctor
```

FILE CONVENTION
---------------
Capability cards are root-level files whose names start with `.recur-`:

```text
.recur-warp
.recur-watch
.recur-git
.recur-trace-id
.recur-reveal
```

These cards are intentionally outside `.recur/` so they can be shared project
knowledge rather than local-only private state. Each card should explain:

- what the capability offers;
- which commands or file patterns it uses;
- what is implemented now versus future/proposed;
- how it powers a lane;
- what future tooling can build from it.

COMMAND BEHAVIOR
----------------
`recur capability list` lists root `.recur-*` cards.

`recur capability explain <name>` prints the selected card. The name may be
given as `warp` or `.recur-warp`.

`recur capability doctor` checks the standard seed cards: `warp`, `watch`,
`git`, `trace-id`, and `reveal`. Doctor is diagnostic and read-only; missing
cards are reported without writing files.

WHY THIS BELONGS IN RECUR
-------------------------
Recur is becoming a portable cognition substrate across local tools, agents, and
LLM workflows. Capability cards keep that substrate legible. They prevent each
operator from rediscovering what `warp`, `watch`, `git`, `trace-id`, and
`reveal` mean by inference.

RELATION TO IMPROVEMENT 27
--------------------------
Improvement 27 proposes `recur warp` as a future eventness-control command
family. Improvement 28 labels the broader recur control surface so that warp,
watch, git, trace-id, reveal, and later capabilities can describe themselves
before they are composed.

TRACE-ID LINES
--------------

```text
defines: main.improvement.28 recur capability-card query surface for root .recur-* files
defines: recur.capability.list read-only command that lists capability cards discovered at the project root
defines: recur.capability.explain read-only command that prints one capability card by name
defines: recur.capability.doctor read-only diagnostic command for required seed capability cards
consumes: main.improvement.27 recur warp and project-control command proposal for eventness optimality scoring
consumes: recur.warp.capability ideal-state lane steering and eventness optimality surface
consumes: recur.watch.capability live filesystem eventness stream for lane state changes
consumes: recur.git.capability git branch head worktree diff commit and push evidence for lane truth
consumes: recur.trace-id.capability role-classified flow evidence for dotted eventness identifiers
consumes: recur.reveal.capability file-backed persona role constraint and handoff rehydration
produces: recur.capability.cards root-level self-describing .recur-* files for shared human agent and local tool understanding
produces: recur.capability.query-surface list explain and doctor commands for capability cards
triggers: recur.capability.card-authoring future command or docs path for creating missing seed cards
```