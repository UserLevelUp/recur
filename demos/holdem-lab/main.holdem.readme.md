# Hello World to Texas Hold'em, Lang first

A progressive experiment in using compact contracts to design a growing Julia
app. The six numbered `.recur` files preserve the specification stages; matching
`.jl` and `.test.jl` files implement and test them. No language parser changes or
Python dependencies are needed.

## Run from the Recur repository root

```powershell
$env:RECUR_BIN = (Get-Command recur).Source
julia --startup-file=no -C generic demos/holdem-lab/main.holdem.app.jl
julia --startup-file=no -C generic --project=demos/web-evidence-lab demos/holdem-lab/main.holdem.test.jl

recur tree main.holdem -d demos/holdem-lab
recur files 'main.holdem.05.**' -d demos/holdem-lab
recur lang -d demos/holdem-lab
recur lang show main.holdem.05.play.recur --scope view.v -d demos/holdem-lab
recur lang show main.holdem.06.coordination.recur --scope settle.s --expand -d demos/holdem-lab
recur trace-id demo.holdem.play --scope 'main.holdem.**' -d demos/holdem-lab --format full
```

The test command reuses the existing demo environment's JSON3 dependency. On a
fresh checkout instantiate `demos/web-evidence-lab` first. Runtime-only tests and
the app use Julia standard libraries. The new suite is also included in
`julia-tests/runtests.jl`. Use a current Lang-capable binary: version 0.2.8 alone
does not distinguish older installed builds from the updated build.

The app prints a scripted two-player hand from greeting through showdown. It is
a local teaching app, not a full poker client. See
[requirements](main.holdem.requirements.md) for its bounded betting rules.

## What grew at each step

| Stage | Contract-first addition | Runtime acceptance |
| --- | --- | --- |
| 01 | One name bundle and one message result | Default Hello World and explicit name |
| 02 | Julia `HelloApp` class-style boundary, greeting-to-caller alias | Defaults, overridden prefix, trimmed/blank names |
| 03 | Deck to deal; one result symbol holds four collections | 52 unique cards, burn positions, private holes, deterministic injection |
| 04 | Ordered rank and two-ranking comparison | All categories, wheel, kickers, ties, suit/permutation invariance |
| 05 | Start, command, view, render; a ten-field state bundle | Turns, raises, blinds, street progression, folds, all-in, replay, conservation |
| 06 | Named contracts, two ranking producers, join, settlement, audit | Both producer identities, failure propagation, odd chip, duplicate-card rejection |

The specs and behavioral examples were written before each numbered
implementation. Each first Julia run failed because that stage's implementation
file did not yet exist; the behavioral assertions then ran after implementation.
Those initial failures establish the observed authoring order, not mutation-test
coverage. The later graph faults deliberately change otherwise working models.

The body becomes a readable account of relationships:

```text
i(a) -> table.plan(a)
     -> fork [left(a), right(a)]
     -> await [left.o(b), right.o(b)]
     -> settle(a)
     -> await settle.o(b)
     -> audit(a)
     -> await audit.o(b)
     -> table.finish(a) -> o(b)
```

`settle.i(a)` contains three distinct messages: the request and the two rankings.
Those messages expose seven fields when expanded. `left.o(b)` and `right.o(b)`
share the named `Ranking` contract but keep separate producer identities. The
body never has to repeat the request fields or ranking tie breakers.

Julia implements the graph explicitly with two tasks and a join; Lang itself
does not launch either task. Cooperative tasks demonstrate ordering, not CPU
parallel performance. The earlier betting implementation and the stage-06
coordinator are intentionally independent implementations of showdown settlement,
so both can be compared. The sample hand agrees on player 2 winning its 12-chip pot.

## Can it reveal circular dependencies?

`main.holdem.graph.test.jl` generates each altered spec in a temporary directory,
queries the real CLI, and checks exact failure classes. The six valid specs stay
unchanged. Run that file directly with the same project argument to reproduce.

| Injection | Observed result |
| --- | --- |
| Left ranking consumes its own output | exit 1, SGR001: `left -> left` |
| Left ranking requires the final audit | exit 1, SGR001: `audit -> left -> settle -> audit` |
| Awaiting audit routes back to settlement | exit 1, SGR002: `audit -> settle -> audit` |
| Settlement awaits only the left ranking | exit 1, SGR004: incomplete join |
| Select only the unaffected right lane | Same full graph and findings remain visible |
| Remove the injected relationship | Original DAG passes with no blocking findings |
| Hide a bounded hello/caller cycle inside Julia only | Lang passes; runtime probe throws; spec hash is unchanged |

These are graph diagnostics, not malformed-input errors. This is useful early
feedback: a single erroneous relationship produces a concrete closed path before
any worker runs. It does not demonstrate a statistically measured reduction in
bugs, nor prove that implementation dependencies agree with the authored model.

The state machine's next action is deliberate sequential feedback. It is carried
as data by the controller; the evaluator and renderer never call back into the
game. It is not modeled as a dependency from final output back to initial input.

## Friction discovered during authoring

1. WIR1 rejects output-to-output aliases with RLIR011. Stage 05 initially tried
   `command.o(d) := start.o(b)`; it must repeat the output fields today. Identical
   shapes remain distinct contracts. CIR1 named contracts provide reuse.
2. For a multiline CIR1 flow, put the expression on the line *after* `async :`.
   Starting the expression on the declaration line caused the parser to stop
   there and report RCIR006 (no fork), despite a fork on a following line.
3. Compact CIR1 JSON uses `input.members`; `--expand` additionally exposes
   `input.messages`. Tests assert the graph is identical across both views.
4. WIR1 cannot fully express or validate the state-plus-action tuple's nested
   type identity, the two-evaluator join, poker rules or the correspondence to
   Julia calls. Those need runtime tests and the separate CIR1 graph.
5. A binding names an implementation; it does not inspect that implementation.
   Only changing the Julia file leaves the Lang source fingerprint unchanged.

These findings support focused follow-ups: reusable named WIR1 output contracts,
clearer multiline-flow diagnostics, explicit state-transition contracts, and
source-bound checks that compare declared dependencies with implementation calls.
The current experiment does not implement those language extensions.

## Evidence and limits

The focused suite observed 527 passing assertions on Julia 1.12.7 and the locally
updated Recur 0.2.8. Tests include golden poker hands, 30 seeded action sequences,
real CLI graph failures and trace-id lineage from requirements through code/tests.
Trace-id records declarations; it does not certify actual runtime call paths.

All Lang checks report `whole_source_validated: false` and `execution: not-run`.
No accepted Warp receipt or complete-state artifact was published for this work.
The existing web Lang inspector accepts WIR1 only; the CIR1 experiment currently
uses the CLI. This demo does not change the web server catalog.

An optional exhaustive five-card category-count oracle is provided:

```powershell
julia --startup-file=no -C generic demos/holdem-lab/main.holdem.exhaustive.jl
```

It enumerates all 2,598,960 combinations against combinatorial counts. Local
attempts with normal compilation and `-O0 --inline=no` terminated in Julia
access violations; exhaustive correctness has **not** been established by those
attempts. The focused hand examples passed. See `main.holdem.observations.md`
for the full-regression outcome and recorded validation commands.

consumes: demo.holdem.graph observed supported graph checks and coverage limits
consumes: demo.holdem.play observed runtime acceptance
