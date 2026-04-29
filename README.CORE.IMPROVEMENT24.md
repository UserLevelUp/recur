RECUR IMPROVEMENT 24
Sealed-Cycle Spin Dispatch (The Washing Machine Model)
======================================================
Date: April 21, 2026
Status: Proposal / future direction
Author: Proposed from 2026-04-21 DLG3 README sweep close-out
  (Skippy coordinator / docs-monkey executor — pattern observed live)

INTENT
------
Improvement 23 gave the vault a subscription primitive.
This improvement is about what sits above that primitive.

The current shape is coordinator-as-thinker:
a Claude-class coordinator writes one instruction, fires one monkey, reads one
response, writes commentary, fires the next monkey, reads the next response,
and so on — all inside a single conversation.
That conversation accumulates every instruction, every response, every
editorial aside, every "mild editorial note," every three-option ladder
handed to the human.
Context grows roughly linearly in the number of rounds.
Token cost grows with it.

Worse, the coordinator's per-round thinking has no load-bearing output.
The only things that leave the conversation are:
- `coord.<id>.instruction.md` files (consumed by executors)
- a final commit (consumed by git)
Everything else — the mid-round speculation, the phrasing alternatives, the
"worth noting" commentary, the anchor rankings — is theater.

The DLG3 README sweep (four sections + one retry) spent five coordinator
turns accomplishing what the same coordinator could have specified in one.
The pattern worked correctly.
It was just expensive.
This document names the architecture that fixes the cost without touching the
correctness.

SUMMARY
-------
Improvement 24 proposes `recur spin`:
a sealed-cycle dispatcher that replaces the mid-cycle coordinator.

The governing metaphor is explicit.
**It is a washing machine.**

1. Load the machine.
   Coordinator writes a spin manifest up front.
2. Close the door.
   `recur spin` starts.
3. Cycle runs.
   Robots execute rounds, responses accumulate, cross-contamination is fenced.
4. Door unlocks.
   Evidence pack is emitted.
5. Coordinator reads the pack once.
   Aligned → done.
   Drift → adjust manifest, spin again.

Between door-lock and door-unlock the coordinator cannot interfere.
This is a feature, not a limitation.
The inability to intervene is what bounds context:
no mid-cycle conversation, no per-round editorial, no option ladders.

Three inputs go into the machine before the door locks:
- **Detergent** — the plan.  Per-round instructions.
- **Softener** — doctrine and tone.  Applies across all rounds.
- **Color-sort** — scope fences.  Per-robot invariants, enforced at dispatch.

Post-cycle the coordinator performs one check:
did the reds bleed into the whites?
If every robot stayed in its lane and output matches intent, the pattern
worked.
If not, adjust the manifest and re-spin.

THE PROBLEM
-----------
The coordinator-as-thinker pattern has three failure modes that only become
visible under volume.

First, context growth is round-linear.
Each round adds an instruction file (read by the coordinator on draft), a
response file (read by the coordinator on review), and an assessment
(written by the coordinator).
By round five the conversation carries the full prose of rounds one through
four, and the coordinator is re-reading it by proximity.
Working context becomes a chronicle of past rounds rather than a focused
description of what is left to do.

Second, per-round coordinator decisions have no load-bearing output.
Draft-review-draft-review has a shape but no product.
The artifacts that matter are the instruction files and the final commit.
Everything the coordinator says between them is discarded:
the executor ignores it, the commit does not need it, the next round starts
fresh.
Those tokens buy nothing.

Third, the coordinator over-specifies out of anxiety.
Drafting in a loop, the coordinator pre-answers questions the specialist
would have answered correctly on its own.
The README sweep produced 100-line coord instructions with anchor-ranked
placement candidates, ranked phrasing suggestions, judgment-call prompts,
and doctrine restatements repeated every round.
The specialist knew the doctrine.
The specialist picked placement correctly in every case.
The ninety extra lines were coordinator cosplay.

None of these failures affect correctness.
They all affect cost.

THE CORE IDEA
-------------
Coordination has three load-bearing moments, not one continuous one.

**Moment 1 — load.**
Coordinator decides what needs to happen, what doctrine applies, and what
the scope fences are.
One concentrated thinking burst.
Output: a spin manifest.

**Moment 2 — execute.**
Specialists run their lanes.
No coordinator thought required — the manifest has already specified intent,
doctrine, and scope.
If the specialists know their area, they execute correctly.

**Moment 3 — judge.**
Coordinator reads the evidence pack once.
Binary check:
did the spin produce output aligned with intent, within scope fences,
sounding like doctrine wants?
If yes, the pattern worked — move on.
If not, name the drift and load a corrective cycle.

The mistake is letting moment 2 accumulate coordinator context.
The coordinator has nothing to contribute during execution — the manifest is
complete and the specialists have domain knowledge.
Any context the coordinator burns during execution is paying to watch a
cycle it cannot meaningfully influence.

The washing-machine metaphor makes the constraint concrete.
Once the door is locked the coordinator cannot:
- add a sock to the load
- change the water temperature
- re-sort the colors
- stop the cycle to inspect progress
- second-guess a scrub direction
Those decisions were made at load time.
The machine runs.
When it finishes, there is laundry.
The coordinator inspects the laundry.

This mechanical seal is what prevents the coordinator from meandering.

DELIVERABLES
------------
Improvement 24 proposes one deliverable.

### Deliverable: `recur spin`

Proposed surface:
```text
recur spin --plan <manifest> [--dir <vault>] [--format <text|json>]
```

A spin manifest is a single file that declares:
- **Intent** — the goal of the cycle in one paragraph.
  Alignment is judged against this.
- **Rounds** — ordered list of `(id, coord-file-path)` tuples.
  Each `coord-file-path` points to a pre-written instruction file in a
  staging directory.
- **Doctrine** — tone, style, jargon bans, shape requirements.
  Applied to all rounds.
- **Scope fences** — per-robot-class invariants.
  E.g. "docs-monkey cannot edit `src/`, cannot run `cargo`, cannot commit."
- **Terminator condition** — typically "all expected responses present,"
  optionally extended with custom predicates.

At launch, `recur spin`:
1. Validates the manifest (all staged coord files exist, doctrine keys
   resolve, scope fences are enforceable).
2. Locks the live vault for the lane's topic prefix — no external coord
   writes are accepted until cycle completion.
3. For each round in order:
   a. Copies `staged/coord.<id>.instruction.md` into the live vault.
   b. Waits for `monkey.<id>.response.md` to land
      (via `recur watch` — composition with Improvement 23).
   c. Validates the response against scope fences.
      A fence violation kills the cycle immediately with a named drift.
4. When all rounds complete, writes `coord.session.complete.md`.
5. Emits an evidence pack:
   all coord files, all responses, a drift report, and a diff of files
   touched by the cycle.
6. Releases the vault lock.

The coordinator's next action is to read the evidence pack once.
There is no step for the coordinator to take mid-cycle.

### Manifest shape (illustrative)

```text
spin: readme-sweep
intent: |
  Document `recur watch` and `recur psyche` in README.md across four
  surgical inserts, matching existing Commands subsection shape, without
  introducing pub/sub jargon or documenting a --fix flag that does not
  exist.
doctrine:
  banned-terms: [pub/sub, broker, event bus, message queue, --fix, --repair]
  required-tone: match neighbor Commands subsections
  timestamp-format: UTC with Z suffix
scope-fences:
  docs-monkey:
    deny-edit: [src/**, Cargo.toml, Cargo.lock, tests/**]
    deny-run:  [cargo, julia]
    allow-write: [README.md, .recur/docs-monkey/monkey.**]
rounds:
  - { id: section-1, coord: staged/coord.section-1.instruction.md }
  - { id: section-2, coord: staged/coord.section-2.instruction.md }
  - { id: section-3, coord: staged/coord.section-3.instruction.md }
  - { id: section-4, coord: staged/coord.section-4.instruction.md }
terminator: all-rounds-complete
```

### Evidence pack shape

After cycle completion `recur spin` writes one summary file:

```text
spin: readme-sweep
status: complete
rounds-processed: 4
verdicts:
  section-1: SHIPPED
  section-2: SHIPPED
  section-3: SHIPPED
  section-4: SHIPPED
files-touched: [README.md]
diff-summary: +23/-0
fence-violations: none
cycle-wall-time: 47s
```

If every verdict is SHIPPED, `fence-violations: none`, and the diff matches
intent, the spin aligned.
The coordinator commits and moves on.
No per-round prose is read.
No editorial is written.

LOAD TAXONOMY
-------------
The three inputs named in the summary have distinct roles and lifecycles.

**Detergent — the plan.**
Per-round instruction files, pre-written during the load moment.
Minimal: target, scope, ground-truth pointer.
Not 100 lines.
Not anchor-ranked.
Not phrasing-suggested.
Specialists know their area.
The instruction names the edit and trusts the specialist to execute it.

**Softener — doctrine and tone.**
Cross-round invariants: banned terminology, required shape, timestamp
conventions.
Declared once in the manifest, inherited by every round.
The specialist does not re-read doctrine per round — the dispatcher
validates outputs against it.

**Color-sort — scope fences.**
Per-robot-class invariants: which files each robot may read, write, or
execute.
Enforced at dispatch time.
If a response shows edits outside the allowed set, `recur spin` treats it as
a fence violation and kills the cycle.
This is what prevents reds from bleeding into whites.

These three inputs are orthogonal.
Changing the plan does not require re-stating doctrine.
Changing doctrine does not require rewriting the plan.
Tightening a scope fence does not touch either.

ALIGNMENT CHECK
---------------
The coordinator's post-cycle deliverable is a binary judgment.

Aligned:
- every round completed with the expected verdict
- no fence violations
- diff matches intent (coordinator reads the diff, not the round-by-round
  commentary)

Drifted:
- a round produced BLOCKED or ABORTED
- a fence violation fired
- the diff departs from intent in a way the coordinator cares about

On alignment: commit the diff.
The pattern worked.
Move on.

On drift: identify the specific drift, adjust the manifest (tighten a fence,
clarify an instruction, split a round), and run `recur spin` again.
Do not adjust in conversation.
Do not explain the drift in prose.
Adjust the manifest — the manifest is the durable artifact.

This discipline is what prevents context creep.
Alignment judgments are cheap.
Drift-correction-by-manifest-edit is cheap.
Drift-correction-by-conversation is expensive and recursively drifts further
every time it happens.

NON-GOALS
---------
Improvement 24 is explicit about what it does not propose:
- NOT building a daemon or long-running service.
- NOT allowing mid-cycle intervention.
  The door lock is load-bearing.
- NOT auto-retrying failed rounds.
  A BLOCKED round surfaces in the evidence pack; the coordinator decides.
- NOT replacing coordinator judgment with heuristics.
  The alignment check is a human (or Claude) reading the evidence pack.
  The machine does not decide "good enough."
- NOT inventing a new topic language.
  `recur spin` uses the same filename patterns and vault doctrine `recur
  watch` already uses.
- NOT replacing `recur watch`.
  Spin composes on top of watch; watch remains the underlying subscription
  primitive.
- NOT merging coordinator and executor roles.
  The roles are distinct and the seal between them is what makes the
  architecture work.

These non-goals matter because each of them, if added, re-inflates the
machinery back into the thing Improvement 24 is deleting.

FAILURE-MODE TAXONOMY
---------------------
Expected failures during a spin:
- a round produces BLOCKED — the executor names ambiguity; the manifest
  needs clarification
- a scope fence fires — the executor tried to edit outside its lane; either
  the instruction misdirected or the fence was too tight
- a round times out — the runtime hosting the executor died before
  responding

All three surface in the evidence pack.
None require the coordinator to intervene mid-cycle.
All get addressed by a manifest edit and a re-spin.

Unexpected failures:
- `recur spin` itself crashes mid-cycle.
  The vault is in an indeterminate state.
  Resolution: restart with the same manifest; staged instructions are
  idempotent as long as prior `monkey.<id>.response.md` files are preserved.
- The executor runtime hangs silently.
  Resolution: per-round timeout produces a timeout verdict and the cycle
  continues with the remaining rounds.
- A fence violation is detected after the executor already wrote to a file
  outside the allowed set.
  Resolution: the cycle kills immediately and emits a violation-evidence
  pack.
  The coordinator decides whether to revert the bad edit or accept it.

None of these failure modes involve cross-executor coordination — the
color-sort fence prevents overlap.
They are about individual executors failing, which is a bounded problem.

COMPOSITION WITH IMPROVEMENT 23
-------------------------------
Improvement 23 established `recur watch` as the subscription primitive over
vault writes.
Improvement 24 uses that primitive as the dispatcher's inner loop.

Specifically:
- `recur spin` does not reimplement filesystem watching.
- For each live round, `recur spin` runs an internal
  `recur watch --filter "monkey.<id>.response.md"` and blocks until it fires.
- For cycle termination, `recur spin` watches for
  `coord.session.complete.md`.

This composition means Improvement 24 is thin.
The load-bearing filesystem subscription work was done in 23.
Improvement 24 is primarily:
- the manifest schema
- the dispatcher state machine
- the scope-fence validator
- the evidence-pack format

Two observation commands already exist to debug in-flight cycles from
outside the seal:
- `recur watch --filter "<spin-prefix>.**"` — human in a tab can observe
  (but cannot inject)
- `recur psyche` — after a crash, surfaces indeterminate vault state

Both are read-only.
Neither breaks the door lock.

COMPOSITION WITH PARALLEL LANES
-------------------------------
A single spin may dispatch more than one executor.
The number is not a property the coordinator needs to track.

A manifest may declare:
- one lane with four sequential rounds
- four lanes with one round each, executed in parallel
- N lanes with M rounds each, mixed sequential and parallel

From the coordinator's perspective these are the same shape:
load → seal → evidence pack.
Whether `recur spin` spawned two executors or fifty-two is an implementation
detail.
The color-sort fence prevents executors from overlapping regardless of
count.

This matters because volume pressure will push lane counts up.
A two-lane pattern where the coordinator can eyeball each round does not
generalize to a twenty-lane pattern.
Sealed spin cycles generalize.

CLOSING
-------
Improvement 24 says something narrow and consequential:
coordination should be a two-touchpoint interaction — load and judge — not a
continuous conversation.

The mechanical shape that enforces this is the sealed spin cycle:
- inputs in (plan, doctrine, fences)
- door locks
- cycle runs autonomously
- evidence pack out
- alignment judged once

Everything that used to happen mid-cycle either moves into the load moment
(pre-written and staged) or disappears entirely (because specialists handle
it).

The design rule is clear:
- prefer manifest edits over conversational adjustment
- prefer specialist autonomy over coordinator micromanagement
- prefer binary alignment judgments over incremental commentary
- prefer sealed execution over mid-cycle chaperoning

The coordinator's value is concentrated in two moments, not diffused across
many.
The machine does the rest.
