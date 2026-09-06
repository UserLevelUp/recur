# main.improvement.27.contract.warp-status-v1.todo.future-plan

## Current split — 2026-09-06

The baseline warp-status-v1 fields and synthetic verdict fixtures are implemented. The large historical JSON sketch below is not the current output contract: prefix_scope, target_state, semi_states, epics and time_frames are proposed extensions, not enabled merely by adding configuration. Actual output/tests are linked in the audited matrix.

See [audited implementation](main.command.warp.docs-reconciliation.current-state.md)
and [claim evidence](main.command.warp.docs-reconciliation.claims.json).
The future-plan filename is retained for remaining proposals and link stability;
it does not mark today's implemented commands as unfinished.

## Historical proposal snapshot — original status and wording follow

Status: `future-plan`
Date: 2026-05-23

## Purpose

Define the JSON contract that should exist before any future `recur warp status`
implementation is exposed.

The contract is intentionally small and evidence-first. It should be stable
enough for tests and personas to consume without making the command clever or
domain-specific.

## Frozen v1 Shape

```json
{
  "schema": "warp-status-v1",
  "lane": "main.improvement.27",
  "scope": "main.improvement.27.**",
  "root": ".",
  "verdict": "sub_optimum",
  "prefix_scope": "demo.supersystem.**",
  "target_state": "demo.supersystem.ready",
  "semi_states": ["demo.supersystem.power.verified", "demo.supersystem.interface.checked"],
  "epics": [
    {
      "name": "demo.supersystem.epic.integration",
      "horizon": "month",
      "target_state": "demo.supersystem.integration.ready",
      "complete": ["demo.supersystem.power.verified"],
      "pending": ["demo.supersystem.interface.checked"],
      "research": ["demo.supersystem.navigation.prototype"],
      "blocked": []
    }
  ],
  "time_frames": [
    {
      "horizon": "now",
      "projected_state": "demo.supersystem.primitive.current",
      "confidence": "observed",
      "residuals": ["missing_interface_verification"]
    },
    {
      "horizon": "month",
      "projected_state": "demo.supersystem.integration.sub_optimum",
      "confidence": "inferred",
      "residuals": ["test_evidence_still_missing"]
    }
  ],
  "objective": 1.5,
  "files": [
    { "path": "main.improvement.27.todo.future-plan.md", "state": "future-plan", "age_days": 0 }
  ],
  "state_suffixes": { "future-plan": 1 },
  "state_groups": { "active": 0, "complete": 0, "interesting": 0, "consumed": 0, "other": 1 },
  "trace_id_roles": { "define": 4, "consume": 1, "produce": 3, "trigger": 1 },
  "signals": [
    { "name": "trace_id_roles_present", "weight": -0.8, "evidence": ["main.improvement.27.todo.future-plan.md"] }
  ],
  "residuals": [
    { "name": "missing_verification_or_complete_state", "weight": 1.5, "evidence": [], "blocker": false }
  ],
  "next_actions": [
    { "kind": "verify", "lane": "main.improvement.27.verification.current", "reason": "record the gate or complete state" }
  ]
}
```

## Field Rules

- `schema` is required and must be `warp-status-v1`.
- `lane` is the user-requested lane.
- `scope` is the actual resolved file scope.
- `verdict` is one of `optimum`, `sub_optimum`, or `blocked`.
- `prefix_scope` is optional in v1 and records the eventness membrane being
  observed, usually a `prefix.base.suffix.*` style scope.
- `target_state` is optional in v1 and names the intended future eventness state
  when the caller supplies one.
- `semi_states` is optional in v1 and lists intermediate acceptable states that
  prove convergence toward the target.
- `epics` is optional in v1 and lists expert-authored milestone frames with
  `complete`, `pending`, `research`, and `blocked` buckets.
- `time_frames` is optional in v1 and lists bounded projections over horizons
  such as `now`, `day`, `month`, `year`, or `decade`.
- `time_frames[].confidence` is one of `observed`, `inferred`, or `speculative`.
- `objective` is residual pressure after scoring, not net optimism.
- `signals` are positive evidence and may have negative weights.
- `residuals` are remaining pressures and must keep a lane out of `optimum`.
- `evidence` values must be concrete file paths or config paths.
- `next_actions` are suggestions only; they must not write, rename, delete, or
  approve project artifacts.

## Frozen Fixture Set

Use only synthetic fixture lanes under `julia-tests/fixtures/warp-status-v1/`:

```text
demo.project.good.complete.md
demo.project.needs.current.md
demo.project.needs.strange.md
demo.project.blocked.current.md
```

Required cases:

- `optimum`: complete or verified state, trace-id roles present, no residuals
- `sub_optimum`: unresolved interesting state, missing trace-id coverage, or
  missing verification
- `blocked`: blocker marker requiring operator approval or external event
- config override: custom suffix mapping changes state grouping

Each fixture has an `expected.json` contract record. The first Rust slice must
produce the required v1 fields with the listed verdict, objective, state groups,
trace-id role counts, residuals, and next actions. Dynamic absolute paths are
not snapshot fields.

The initial v1 implementation deliberately omits `prefix_scope`, target and
semi-states, epics, and time frames unless explicitly configured. Those remain
valid optional extensions and must not complicate the first scorer.

## Deferred Contract Questions

- `objective` is the sum of residual weights; positive signal weights explain
  evidence but do not create negative residual pressure.
- Should target-state and semi-state fields be first-class in v1, or deferred to
  `warp-explain-v1`?
- Should temporal projection frames be first-class in `warp-status-v1`, or only
  appear in a later `warp-project-v1` shape?
- Should epics and milestone buckets be part of `warp-status-v1`, or should
  they be a separate `warp-plan-v1` contract?
- Should default horizons be fixed, or should config define local frame names?
- Should stale age use filesystem mtime, git history, or both?
- Should `future-plan` be its own state group or remain `other`?
- Should `blocked` require a blocker marker, or can it be inferred from missing
  operator approval language?

## Trace-Id Lines

```text
defines: main.improvement.27.contract.warp-status-v1 future JSON contract for read-only recur warp status output
defines: recur.warp.status read-only future lane verdict command returning optimum sub_optimum or blocked
defines: recur.warp.status.schema.v1 schema verdict objective files suffixes roles signals residuals and next_actions
defines: recur.warp.future.state.convergence optional target_state and semi_states fields for comparing current evidence against intended future eventness
defines: recur.warp.temporal.frame.projection optional prefix_scope and time_frames fields for bounded eventness membrane projection
defines: recur.warp.eventness.epic optional epics field for expert-authored milestone buckets complete pending research and blocked
consumes: main.improvement.27.recur-ready future readiness gates for making recur warp implementation-ready without shipping it
produces: recur.warp.test.fixtures synthetic optimum sub_optimum blocked and config override fixture requirements
triggers: recur.warp.status.implementation return to Rust only after schema and fixture expectations are stable
```
