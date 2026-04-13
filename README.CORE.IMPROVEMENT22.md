RECUR IMPROVEMENT 22
Reveal Doctrine / Lane-Scoped Rehydration
=========================================
Date: April 13, 2026
Status: Proposal / future direction
Author: Proposed from recurring eventness, Skippy persona work, and multi-agent lane coordination thinking


INTENT
------
This document proposes a future improvement for a problem recur is now close to
but does not fully solve:

recur can discover lanes well, but cold-start rehydration is still too manual
when:

- multiple agents work in parallel
- a lane looks calm on the surface but has one real loose thread
- persona, agent role, agenda, and next pull-point need to be re-established
- a repo wants one coordinated reveal per run instead of many competing
  rediscovery threads

Improvement 22 proposes a reveal doctrine:

- `.recur/config.toml` defines how a lane should be re-entered
- `*.recur.md` is the lane-local gift or entry point
- one lane, run, or thread gets one primary pull-point by default
- agents can still branch out, but recur offers a simpler coordinated reveal
  first

The goal is not more decorative metadata.
The goal is to make the next session feel like the repo remembers how to start.


SUMMARY
-------
Current recur is already strong at:

- visible eventness in file names
- separator-aware lane discovery
- query-first rediscovery through `files`, `tree`, `find`, `trace-id`, and
  related commands
- config-driven separator inference through `.recur/config.toml`

Improvement 22 proposes a layer above that:

- a reveal doctrine in `.recur/config.toml`
- a lane-local `*.recur.md` file as the human/agent ignition capsule
- an ordered re-entry sequence:
  persona -> agent -> agenda -> goals -> schedule -> pull -> verify -> ready
- optional nested lane roots, run roots, or thread roots with their own
  `.recur/config.toml`
- clearer multi-agent merge posture and handoff discipline

Short version:

recur should not just find files.
It should help decide how to pull the right thread first.


THE PROBLEM
-----------
When a repo matures, a lot of eventness collapses.

That is good.

But it creates a new problem:

- the surface can look clean and boring
- the real open edge can be small and hidden
- the next useful command can be obvious only to the previous session
- multiple agents can start from the same broad search and step on each other

Today a strong human or AI can recover by reading docs, querying trees, and
manually reconstructing intent.

That works, but it is not yet a first-class doctrine.

The ideal resume experience is:

1. identify the lane
2. reveal the one loose thread that still matters
3. remember who the current agent is in that lane
4. run the next useful command
5. verify the truth
6. feel oriented instead of scattered


THE CORE IDEA
-------------
Add a future reveal doctrine to `.recur/config.toml`.

That doctrine would define:

- whether the default resume mode is single-thread or multi-thread
- what suffix marks the lane-local reveal gift
- what ordered fields should be shown or consumed first
- whether persona can be skipped when already known
- how many threads should be pulled by default in one run
- what merge and handoff posture should be assumed

Then each meaningful lane can have one small `*.recur.md` file.

That file is not a replacement for:

- `*.current.md`
- `*.recurring.md`
- tests
- source code

It is the ignition capsule.

It says:

- who I am here
- why this lane matters
- what not to reopen
- what command starts the real unraveling


THE LOOSE THREAD MODEL
----------------------
Improvement 22 assumes that good eventness is not maximum noise.

It is better understood like this:

- the environment is mostly calm
- most visible eventness has already collapsed
- one small line still carries the voltage
- that line is the loose thread

The reveal system should make that thread easy to find without forcing the repo
back into chaos.

So the doctrine should prefer:

- one primary entry per lane
- one clear first command
- one explicit verification step
- one explicit "do not disturb" warning when the fake drama is known

This is a better resume model than many simultaneous pull-points by default.


WHAT `*.RECUR.MD` WOULD MEAN
----------------------------
`*.recur.md` would be the lane-local gift.

It is not another state marker like:

- `current`
- `complete`
- `future-plan`
- `recurring`

Instead it means:

"Start the conversation here."

The file should be short, deliberate, and command-first.

Example:

```text
main.command.trace-id.recur.md
main.demo.skippy.trace-id.recur.md
main.improvement.delivery-loop.recur.md
```

Each one should expose the ordered reveal, not act like a diary dump.


PROPOSED REVEAL ORDER
---------------------
The reveal order should be explicit and stable.

Recommended default order:

1. `persona`
2. `agent`
3. `agenda`
4. `goals.now`
5. `schedule.next`
6. `pull.first`
7. `pull.then`
8. `verify`
9. `tool.escape`
10. `do.not.disturb`
11. `ready.state`

Meaning:

- `persona`: who am I in this lane, if I need reminding?
- `agent`: what role am I playing here?
- `agenda`: why does this lane exist right now?
- `goals.now`: what matters in this session?
- `schedule.next`: what order should work happen in?
- `pull.first`: what exact command begins the real work?
- `pull.then`: what query narrows the thread further?
- `verify`: what proves I am not hallucinating?
- `tool.escape`: what wider map do I use if I get lost?
- `do.not.disturb`: what trap should I avoid reopening?
- `ready.state`: what tells me I am oriented now?

If persona is already known, the doctrine should allow skipping it.


EXAMPLE `*.RECUR.MD` FILE
-------------------------
Example shape:

```text
# main.command.trace-id.recur

recur.gift = core trace-id is done; saved-run policy is the only real open edge
persona = recur expert in the trace-id lane
agent = resume and verify, not rediscover from scratch
agenda = saved-run policy and docs alignment
goals.now = decide latest-only vs history
schedule.next = docs -> focused tests -> code only if contract changed

pull.first = recur files "main.command.trace-id.**" -d docs/
pull.then = recur find "saved-run" --scope "main.command.trace-id.**" -d docs/ -i
verify = julia julia-tests/main.command.trace-id.test.jl
tool.escape = recur tree "main" --sep . --sep _ --show-sep

do.not.disturb = do not reopen core heuristics without a failing test
ready.state = I know who I am, where I am, and what to pull next
```

This is the pleasant surprise:

the next session does not start from zero.
It starts from a deliberate reveal.


PROPOSED CONFIG SHAPE
---------------------
Possible future config shape:

```toml
[reveal]
mode = "single-thread"
entry_suffix = ".recur.md"
trust = "config-first"
max_threads = 1
skip_persona_if_known = true

[reveal.order]
steps = [
  "persona",
  "agent",
  "agenda",
  "goals.now",
  "schedule.next",
  "pull.first",
  "pull.then",
  "verify",
  "tool.escape",
  "do.not.disturb",
  "ready.state",
]

[reveal.merge]
default_role = "additive"
default_handoff = "verify passes and lane truth updated"
default_touch = "declared-by-lane"

[reveal.rank]
prefer_current = true
prefer_trigger_event = true
prefer_recurring = true
prefer_complete = false
```

This does not need to be implemented now.

The important idea is:

- `.recur/config.toml` becomes the one source of trust for how reveal works
- `*.recur.md` files hold lane-local content inside that doctrine


MULTIPLE ROOTS / MULTIPLE LANE FOLDERS
--------------------------------------
Improvement 22 also proposes a future bold step:

allow multiple lane roots, run roots, or thread roots to have their own
`.recur/config.toml`.

That would allow:

- one repo root doctrine
- one lane-local doctrine
- one run-local doctrine
- one thread-local override when needed

Example future layout:

```text
.recur/config.toml

docs/main.command.trace-id.recur.md
docs/main.demo.skippy.trace-id.recur.md

runs/
  trace-id/
    .recur/config.toml
    docs/
    src/

  skippy-demo/
    .recur/config.toml
    docs/
    demos/

lanes/
  release-a.0.2.8/
    .recur/config.toml
```

Then `recur init` could be used intentionally at multiple roots:

```powershell
recur init
recur init -d runs/trace-id
recur init -d runs/skippy-demo
recur init -d lanes/release-a.0.2.8
```

This would let each local root establish:

- separator policy
- reveal doctrine
- trait overrides
- run-local checkpoints
- lane-local status conventions

The current nearest-config-root behavior already hints at this direction.
Improvement 22 proposes formalizing it.


WHY MULTIPLE ROOTS MATTER
-------------------------
This becomes especially useful when:

- multiple agents work on different subproblems
- each agent needs a bounded local truth
- each lane wants a local restart ritual
- merge coordination should happen after local verification, not before

Without that, every agent starts from the same global tree and has to mentally
carve out a lane.

With lane-local or run-local roots:

- the lane becomes physically and semantically bounded
- the local `.recur/config.toml` declares the doctrine
- the local `*.recur.md` file starts the conversation
- the root repo still remains the broader coordinating truth


MULTI-AGENT COORDINATION
------------------------
Improvement 22 is a natural fit for multiple agents working in parallel.

Each agent can get:

- one lane root or run root
- one reveal gift
- one explicit agenda
- one declared verification step
- one declared merge posture

Possible lane-local fields:

```text
agent.role = docs-and-tests
agent.scope = trace-id.run
merge.role = additive
merge.touch = docs + tests
merge.avoid = core heuristics
handoff.when = verify passes and lane truth is updated
```

This creates a cleaner coordination model:

1. each agent begins in one lane
2. each lane exposes one reveal
3. each agent verifies locally
4. merge happens after declared handoff conditions are met

That is much safer than many agents pulling many threads at once.


THE SIMPLER DEFAULT
------------------
Improvement 22 should not force a complex orchestration system on every repo.

The default should stay simple:

- one primary reveal per lane
- one primary thread per run
- persona may be skipped if already known
- multi-thread exploration is allowed, but not the default

In other words:

users may still bungle themselves into ten threads if they want to.
recur should simply offer a cleaner and more coordinated alternative first.


NON-GOALS
---------
Improvement 22 is not trying to:

1. replace existing eventness suffixes
2. turn every lane into a heavy state machine
3. force persona recitation on every resume
4. make `*.recur.md` the canonical source of implementation truth
5. require nested lane roots for ordinary repos
6. prevent skilled users from doing broad exploratory work when that is the
   right move

This is a reveal and coordination layer, not a replacement for source, tests,
or current docs truth.


WHY THIS MATTERS
----------------
If done well, Improvement 22 would improve:

- next-session delight
- cold-start orientation
- multi-agent coordination
- merge quality
- lane-local identity and role clarity
- rediscovery quality for both humans and AI

It would make the repo feel less like:

- a pile of previously useful notes

and more like:

- a calm environment with one deliberate loose thread
- a hidden door that opens in the right order
- a system that remembers how to resume itself


RELATIONSHIP TO EXISTING IMPROVEMENTS
-------------------------------------
This fits naturally with:

- Improvement 14: mirror eventness
- Improvement 19: lane productization for human + AI teams
- Improvement 21: directory projection / namespace mapping

Rough relationship:

- Improvement 14 asks where canonical and mirrored eventness should live
- Improvement 19 asks how lanes should scale across larger teams
- Improvement 21 asks how physical structure should project into logical names
- Improvement 22 asks how a lane should reveal itself when work resumes

So Improvement 22 is not replacing those directions.
It is the re-entry and orchestration layer above them.


POSSIBLE STAGING
----------------
Stage A - Doctrine only

- define the reveal model
- define `*.recur.md`
- define default field order

Stage B - Config recognition

- parse `[reveal]` sections in `.recur/config.toml`
- tolerate absent reveal sections cleanly

Stage C - Lane-local reveal helpers

- add helper commands or scripts that discover `*.recur.md`
- support "show me the reveal for this lane"

Stage D - Multi-root formalization

- support nested lane roots and run roots intentionally
- define inheritance/override behavior between configs

Stage E - Merge/handoff coordination

- make lane-local merge posture and handoff conditions queryable
- support cleaner multi-agent orchestration

Because this is future work, the design should be allowed to be bold now.
The repo will have progressed before this needs to harden into a final surface.


FINAL THESIS
------------
recur already helps discover the tree.

Improvement 22 asks it to help reveal the right thread to pull next.

Not every session should begin with broad search.
Some sessions should begin with a gift.

That gift should be:

- lane-local
- command-first
- role-aware
- calm on the surface
- precise underneath

And when multiple agents are involved, that same reveal doctrine can help each
agent stay in-lane, verify honestly, and merge more coherently later.
