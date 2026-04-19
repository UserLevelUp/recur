RECUR IMPROVEMENT 23
Vault Observability & Pub/Sub-Native Coordination Substrate
===========================================================
Date: April 18, 2026
Status: Proposal / future direction
Author: Proposed from 2026-04-18 multi-lane coordination session (Lane D close-out)

INTENT
------
This document proposes the next improvement for a problem recur is now close to
but does not yet solve cleanly:
multi-agent vault coordination already has a shared state substrate, but it
still routes too much awareness through a human:
- one lane flips `status.current`
- another lane writes `last-run.current`
- a coordinator should react
- instead the human relays "yo dude, the thingy responded"
That does not scale past a small number of lanes.
The missing primitive is not a broker, a queue, a daemon, or a new family of
tools.
The missing primitive is a way to subscribe to the filenames that already carry
eventness.
The vault already holds the events that matter:
- brief dispatches
- status flips
- executor reports
- capsule rewrites
- completion records
What recur lacks is a pattern-filtered subscription surface over those writes.
Improvement 23 proposes that surface:
- `recur watch` as the subscription primitive
- `recur doctor` as passive observability over vault inconsistencies
The goal is not to add infrastructure.
The goal is to let the existing naming doctrine do more of the work.

SUMMARY
-------
Improvement 23 proposes two deliverables:
1. `recur watch`
   A pub/sub-native event subscription command over the vault.
   It subscribes by filename pattern, emits one line per event, and supports
   two modes selected by a flag: default streaming or crash-safe polling via
   `--poll-framing <seconds>`.
2. `recur doctor`
   A passive observation command that walks `.recur/` and reports obvious
   inconsistencies in coordination state.
   It does not repair anything.
   It tells the human what is weird.
Short version:
the filesystem already is the event log.
Improvement 23 makes it the subscription substrate too.

THE PROBLEM
-----------
The current coordination layer has already proven a useful happy path, but the
observation loop is still too manual.
Three pressures make that visible.
First, the substrate is forced.
As articulated in `memory/project_forced_filesystem_api.md`, CLI runtimes do
not expose a shared IPC surface to each other.
Multiple agents can all read and write files.
They cannot all hold a shared message bus.
Vault-as-API is not a preference.
It is the only common surface.
Second, the human as message passer does not scale.
At two lanes, a human can relay:
- "test-monkey finished"
- "skippy1 raised green"
- "git-monkey can fire"
At higher lane counts that becomes operational drag.
The human becomes the subscription primitive by accident.
That is the exact layer recur should delete.
Third, whiteboarding recovery infrastructure this early is a trap.
As articulated in `memory/project_failure_mode_evolution.md`, the right first
move is to ship the happy path, let real usage name the failure modes, and add
passive observation before active repair.
Premature lock, retry, broker, or reconciler design adds machinery before the
actual coordination substrate is even present.
The narrow gap is simpler than that:
- the vault already stores the events
- the naming doctrine already defines their hierarchy
- recur already has a pattern language over filenames
- there is no subscription primitive over that pattern language yet
Improvement 23 closes that gap without inventing a second system.

THE CORE IDEA
-------------
The governing doctrine is now explicit in
`memory/project_eventness_as_pubsub.md` and
`memory/eventness_conventions.md`:
**filename eventness IS the pub/sub topic hierarchy.**
That means pub/sub terminology already has a direct eventness equivalent:
| Pub/Sub concept | Eventness equivalent |
|---|---|
| Publish | Writing a file to the vault |
| Topic | Eventness suffix (or prefix+suffix combination) |
| Subscribe | `recur watch --filter <pattern>` |
| Topic hierarchy | Filename prefix/baseline/suffix doctrine |
| Message body | File contents |
| Message timestamp | File mtime |
| Offset tracking | File mtime per subscriber |
| Consumer group | Each watcher process is independent |
| Retention | `.current` -> `.complete` -> `.resolved` lifecycle |
| Durability | Filesystem provides it natively |
This produces the load-bearing property:
**subscription language == query language**
`recur watch --filter <pattern>` should accept the same pattern language
`recur files`, `recur tree`, and `recur find` already accept.
That is not sugar.
That is the reason a broker layer is unnecessary.
Canonical subscription examples:
- `**.status.current` - all lane state flips
- `**.last-run.current` - all executor reports
- `**.work.current` - all briefs dispatched
- `**.recur.md` - all capsule rewrites
- `**.complete.md` - all lane completions
- `<lane>.**` - everything a single lane does
- `**.current` - the live queue, everything in flight
The consequence is structural:
publish does not need a new verb.
A file write already is publish.
subscribe does not need a new topic DSL.
Filename patterns already are the topic tree.
retention does not need a broker policy.
Eventness lifecycle already gives one.
Improvement 23 is therefore not "add messaging."
It is "expose subscription over the naming system already present."

DELIVERABLES
------------
Improvement 23 proposes two deliverables that live at different layers.
### Deliverable: `recur watch`
`recur watch` is the subscription primitive over the vault.
Proposed surface:
```text
recur watch [--filter <pattern>] [--dir <path>] [--format <oneline|json>] [--poll-framing <seconds>]
```
Command meaning:
- `--filter` selects which filenames to observe using the same glob language
  recur already accepts elsewhere
- `--dir` scopes the watch to a subtree, defaulting to `.`
- `--format` selects emitted event serialization: `oneline` by default or
  `json` when machine consumption matters
- `--poll-framing <seconds>` selects poll mode
Mode behavior:
- omitted `--poll-framing`
  filesystem-event streaming mode
  fast, notify-based, process-lifetime bound
  best for humans in a terminal tab and short-lived CI hooks
- present `--poll-framing <seconds>`
  poll mode
  every N seconds, list files matching `--filter`, compare mtimes against the
  previous tick, emit events for anything created, modified, or deleted
  crash-safe, stateless, coordinator-friendly
Default event format:
```text
<unix-ts> <event-type> <path>
```
This command runs until SIGINT.
It is not a daemon.
It is not a service.
It is a process-lifetime subscription surface over filesystem-native state.
### Why the single-command, flag-selected-mode design
The design is locked:
- one `recur watch` command
- same pattern language in both modes
- same output formats in both modes
- same scope fence in both modes
- different "when/how" selected by a flag
That follows the rule articulated in `memory/feedback_mode_selector_flag.md`:
if the mental model is "same thing, different mode," use a flag.
Streaming and polling are both watching.
They are not different commands.
The split is load-bearing because the two consumers differ:
- a human in a terminal tab wants low-latency streaming
- an async coordinator that may be yanked by a CLI runtime timeout wants a
  fresh vault read each tick and no held subprocess state
One command keeps the mental model small.
The mode flag changes timing semantics, not conceptual ownership.
### Time Value Convention
`--poll-framing` takes plain integer seconds.
- `--poll-framing 5` is valid
- `--poll-framing 5s` is invalid
- no millisecond variant exists
- no duration parser exists
This follows the locked recur convention:
all time values are integer seconds everywhere.
### Deliverable: `recur doctor`
`recur doctor` is passive observation over coordination state.
Proposed surface:
```text
recur doctor [--dir <path>]
```
Purpose:
- walk `.recur/`
- inspect vault state
- report obvious inconsistencies
- repair nothing
Examples of inconsistencies worth surfacing:
- status says `active` but no corresponding `work.current` file exists
- `work.current` is present but status still says `idle`
- `last-run.current` is older than the status file's mtime
- a lane is stuck `active` for too long without a fresh progress log entry
This is deliberately an envelope-check tool.
It does not become a reconciler on first ship.
It does not auto-heal desks.
It surfaces weirdness so a human can decide.

NON-GOALS
---------
Improvement 23 is explicit about what it does not propose:
- NOT building `recur-message`
- NOT building `recur-timer`
- NOT adding a daemon, server, or background service
- NOT adding auto-repair to `recur doctor`
- NOT shipping retry, lock, or reconciler logic on first pass
- NOT introducing a second topic language separate from recur's existing
  filename pattern language
These non-goals matter because they prevent the proposal from re-inflating
into infrastructure.

FAILURE-MODE TAXONOMY
---------------------
Improvement 23 should be understood through the failure taxonomy already named
in `memory/project_failure_mode_evolution.md`.
Expected failures live inside the normal operating envelope:
- Gate 1 rejects
- tests fail
- a coordinator dispatches correction work
Those need vocabulary, not heroics.
Unexpected failures are off the rails:
- a worker crashes mid-run
- a flag stays `active`
- filesystem state drifts from reality
- two desks appear live when only one should be
Improvement 23 responds to this taxonomy with discipline:
- `recur watch` gives the subscription primitive for seeing normal events
- `recur doctor` gives passive observation for weird states
- neither deliverable tries to auto-repair the system on first ship
That is the correct posture.
Ship the happy path.
Observe real usage.
Add repair only when the failure mode is proven and named.

COMPOSITION WITH ASYNC COORDINATOR
----------------------------------
The target coordinator shape is already articulated in
`memory/project_self_terminating_coordinator.md`.
That target is a self-terminating async coordinator:
- it wakes on a tick
- it carries session memory
- it hard-stops on validated completion
The vault is the coordinator's termination-predicate source.
That means Improvement 23 composes as follows:
- the coordinator uses `recur watch --poll-framing 5` or a similar integer
  interval
- each tick is a fresh vault read
- no held subprocess state is required between ticks
- if the coordinator is yanked by a runtime timeout, the next session resumes
  cleanly from vault state alone
This is why polling mode exists inside `recur watch` instead of as a sibling
command:
- default streaming mode is for humans in a tab and bounded-lifetime hooks
- poll mode is for async coordinators that need crash-safe re-entry behavior
Role assignment stays clean:
- async-capable substrates take coordinator roles
- reactive substrates take executor roles
No daemon is introduced.
The hard-stop remains load-bearing.

CLOSING
-------
Improvement 23 says something narrow and consequential:
`recur watch` should be the subscription primitive for both humans and
coordinators, and `recur doctor` should be the passive observation tool for
vault inconsistencies.
Everything else composes on top of that:
- the vault produces the events
- `recur watch` emits them
- coordinators consume them
- `recur doctor` surfaces states that look wrong
The design rule is equally clear:
- prefer flag-selected modes over sibling commands
- keep one mental model
- keep one pattern language
- keep one scope fence
The subscription substrate does not need a broker.
It needs recur to admit what the naming doctrine already made true.
