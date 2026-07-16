# main.improvement.27.epic.milestone.todo.future-plan

Status: `future-plan`
Date: 2026-05-23

## Purpose

Capture the warp planning layer: expert-authored eventness epics, steps, and
milestones that make complete, pending, research, and blocked states visible
across domains.

This is still future-plan. It does not mean `recur warp` has shipped.

## Concept

A warp epic is a framed target over a scoped eventness membrane:

```text
epic = horizon + target_state + complete_set + pending_set + research_set
milestone = target_state + evidence_gates + state_buckets
```

The expert supplies the intended milestones. Recur supplies the eventness
inspection surface. The command should show how the current files line up with
the planned epic or milestone:

- `complete`: evidence that should exist and does exist
- `pending`: named work that belongs in the epic but is not complete yet
- `research`: unknowns, prototypes, experiments, trade studies, or capability
  questions
- `blocked`: items waiting on approval, external evidence, missing specs, or
  unresolved safety/risk decisions

An epoch is still useful for long-term warping, but it should mean a larger
time horizon that groups multiple epics or milestones. The near-term planning
unit is the epic, step, or milestone.

## Cross-Domain Use

The same eventness shape should work for repeatable technology stacks and new
inventions:

- benign schematic and voltage expectation checks
- robotics capability envelopes and subsystem interfaces
- software releases and platform migrations
- production-run conformance and inspection evidence
- research prototypes and invention paths
- complex aerospace examples only when synthetic and non-sensitive

The value is commonality. Repeatable work can become more efficient because the
epic shape is reusable. New prototypes can inherit proven milestone patterns
without pretending the new invention is already solved.

## Boundary

`recur warp` should audit local eventness against expert-authored epics and
milestones. It must not invent domain specifications, certify safety, authorize
engineering changes, or replace qualified review.

## Candidate Script Surface

A future cross-domain script or wrapper could consume the same JSON contract:

```powershell
recur warp status demo.supersystem --json
recur warp explain demo.supersystem --json
recur warp next demo.supersystem --limit 5 --json
```

The script should remain generic: it may render complete/pending/research views
and reusable milestone patterns, but any domain-specific authority belongs to
the operator, project config, or external qualified tooling.

## Trace-Id Lines

```text
defines: main.improvement.27.epic.milestone future-plan lane for expert-authored warp epics steps and milestones
defines: recur.warp.eventness.epic expert-authored horizon target complete pending and research milestone frame
defines: recur.warp.milestone.map accessible complete pending research and blocked state view for one epic or milestone
defines: recur.warp.long.term.epoch long-horizon grouping of multiple warp epics or milestones
defines: recur.warp.cross.domain.pattern reusable eventness structure shared across repeatable tech stacks prototypes and inventions
consumes: recur.warp.temporal.frame.projection bounded projection over now day month year and decade eventness frames
consumes: recur.warp.future.state.convergence intended future eventness state and intermediate semi-state framing
produces: recur.warp.success.patterns reusable eventness patterns that can improve efficiency and prototype success probability
triggers: main.improvement.27.contract.warp-status-v1 consider epics milestones and state buckets in JSON contract
```
