# Warp 10 Sub-7: Playable Runtime Acceptance

## Scope

Reality-test the entire Warp 10 player loop against the frozen final contract.
This slice integrates and verifies prior work; it does not waive their residual
requirements.

## Required run

```text
launch -> orient and pan map -> select site -> transition to site
       -> approach person -> converse and choose -> gain/refine idea
       -> save/load proof -> leave site -> restored map context
```

## Acceptance evidence

- Exact source revision and clean description of any runtime-only configuration.
- End-to-end capture plus structured observations for every required run step.
- Godot debugger output and scene/resource loading results.
- Map hit-testing, transition restoration, dialogue branching, idea
  idempotency, persistence, focus, controller, responsive-layout, and
  reduced-motion results.
- Journey trace covering interaction-state order, transition budget, focus and
  input-lock ownership, presentation depth, fallbacks, and recovery.
- Human observation of hesitation, backtracking, repeated input, loss of
  context, and route abandonment; unexplained confusion remains a residual even
  when automated checks pass.
- Focus-group residual matrix updated with accepted, rejected, deferred, and
  still-open findings and reasons.
- Known risks, limitations, and follow-up Eventness preserved.

## Final transition

Only accepted runtime evidence allows this artifact and the Warp 10 final to
be recorded complete.  Partial success remains current or strange; a static
suite or polished capture alone cannot complete Warp 10.
