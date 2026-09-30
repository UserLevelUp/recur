# Recur Lang: remaining work

```text
defines: recur.lang.work.remaining next focused choices for the opinionated companion
consumes: recur.lang Lang design context across all work items
consumes: recur.lang.companion opinionated companion implementation context
consumes: recur.eventness.todo stable todo identity and attention convention
```

Status: open. Updated: 2026-09-29. Design parent:
[Improvement 30](README.CORE.IMPROVEMENT30.md).

Use this as one overall todo, or extract a bounded item into
`README.CORE.IMPROVEMENT30.recur-lang.<focus>.todo.md` when it needs separate
attention. Any number of named focus files is allowed. Keep detail in its focus
file and link it here instead of maintaining competing checklists.

On verified completion, record the source revision, checks and evidence, then
rename the focused file to `<focus>.todo.checked.md` (or append `.checked` after
any existing Eventness segments). Update this index's link and checkbox. Leave
this parent open until its remaining scope is resolved. Filename checking and
formal Warp gate acceptance are separate; see
[Eventness policy](README.CORE.EVENTNESS.md#focused-todos-and-checked-residue).

## Selected focus

consumes: recur.lang.work.remaining.init focused initialization todo
consumes: recur.lang.init.v1 exact initialization contract

- [ ] [Initialize project policy](README.CORE.IMPROVEMENT30.recur-lang.init.todo.md)
  through `main.lang.init`. The contract/map/standalone red tests exist; implement
  additive `[recur-lang]` defaults with preview and preservation first.

## Following increments

- [ ] Define and implement the proposed `main.lang.implementation-plan` Warp.
  Consume initialized policy and existing WIR1/CIR1/SGR1 facts to produce a
  source-bound work packet with implementation boundaries, prioritized graph
  findings, suggested tests, unresolved decisions and required evidence.
  No map or `recur-lang plan` command exists yet.

  defines: recur.lang.work.remaining.planning policy-informed implementation and test packets

- [ ] Define a separate scaffolding increment after planning is useful. Generate
  reviewable implementation/test skeletons for an explicitly selected target,
  preserve user code and trace IDs, and make repeated use predictable.

  defines: recur.lang.work.remaining.scaffolding implementation and test skeleton generation

- [ ] Investigate declared-versus-implemented dependency checking. The Hold'em
  experiment catches declared cycles but deliberately demonstrates that a Julia
  cycle hidden behind a binding is outside current static coverage.

  defines: recur.lang.work.remaining.dependencies declaration-to-implementation correspondence
  defines: recur.lang.work.remaining.dependencies.hidden-cycles implementation-only circular references
  consumes: demo.holdem.graph observed graph checks and coverage limits

- [ ] Turn observed authoring friction into bounded follow-ups: reusable WIR1
  output contracts (output-to-output aliases currently raise RLIR011), clearer
  multiline CIR1-flow diagnostics, and explicit state-plus-action contracts.
  Keep proposed syntax separate from implemented grammar.

  defines: recur.lang.work.remaining.contracts reusable outputs and state-plus-action contracts
  defines: recur.lang.work.remaining.diagnostics multiline coordination-flow errors

## Evidence and integration work still open

defines: recur.lang.work.remaining.evidence verification freshness and integration work
consumes: recur.lang.work.remaining.verification detailed branch-analysis follow-ups

- [ ] Work through the [baseline and evidence verification focus](README.CORE.IMPROVEMENT30.recur-lang.verification.todo.md).
  Its [runnable demos and test catalog](demos/lang-verification/main.lang.verification.readme.md)
  feed the tests-first `main.lang.verification` Warp; advanced features remain
  in separate `main.lang.advanced.*` contract-first Warps.
  It incorporates the supplied branch analysis, two locally reproduced query
  discrepancies, the inspected path-test/CI gaps, and unexecuted corruption and
  parser hypotheses. Its detailed checklists own those follow-ups; the items
  below retain the integration priorities and links.

- [ ] Recover a reproducible Julia validation run. The Hold'em lab observed a
  clean 527-assertion pass, then intermittent compiler/interpreter faults in
  later focused and full-suite runs. Exhaustive poker validation remains
  unestablished. Read the [observations](demos/holdem-lab/main.holdem.observations.md).

  defines: recur.lang.work.remaining.evidence.julia reproducible validation after runtime faults

- [ ] Reassess the existing [checked-transition](warps/main.lang.checked-transition.readme.md)
  Warp's stale evidence against the current source. Its implementation exists;
  stale acceptance is not a reason to reimplement it.

  defines: recur.lang.work.remaining.evidence.checked-transition current evidence for existing transitions

- [ ] Resume [runtime evidence](warps/main.lang.runtime-evidence.readme.md) and
  browser-inspector association work using its existing contract and actual
  blockers. The browser currently supplies no runtime receipts and supports
  WIR1 views; it does not yet present the CIR1 Hold'em graph.

  defines: recur.lang.work.remaining.evidence.runtime browser association and observed receipts

- [ ] Reconcile affected Warp evidence and run appropriate Cargo/Julia tests
  before installation or a release claim. Keep failure logs and accepted
  historical observations distinct from current source freshness.

  defines: recur.lang.work.remaining.evidence.release validation before installation or release claims

The broader Improvement 30 proposals (subsystems/imports, general execution,
bounded feedback, grids and multi-worktree coordination) remain design context.
Create focused todos and explicit Warps when choosing to implement those scopes;
this list does not assert that every design proposal is committed release work.

## Rehydrate

```powershell
recur tree README.CORE.IMPROVEMENT30.recur-lang -d . --sep .
recur files 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --sep .
recur warp show main.lang.init -d .
recur warp slices main.lang.init -d .
recur trace-id 'recur.lang.work.remaining.**' --scope 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --format full
recur trace-id 'recur.lang.work.remaining.dependencies.**' --scope 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --format full
recur trace-id recur.lang.init.v1 --scope '**' -d . --format full
```
