RECUR IMPROVEMENT 27
Eventness Warp and Project-Control Commands
===========================================
Date: May 22, 2026
Status: Recur v0.2.8 implementation; Warp Methodology v0.3.0;
pure compositional queries and confirmed Slice-layer writer implemented
Author: Captured from a 2026-05-22 design discussion

INTENT
------
Make the eventness "warp" idea usable as a generic recur command concept:
given a lane and its surrounding eventness, estimate whether the work is at an
optimum, sub-optimum, or blocked state; explain the evidence; predict what
should collapse; and suggest the next scoped management action.

METHODOLOGY ADDENDUM
--------------------
`README.CORE.IMPROVEMENT27.Appendum.md` expands Warp as an explicit,
versioned coordination mental model for participating intelligences. It keeps
observed evidence, local viewpoints, shared understanding, disagreement,
Eventness, bounded Slices, receipts, authority, and Warp evolution distinct.

Improvement 27 owns Warp methodology and the proposed `recur warp`
project-control surface. Improvement 30 is separate: Recur Lang maps exact
inputs, functions or methods, outputs, and their relationships so contracts,
dependencies, branches, joins, waits, retries, and circular references can be
inspected before or during development. Recur Lang may consume a bounded Warp
Slice contract; it does not own or redefine Warp.

This proposal keeps recur generic. It does not encode any private project,
domain, persona, or product rule. It treats eventness as plain files with
hierarchy, suffix states, trace-id roles, optional config weights, and project
management pressure signals.

SUMMARY
-------
Improvement 27 proposes a read-first command family:

- `recur warp status <lane>` - score a lane's current state.
- `recur warp next <lane>` - rank the next management actions.
- `recur warp collapse-plan <lane>` - predict which eventness should collapse
  and which should remain interesting.
- `recur warp explain <lane>` - show the evidence behind the verdict.
- `recur warp config` - show the configured weights, state mappings, and hard
  constraints.
- `recur warp map <warp>` - show the declared final Slice coverage map.
- `recur warp merge <warp>` - purely compose accepted Slice layers
  over that map and report coverage, gaps, blockers, and conflicts.

The command is a coordination layer over existing recur primitives:

- `recur reveal` restores lane-local context.
- `recur files` enumerates the scope.
- `recur tree` shows structure.
- `recur related` finds sibling pressure.
- `recur trace-id` classifies define / consume / produce / trigger evidence.
- `recur watch` and companion watcher tools can provide freshness or external
  event signals when configured.

The first implementation should be read-only and JSON-capable. It may propose
new lanes or collapse actions, but it must not delete, rename, approve, stage,
or write project artifacts without an explicit future write-side command and
operator confirmation.

THE PROBLEM
-----------
Eventness makes work visible, but visibility is not the same thing as project
control.

A lane can have many `.current`, `.complete`, `.strange`, `.solved`, `.ready`,
or `.acknowledged` files. Humans and coordinator agents can often infer whether
the lane is healthy:

- enough artifacts exist;
- tests or checks passed;
- strange cases were resolved;
- current work is stale;
- risk is unowned;
- a worker response was consumed;
- the next action is visible;
- a contradiction is still interesting.

That inference currently lives in a person's head or a persona's prompt. Recur
can make it inspectable by turning those signals into a small state-vector and
project-management verdict.

CORE IDEA
---------
Treat a lane as a state vector plus a project-control vector.

```text
E(t)  = {lane, current_state, interest, uncertainty, friction, completion, risk, worker_signal}
P(t)  = {scope, schedule, effort, quality, risk, stakeholders, communication, dependencies}
E*    = {tested, documented, scoped, collapsed_or_interesting, next_action_visible}

dE/dt = +artifact +verification +worker_signal +clear_trace
        -uncertainty -friction -stale_interest -unowned_risk

J(E, P) = w1*scope_drift + w2*schedule_lag + w3*effort_waste
        + w4*defect_risk + w5*unowned_risk + w6*communication_debt
        + w7*dependency_unknowns + w8*stale_interest
        - w9*delivered_value - w10*learning_value
```

The command does not need continuous mathematics to be useful. It needs a
practical discrete approximation:

- derive signals from file names, timestamps, text markers, trace-id roles, and
  optional config;
- compute a verdict;
- show the largest residual pressures;
- propose the next action that most reduces the residual objective.

The math is a control metaphor with an auditable output. Every score must point
back to concrete files or configured rules.

COMPOSITIONAL WARP BUBBLES
--------------------------
A Warp declares a final Slice coverage map. Accepted completion layers compose
over that map without imposing an arbitrary completion order, and the derived
projection self-reports covered, pending, blocked, stale, conflicting,
complete, or exploded state. Core `recur warp` remains pure; `recur-warp`
owns confirmation-gated completion writes.

The bubble is recursively ringed: the outer ring owns coordinator/orchestrator
convergence, while inner rings identify specialized directory domains with
their own nearest Recur config, reveal capsule, Eventness, subscriptions, and
local Warp. Inner-domain completion remains distinct from outer-ring
integration acceptance.

The full model, merge laws, schemas, self-reporting behavior, command boundary,
and observable explosion/evolution semantics live in
`README.CORE.IMPROVEMENT27.Appendum.md`. The executable command contract lives
in `docs/main.command.warp.readme.md`.

CONCEPT TRANSITION INTUITION
----------------------------
The warp metaphor can also be read as a concept transition: the workspace before
a concept exists has one eventness geometry; the workspace after the concept is
named has another.

```text
E_before = eventness space before concept C is named
C        = concept, decision, hypothesis, framework, or new control lens
E_after  = eventness space after C is applied
warp(C): E_before -> E_after
delta_C  = E_after - E_before
```

The practical question is not whether the metaphor is elegant. The practical
question is whether naming `C` changes the visible action surface:

- some branches collapse because the concept explains them;
- some branches become interesting because the concept exposes a boundary;
- some risks become owned because the concept supplies a management dimension;
- some next actions become obvious because the coordinate system changed.

`recur warp` should make that transition inspectable. It should compare the
pre-concept and post-concept eventness residue when enough evidence exists, or
approximate the transition by asking what the concept would newly define,
consume, produce, or trigger.

FUTURE-STATE CONVERGENCE
------------------------
The stronger form of warp is not only "what is wrong with this lane?" It is:
"what future eventness state are we trying to converge toward, and which
intermediate semi-states make that convergence real?"

This matters for complex engineered supersystems where many primitive pieces
must agree before the final build can be trusted:

- schematics must align expected voltages, tolerances, connectors, and test
  points;
- robotics subsystems must align capabilities, payload limits, sensors,
  actuators, control loops, safety envelopes, and interfaces;
- production runs must align specification, inspection evidence, revisions,
  deviations, approvals, and lot-level state;
- software, electrical, mechanical, process, and human-approval lanes must
  converge into one higher-order build state.

In that reading, `recur warp` imagines an intended future eventness state, or a
family of acceptable future states, then scores the current primitive setup
against that target. It should expose the semi-states between now and the
"ultra state": the completed supersystem where components, constraints,
capabilities, verification evidence, and approvals line up.

The command should still remain evidence-first. It should not invent domain
specifications, certify safety, or replace engineering review. It should show
where local evidence agrees with the intended future state, where it diverges,
and which subsystem boundary or missing proof prevents convergence.

TEMPORAL MEMBRANE PROJECTION
----------------------------
Warp also has a time-frame reading. A semi-autonomic or semi-automatic system
does not only have one current state; it spins through operational time. A day,
month, year, or decade can be treated as a frame over the same eventness
membrane.

In recur terms, the membrane is the scoped eventness surface:

```text
prefix.base.suffix.*
```

The prefix/base/suffix scope names the medium being observed. The frame names
the horizon being projected. The output should show what the membrane appears
likely to become from an eventness standpoint:

- what states are already observed;
- what states are plausible next semi-states;
- what states are blocked by missing evidence, approvals, tests, or specs;
- what residuals grow or shrink across day, month, year, or decade frames;
- what new prefix/base/suffix branches may need to exist for the future state
  to become real.

This is prediction in the bounded engineering sense: evidence-backed projection
from current files, trace roles, suffix states, timestamps, config, and known
constraints. It should carry uncertainty and evidence links. It should not
pretend to forecast beyond the material it can inspect.

EVENTNESS EPICS, MILESTONES, AND LONG-HORIZON EPOCHS
----------------------------------------------------
The practical planning unit is usually an eventness epic, step, or milestone.
An expert operator with deep domain experience can name the goals a system
should pass through, then let recur keep those states inspectable.

An epic is a framed target over an eventness membrane:

```text
epic = horizon + target_state + complete_set + pending_set + research_set
milestone = target_state + evidence_gates + state_buckets
```

An epoch can still exist in long-term warping, but it should mean a larger
time horizon that groups many epics or milestones. Near-term warp planning
should talk about epics, steps, and milestones rather than epochs.

The point is not to make a generic AI planner. The point is to let a real
coach, systems engineer, robotics lead, production engineer, or research lead
encode what "good progress" looks like in eventness terms:

- completed states: artifacts, tests, specs, approvals, or subsystems that
  should exist by this epic or milestone;
- pending states: known work that is not done yet but has a place in the plan;
- research states: unresolved unknowns, prototypes, experiments, trade studies,
  or capability questions;
- reusable patterns: common eventness structures from repeatable technology
  stacks that can make future work more efficient;
- invention patterns: shared milestones that help new ideas and prototypes
  inherit successful paths from older systems without pretending they are the
  same system.

That makes warp cross-domain. A synthetic robotics build, benign schematic,
software release, production run, research prototype, or complex aerospace
program can all expose the same kind of eventness: what is complete, what is
pending, what is research, what is blocked, and what evidence supports that
classification.

The command should keep expert intent and local evidence separate. Human or
domain tooling names the epics and milestones; `recur warp` audits the local
eventness against them. For safety-critical, regulated, hazardous, or
proprietary systems, the output is planning evidence only, not certification or
engineering authorization.

VERDICTS
--------
`recur warp status <lane>` should produce one of three verdicts.

### optimum

The lane is good enough for its current purpose:

- scope is named and bounded;
- artifacts exist;
- quality gates are green or explicitly accepted;
- risk is mitigated, accepted, or assigned;
- stakeholders have current actionable status;
- remaining residue is classified as collapse-known or still-interesting;
- the next action is visible, or no action is required.

### sub_optimum

The lane moved, but one or more residual gradients remain:

- scope drift;
- stale `.current` state;
- unresolved `.strange` state;
- missing owner;
- missing test or verification;
- hidden dependency;
- unowned risk;
- communication debt;
- repeated facts that should collapse;
- interesting contradiction not yet named as a lane.

### blocked

The best next action requires an external event, operator approval, destructive
action, private-sensitive decision, or product-direction judgment.

COMMAND SURFACE
---------------

### `recur warp status`

```powershell
recur warp status <lane> [-d <root>] [--scope <glob>] [--json]
```

Example text output:

```text
Lane: main.improvement.27
Verdict: sub_optimum
Objective: 3.8

Positive signals:
- proposal exists
- docs bridge exists
- trace-id roles present

Residual pressures:
- no test fixture yet
- command surface not implemented
- JSON schema not specified

Next action:
- write fixture criteria for warp status JSON output
```

JSON output should include:

```json
{
  "lane": "main.improvement.27",
  "verdict": "sub_optimum",
  "objective": 3.8,
  "signals": [
    { "name": "proposal_exists", "weight": -1.0, "evidence": ["README.CORE.IMPROVEMENT27.md"] }
  ],
  "residuals": [
    { "name": "missing_test_fixture", "weight": 1.0, "evidence": [] }
  ],
  "next_actions": [
    { "kind": "write_test", "lane": "main.improvement.27.test.criteria", "reason": "largest residual" }
  ]
}
```

### `recur warp next`

```powershell
recur warp next <lane> [-d <root>] [--json] [--limit <n>]
```

Ranks next management actions. Candidate action kinds:

- `verify` - run or record a gate;
- `assign` - identify the missing owner;
- `split` - move fuzzy work into a named lane;
- `collapse` - summarize known residue;
- `preserve` - keep an interesting contradiction open;
- `ask` - request the one operator decision that blocks progress;
- `watch` - wait for a real external event;
- `implement` - proceed with the scoped change.

### `recur warp collapse-plan`

```powershell
recur warp collapse-plan <lane> [-d <root>] [--json]
```

Separates residue into buckets:

- `collapse_known` - complete, solved, accepted, repeated, or consumed state;
- `preserve_interesting` - contradictions, failed gates, decisions, boundary
  questions, or signals that still change action;
- `blockers` - items that need external approval or an external event;
- `ambiguous` - items requiring config or operator judgment.

This command must be read-only in the first slice. It should never delete or
rename files. A future write-side tool can consume this plan after explicit
operator approval.

### `recur warp explain`

```powershell
recur warp explain <lane> [-d <root>] [--json]
```

Shows how the verdict was derived, including:

- files matched by scope;
- state suffix distribution;
- trace-id role counts;
- related lanes;
- freshness or watch signals when available;
- config weights and hard constraints;
- residual pressure ranking.

### `recur warp config`

```powershell
recur warp config [-d <root>] [--json]
```

Shows the active warp configuration and defaults.

CONFIGURATION
-------------
Projects should be able to tune the scoring without making recur domain-aware.

Possible `.recur/config.toml` shape:

```toml
[warp]
default_scope = "**"
stale_current_days = 7
json_schema = "warp-status-v1"

[warp.weights]
scope_drift = 1.0
schedule_lag = 0.5
effort_waste = 0.5
defect_risk = 1.5
unowned_risk = 2.0
communication_debt = 0.8
dependency_unknowns = 1.2
stale_interest = 0.7
delivered_value = 1.5
learning_value = 0.6

[warp.states]
active = ["current", "composing", "constructing", "ready"]
complete = ["complete", "solved", "approved"]
interesting = ["strange", "questioning", "spike"]
consumed = ["acknowledged", "collapsed", "merged"]

[warp.constraints]
require_named_lane = true
require_visible_next_action = true
require_operator_for_write_side_collapse = true
```

SCORING SIGNALS
---------------
The first version can use simple, auditable signals:

- suffix distribution: count active, complete, strange, acknowledged, solved;
- staleness: age of `.current` files;
- trace-id coverage: define / consume / produce / trigger role counts;
- dangling triggers: trigger lines with no matching producer inside scope;
- unresolved strange: `.strange` files without solved or complete siblings;
- consumed response: `.acknowledged` or worker response referenced by a
  controller record;
- lane spread: too many sibling current files under one prefix;
- missing next action: no trigger, todo, or current owner marker;
- verification: configured test/build/gate marker present or missing;
- risk words: configured risk terms present without accepted/assigned state.

The implementation should prefer transparent heuristics over hidden cleverness.
Every positive or negative signal should cite file evidence or config evidence.

RELATIONSHIP TO EXISTING COMMANDS
---------------------------------
`recur warp` should not replace existing primitives.

- `recur files` remains the enumeration primitive.
- `recur tree` remains the shape primitive.
- `recur related` remains the sibling primitive.
- `recur reveal` remains the lane rehydration primitive.
- `recur trace-id` remains the role-classification primitive.
- `recur watch` remains watcher-state query.
- Companion tools such as `recur-watch`, `recur-version`, and `recur-trace`
  remain responsible for active watching, artifact versioning, and governance
  lineage.

`recur warp` composes those signals into a project-control verdict.

PRIVACY AND GENERICNESS
-----------------------
Tests and examples must use synthetic lanes such as `main.improvement.27`,
`care.subject.routine`, or `demo.project.slice`. Do not encode private names,
medical facts, electronics project details, game project details, or persona
inside core behavior.

Engineering examples must stay synthetic and non-sensitive. Use invented
schematics, benign robotics assemblies, toy production runs, and generic
supersystem terms. Do not encode real defense-system specifications, hazardous
operating parameters, proprietary manufacturing data, or instructions that
could substitute for qualified engineering review.

If a persona wants to use `recur warp`, it should consume the JSON output and
synthesize its own response. The command itself should remain neutral.

PRODUCT POSITIONING AND PERCEPTION
----------------------------------
One early perception is that `recur warp` could have major project-management
implications. Treat that as a possibility to preserve, not as a settled product
claim. The public posture should stay neutral and evidence-based.

The command should not be framed as an attack on existing ticket trackers,
planning suites, or enterprise project-management products. It should be
framed as a local, file-first project-control layer that can integrate with
those systems or operate without them.

Useful positioning boundaries:

- `recur warp` answers "what does the local eventness evidence say should
  happen next?"
- Ticket systems answer broader organizational questions about assignment,
  reporting, approvals, and portfolio visibility.
- The first implementation should preserve import/export paths through JSON,
  Markdown, CSV, or future adapters rather than assuming replacement.
- The differentiator is auditable local state: lanes, suffixes, trace-id roles,
  collapse prediction, residual pressure, and next-action ranking.
- Competitive or partnership strategy belongs outside the command behavior.

This keeps Improvement 27 useful whether recur becomes a standalone planning
engine, a personal/local cockpit, a signal layer feeding existing project
management systems, or something quieter that only improves local eventness
workflows.

FIRST IMPLEMENTATION SLICE
--------------------------
Recommended first slice:

1. Add `recur warp status <lane> --json` as read-only.
2. Compute suffix distribution and trace-id role counts.
3. Emit `optimum`, `sub_optimum`, or `blocked` with residual reasons.
4. Add fixture tests for optimum, sub-optimum, and blocked lanes.
5. Add docs for the JSON schema.

Defer:

- write-side collapse;
- active watching;
- version integration;
- natural-language summaries;
- sophisticated solvers;
- persona-specific behavior.

TRACE-ID LINES
--------------

```text
defines: main.improvement.27 recur warp and project-control command proposal for eventness optimality scoring
defines: recur.warp.status read-only lane verdict command returning optimum sub_optimum or blocked
defines: recur.warp.next ranked next management action proposal based on residual pressures
defines: recur.warp.collapse-plan read-only prediction of collapse_known preserve_interesting blockers and ambiguous residue
defines: recur.warp.explain evidence surface for state suffix distribution trace-id roles related lanes config weights and residual ranking
defines: recur.warp.config configuration surface for weights state mappings and hard constraints
defines: recur.warp.bubble.map declared final qualified Slice coverage and evidence-gate manifest
defines: recur.warp.slice.completion.layer accepted receipt-bound coverage contribution with stable Warp Slice and contract identities
defines: recur.warp.merge pure deterministic composition of target map and accepted Slice layers independent of readiness order where contracts permit
defines: recur.warp.self-reporting derived covered pending blocked conflicting stale and complete state from discoverable layers
defines: recur.warp.explosion observable non-convergence caused by conflict stale contract or falsified material assumption
defines: recur.warp.concept.transition eventness-space change from before a concept is named to after the concept reshapes collapse interest risk and next actions
defines: recur.warp.future.state.convergence comparison between current primitive setup intermediate semi-states and intended supersystem eventness target
defines: recur.warp.supersystem.spec.alignment evidence-first alignment of subsystem constraints capabilities verification and approvals without replacing engineering review
defines: recur.warp.temporal.frame.projection bounded day month year decade projection over an eventness membrane
defines: recur.warp.eventness.membrane scoped prefix base suffix surface whose future semi-states are compared across frames
defines: recur.warp.eventness.epic expert-authored horizon target complete pending and research milestone frame
defines: recur.warp.long.term.epoch long-horizon grouping of multiple warp epics or milestones
defines: recur.warp.cross.domain.pattern reusable eventness structure shared across repeatable tech stacks prototypes and inventions
defines: recur.warp.product.perception early perception that eventness warp may have major project-management implications while public command posture stays neutral
defines: recur.warp.product.positioning vendor-neutral local project-control layer that can integrate with or operate beside ticket systems
consumes: main.improvement.25 reveal lanes and topology over the vault
consumes: main.improvement.26 watcher versioning and governance command boundary
consumes: recur.trace-id define consume produce trigger evidence classification
produces: recur.warp.verdict optimum sub_optimum or blocked verdict for a lane
produces: recur.warp.residuals auditable project-control pressure list with file or config evidence
produces: recur.warp.next-actions scoped management action recommendations
produces: recur.warp.concept.delta auditable before-after residue comparison for concept-driven eventness changes
produces: recur.warp.convergence.residuals auditable gaps between current component evidence and intended future supersystem state
produces: recur.warp.temporal.residuals auditable residual changes across now day month year and decade frames
produces: recur.warp.milestone.map accessible complete pending research and blocked state view for one epic or milestone
produces: recur.warp.success.patterns reusable eventness patterns that can improve efficiency and prototype success probability
produces: recur.warp.integration.surface future JSON Markdown CSV or adapter path for feeding existing project-management systems
produces: recur.warp.composed.coverage reproducible projection whose gaps and conflicts determine remaining Slice pressure
triggers: recur-warp.slice.accept future confirmed write-side persistence of qualified completion layers and receipts
triggers: recur.warp.test.fixtures synthetic optimum sub_optimum and blocked lane fixtures for JSON contract tests
triggers: recur.warp.docs.schema document warp-status-v1 JSON output schema
```
