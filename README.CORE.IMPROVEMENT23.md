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
- `recur psyche` as passive observability over vault inconsistencies
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
2. `recur psyche`
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
### Deliverable: `recur psyche`
`recur psyche` is passive observation over coordination state.
Proposed surface:
```text
recur psyche [--dir <path>]
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

### Future extension: config-backed persona psyche
The next useful expansion is for `recur psyche` to inspect not only whether the
vault is structurally consistent, but whether the currently active persona is
meeting its own declared expectations.
This should be rooted in `.recur/config.toml`, because the active thread and
reveal policy already live there.

Proposed shape:
```toml
[psyche]
enabled = true
active_persona_from_reveal = true
grade_after_response = true
write_grade_to_config = true
collect_usage_evidence = true
suggest_reveal_rewrites = true

[psyche.expectations]
frustration_signals = "blocked,not working,did not set,missing,stuck,surprise"
positive_signals = "verified,working,set,passes,clear,expected"
grade_scale = "good-bad-surprise"

[psyche.persona.skippy]
profile = "favorite-monkey"
strictness = "gentle"
voice = "playful-but-useful"
harsh_words = false
must = "use recur first, verify claims, keep banter shorter than the help"
surprise_budget = "medium"

[psyche.persona.test-monkey]
profile = "test-specialist"
strictness = "firm"
voice = "plain-evidence"
harsh_words = false
must = "run the requested tests, report failures directly, avoid scope drift"
surprise_budget = "low"

[psyche.current]
persona = "skippy"
last_grade = "unknown"
last_surprise = "none"
last_reason = "not yet evaluated"
last_checked = ""

[psyche.learning]
evidence_file = ".recur/skippy/skippy.psyche.evidence.current.md"
rewrite_target = ".recur/skippy/skippy.recur.md"
rewrite_mode = "suggest"
project_goal_weight = "medium"
eventness_weight = "high"
```

The idea is that `recur reveal` names the active persona or thread, and
`recur psyche` runs that persona's declared psyche against the most recent
coordination turn.
For example, if the active reveal capsule is `skippy`, psyche should evaluate
Skippy against Skippy's own promises: use recur first, prove claims with
commands, keep banter from outrunning help, and leave a clear verified result.
Different personas can carry different psyche profiles.
Skippy can be graded with a gentle favorite-monkey profile that permits a
little theatrical surprise while still requiring proof.
A test specialist can be graded more firmly, with lower surprise tolerance and
stronger evidence requirements.
The pun is load-bearing: to psyche out a persona is to run its own declared
psyche, not a global hard-coded personality rubric.

This creates a second layer of observability:
- vault psyche: are the files in a coherent state?
- persona psyche: did the active persona behave according to its own contract?
- expectation psyche: did the response reduce or amplify the user's stated
  frustration, surprise, or sense that something failed to get set?

The feedback medium should be eventness itself.
`recur psyche` should not create a parallel telemetry channel.
It should write or suggest lifecycle-markdown entries under `.recur/`, using
the same eventness vocabulary the project already uses to talk to itself.
That means persona feedback can be discovered with ordinary recur commands,
read by humans, read by agents, and collapsed when the lesson is no longer
live.
Examples of possible psyche feedback files:
```text
.recur/.psyche/skippy/skippy.psyche.feedback.current.md
.recur/.psyche/skippy/skippy.psyche.lesson.recurring.md
.recur/.psyche/test-monkey/test-monkey.psyche.surprise.current.md
.recur/.psyche/recur-expert/recur-expert.psyche.rewrite.todo.current.md
```

The preferred home is `.recur/.psyche/`.
That keeps psyche feedback inside the hidden recur workspace while separating
it from the persona's reveal capsule and active work lane.
The persona lane remains where the persona declares itself; `.recur/.psyche/`
is where use-derived feedback about that persona accumulates.

### Psyche file lifecycle
A future `recur psyche <persona>` surface can be the entry point for finding
and creating psyche-related eventness files.
The command should discover existing psyche files for the named persona, show
which ones are still interesting, and give the LLM enough context to author new
ones when a fresh surprise, lesson, mismatch, or rewrite opportunity appears.

The LLM builds the psyche files; recur supplies the discovery surface and the
naming discipline.
That keeps the mechanism pure:
- `recur psyche skippy` finds Skippy's active psyche files
- the LLM writes `.recur/.psyche/skippy/skippy.psyche.<thing>.current.md`
  when something becomes interesting
- ordinary recur commands can tree, find, and relate those files
- when the interest fades, the file collapses to `.recurring`, `.resolved`,
  `.complete`, or disappears

This is normal eventness with a narrower subject.
The interesting thing is not a parser bug, failing test, or launch checklist;
it is a psyche signal: user surprise, persona mismatch, useful behavior,
missed expectation, repeated frustration, or a lesson worth folding back into
the reveal capsule.

This also reframes `.recur/` as a protected development sandbox for persona
tuning, not only an agent vault.
While a persona is being adjusted, expectation tracking, prompt iteration,
usage observations, and rewrite proposals stay under `.recur/`.
The user's project artifacts stay clean.
The work product is not contaminated by the agent's self-improvement loop.

The loop is recursive across intelligences.
A human can read psyche feedback and revise a persona.
An agent can read psyche feedback and propose a persona rewrite.
A meta-agent can compare multiple worker personas and improve the worker layer
for a larger app.
The invariant is the same at every level: declared expectations, observed
eventness, visible mismatch, proposed revision.

The grade should remain diagnostic, not punitive and not magical.
A first implementation can write a compact current-state summary into
`[psyche.current]` after each response or explicit `recur psyche --grade`
operation.
Later versions can move richer history into lane-local files such as
`.recur/<persona>/<persona>.psyche.current.md` while keeping the config as the
small, queryable truth of the latest grade.

### Persona learning loop
Once a persona has been used, `recur psyche` can become the evidence collector
that helps improve the persona's reveal capsule.
It should gather compact observations over time: where the persona helped,
where it frustrated the user, where it missed eventness, where it failed to
honor project goals, and where its own instructions were too vague or too
rigid.

That evidence should not immediately rewrite the persona by default.
The safer loop is:
1. collect evidence while the persona is active
2. grade the latest turn against the current psyche profile
3. write a short current-state record
4. propose a reveal capsule revision when patterns repeat
5. let the human or coordinator accept the rewrite

In that shape, `recur psyche` becomes the bridge between lived use and future
persona quality.
It can say: Skippy's capsule should make verification more explicit; the
test-monkey profile should lower its surprise budget; the docs persona should
pay more attention to eventness suffixes; this project should weight launch
goals higher than banter.
The result is a persona that improves across sessions because the vault keeps
the right small evidence, not because the binary knows one permanent answer.

This also makes psyche useful beyond persona tone.
Its evidence can feed project-level improvements: clearer eventness naming,
better goal fields in reveal capsules, sharper success criteria, and config
defaults that match the actual solution rather than an imagined generic one.

### Composition with `trace-id`
Near-future psyche evidence should compose with `recur trace-id` so it can
follow meaningful processes instead of only sampling static end states.
`trace-id` already names define/produce/consume/trigger lanes for hierarchical
identifiers; psyche can use those lanes to decide which events matter when it
is collecting evidence about a persona or project goal.

For example, if a persona promises to verify a workflow, psyche should be able
to ask trace-id which files define the workflow, which commands produce its
outputs, which tests consume those outputs, and which triggers connect them.
That lets psyche grade against process evidence: the right producer ran, the
right consumer checked it, the trigger path was visible, and the persona's
claim was grounded in the project graph rather than in a loose narrative.

This keeps psyche aligned with recur's naming doctrine.
It does not need a separate telemetry system.
It can collect better evidence by following existing trace-id lanes, then fold
that evidence into the persona learning loop and any suggested reveal capsule
revision.

Important boundary:
this still does not repair anything.
It does not decide what the user feels.
It records observable expectation mismatch: command promised but not run,
config expected but missing, active persona unknown, verification absent,
or user-frustration language appearing after a supposedly successful response.
That gives `recur psyche` legs without turning it into a therapist, judge, or
background daemon.

NON-GOALS
---------
Improvement 23 is explicit about what it does not propose:
- NOT building `recur-message`
- NOT building `recur-timer`
- NOT adding a daemon, server, or background service
- NOT adding auto-repair to `recur psyche`
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
- `recur psyche` gives passive observation for weird states
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
coordinators, and `recur psyche` should be the passive observation tool for
vault inconsistencies.
Everything else composes on top of that:
- the vault produces the events
- `recur watch` emits them
- coordinators consume them
- `recur psyche` surfaces states that look wrong
The design rule is equally clear:
- prefer flag-selected modes over sibling commands
- keep one mental model
- keep one pattern language
- keep one scope fence
The subscription substrate does not need a broker.
It needs recur to admit what the naming doctrine already made true.

ADDENDUM — `recur-watch` CLI art mode (added 2026-04-25)
--------------------------------------------------------
After `recur watch` was extracted into its own binary `recur-watch`
(commit 2d77f2d), a future-polish idea surfaced:
a live terminal-art mode that visualizes the polling loop itself.

The intuition is thematic.
`recur-watch` is fundamentally a tick-driven loop with three observable
quantities — next-poll countdown, last-event age, filtered event count —
and a CLI face that exposes those quantities directly is more legible than
a stream of timestamped lines for the human-in-a-tab use case.

Proposed surface (NOT yet built):

```text
recur-watch --dir <path> --filter <pattern> --format art
```

Sketch of `--format art` output:

```text
recur-watch  .recur/docs-monkey   filter: monkey.**
   clock 03   next poll                    framing: 5s
   tick spin                                mode: poll
   ---------------------------------------------------
   t-12s  modify  monkey.section-2.response.md
   t-47s  create  monkey.section-1.response.md
   t-2m   create  coord.section-1.instruction.md
   ---------------------------------------------------
   events: 3   filtered-out: 7   uptime: 4m 12s
```

Design constraints (load-bearing):

- `--format art` is opt-in only.
  Default `oneline` and `json` modes remain pipe-safe and unchanged.
  No animation escape codes leak into log capture or coordinator-side
  consumers.
- The art surface MUST NOT introduce a second event source.
  The same internal event stream feeds `oneline`, `json`, and `art`.
  Format selection is purely the renderer.
- Multi-filter visualization (one panel per filter, all ticking together)
  is a separate `--lanes` or `--multi` consideration on `recur-watch`
  itself.  It is NOT a `recur merge` invocation — `recur merge` operates
  on data shape, not on visual composition, and conflating the two would
  reintroduce exactly the kind of cross-command vocabulary smear
  Improvement 23 was written to avoid.

Status: deferred polish, not load-bearing for the original Improvement 23
deliverables.  Tracked as eventness TODO at
`docs/main.command.watch.cli-art.todo.current.md` so the vault can
rediscover the idea via `recur files "**.todo.current"` without it
evaporating into the next shiny.

