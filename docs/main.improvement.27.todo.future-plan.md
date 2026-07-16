# main.improvement.27.todo.future-plan

Status: `future-plan`
Date: 2026-05-22

## Purpose

Make Improvement 27 discoverable from the docs-side improvement eventness tree.

## Canonical Proposal

- `README.CORE.IMPROVEMENT27.md`

## Summary

Improvement 27 proposes a generic `recur warp` command family for eventness
optimality and project-control analysis:

- `recur warp status <lane>` scores a lane as `optimum`, `sub_optimum`, or
  `blocked`.
- `recur warp next <lane>` ranks next scoped management actions.
- `recur warp collapse-plan <lane>` predicts which residue should collapse and
  which should remain interesting.
- `recur warp explain <lane>` shows evidence and residual pressures.
- `recur warp config` shows configured weights, state mappings, and hard
  constraints.

The command family composes existing recur primitives rather than replacing
them: `recur reveal`, `recur files`, `recur tree`, `recur related`, and
`recur trace-id` provide the raw evidence; `recur warp` turns that evidence
into an auditable project-management verdict.

The guiding intuition is a concept transition: eventness before a concept is
named has one geometry; eventness after the concept is named has another.
`recur warp` should make that before/after residue inspectable enough to show
what collapsed, what stayed interesting, and what next action became visible.

The larger intuition is future-state convergence. Warp should help a complex
system move from primitive pieces toward a desired supersystem state by naming
the target state, the acceptable intermediate semi-states, and the residual
gaps between current evidence and final build readiness. Synthetic examples can
include schematics, voltages, robotics capability envelopes, subsystem
interfaces, and production-run conformance, but core behavior must stay generic,
evidence-first, and non-certifying.

Warp also has a temporal projection reading. Given a scoped membrane such as
`prefix.base.suffix.*`, it should be able to capture eventness frames over
useful horizons such as now, day, month, year, or decade, then describe how the
medium is likely to reform: observed states, plausible semi-states, blocked
states, growing residuals, shrinking residuals, and branches that may need to
exist for the intended future state.

The planning unit is usually an eventness epic, step, or milestone: an
expert-authored target that names the complete, pending, research, and blocked
states that should be visible at that point. A longer epoch can group many
epics or milestones for long-term warping. That makes warp useful for
repeatable technology stacks and for new prototypes: common milestone patterns
can make known work more efficient and give new inventions a
higher-probability path without pretending the system is already solved.

The first slice should eventually be read-only and JSON-capable. Write-side
collapse, active watching, version integration, and persona-specific summaries
are deferred.

## Parking Decision

For the current packaging/release lane, keep Improvement 27 as future-plan only.
Do not expose `recur warp` in the command surface yet.

The likely implementation horizon is a later maturity release such as `0.2.20`,
after the scoring contract, fixtures, and command family boundaries settle.
When implementation resumes, prefer a core `recur warp status` query first; a
separate `recur-warp` companion should wait for write-side collapse or
approval-gated automation.

## Recur-Ready Sublanes

This proposal is now split into durable future-plan sublanes so `recur tree
"main.improvement.27" -d docs/` shows the next useful work without implying the
command has shipped:

- `docs/main.improvement.27.recur-ready.todo.future-plan.md`
- `docs/main.improvement.27.contract.warp-status-v1.todo.future-plan.md`
- `docs/main.improvement.27.command-boundary.todo.future-plan.md`
- `docs/main.improvement.27.epic.milestone.todo.future-plan.md`

## Product Positioning And Perception

One early perception is that `recur warp` may compete with, complement, or feed
existing ticket trackers and project-management suites. Treat that as a possible
product implication, not a settled strategy. The proposal should stay
vendor-neutral, and the command behavior should remain a local evidence surface:
eventness in, auditable verdict and next-action ranking out.

Future integration paths should prefer neutral formats such as JSON, Markdown,
CSV, or adapters rather than assuming replacement.

## First Test Todo

Add synthetic fixture tests for:

- an `optimum` lane with complete artifacts, trace-id coverage, and no residual
  blockers;
- a `sub_optimum` lane with stale current work or unresolved strange state;
- a `blocked` lane requiring operator approval or an external event;
- JSON output contract for `warp-status-v1`;
- config-driven weights and state suffix mappings.

## Discovery

```powershell
recur files "main.improvement.27.**" -d docs/
recur tree "main.improvement.27" -d docs/
recur files "README.CORE.IMPROVEMENT27" -d . --sep .
recur trace-id "recur.warp.status" --scope "main.improvement.27.**" --dir docs --ext md
recur files "main.improvement.27.**.todo.future-plan" -d docs/
```

## Trace-Id Lines

```text
defines: main.improvement.27 docs-side future-plan bridge for recur warp and project-control command proposal
defines: recur.warp.status read-only future lane verdict command returning optimum sub_optimum or blocked
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
consumes: main.improvement.27 recur warp and project-control command proposal for eventness optimality scoring
produces: main.improvement.27.future-plan discoverable improvement tree handle for the warp command proposal
produces: main.improvement.27.recur-ready future implementation gates for making warp command-shaped without shipping it
produces: main.improvement.27.contract.warp-status-v1 future JSON contract lane for read-only status output
produces: main.improvement.27.command-boundary future command split lane for core recur query vs companion automation
produces: main.improvement.27.epic.milestone future-plan lane for expert-authored warp epics steps and milestones
produces: recur.warp.status.future-plan docs-side readiness handle without public command exposure
produces: recur.warp.concept.delta auditable before-after residue comparison for concept-driven eventness changes
produces: recur.warp.convergence.residuals auditable gaps between current component evidence and intended future supersystem state
produces: recur.warp.temporal.residuals auditable residual changes across now day month year and decade frames
produces: recur.warp.milestone.map accessible complete pending research and blocked state view for one epic or milestone
produces: recur.warp.success.patterns reusable eventness patterns that can improve efficiency and prototype success probability
produces: recur.warp.integration.surface future JSON Markdown CSV or adapter path for feeding existing project-management systems
triggers: recur.warp.test.fixtures synthetic optimum sub_optimum and blocked lane fixtures for JSON contract tests
```
