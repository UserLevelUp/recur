# Improvement 22: Reveal Doctrine / Lane-Scoped Rehydration

Status: `todo.future-plan` (proposal / future direction)

## Objective

Keep the reveal-doctrine idea visible in eventness so the lane-local gift /
`*.recur.md` concept can be rediscovered through the normal improvement tree.

## Current Posture

- phase 1 shipped:
  - `[reveal]` defaults scaffold through `recur init`
  - `recur reveal` lists and opens lane-local `*.recur.md` files
  - repo dogfoods reveal capsules in active lanes
- phase 2 next slice: `current_thread` field in `[reveal]`
  - `recur reveal` with no args opens the configured thread directly
  - falls back to listing all capsules when `current_thread` is absent
  - `recur init` scaffolds the field blank; user sets it when ready
  - delight state: `recur reveal` alone is enough to resume any session
- broader doctrine remains future-plan
- treat the root proposal as the longform design note
- do not treat these landed slices as the whole reveal doctrine

## What This Improvement Is About

- a reveal doctrine in `.recur/config.toml`
- one lane-local `*.recur.md` gift as the default pull-point
- calmer cold-start rehydration for humans and AI
- cleaner multi-agent coordination through bounded lane reveals

## Discovery

```powershell
recur files "main.improvement.22.**" -d docs/
recur files "README.CORE.IMPROVEMENT22" -d ./
recur find "reveal doctrine" --scope "main.improvement.22.**" -d docs/ -i
```

## Multi-Agent Coordination Goal

Improvements 21 and 22 are the joint foundation for multi-agent coordination:

- **Improvement 21** (directory projection / namespace mapping) gives each agent
  a physically scoped lane — a directory that projects into a logical namespace
- **Improvement 22** (reveal doctrine) gives each lane a consciousness capsule —
  a `*.recur.md` gift in `.recur/` that carries persona, agenda, and pull-point

Together they enable:
- agents working in separate physical lanes without stepping on each other
- each agent knowing its own context without reading the whole repo
- coordinated merge, build, test, iterate, commit, and push across lanes
- `.recur/` as the vault — lane-local trade context that need not be public

The `.recur/` vault is the endgame. `docs/*.recur.md` is the bootstrap.

## trace-id is the Handoff Contract

The verification mechanism between lanes is not a new primitive — it is
`trace-id` applied at lane scope.

A lane's work produces eventness files. Those files carry publish/subscribe/
trigger relationships to other lanes. Before merge is allowed, `trace-id`
verifies the cascade is complete: all subscribers satisfied, all triggers
resolved.

The reveal capsule already speaks this language:

```toml
default_handoff = "verify passes and lane truth updated"
default_touch   = "declared-by-lane"
```

That is trace-id vocabulary in reveal clothing. An agent that wandered out
of its declared scope shows up as a subscription with no matching publisher
in the expected lane. No separate improvement needed — the tools are already
there, aimed at a higher scope level.

## Related

- `README.CORE.IMPROVEMENT22.md`
- `README.CORE.IMPROVEMENT21.md`
- `docs/main.command.reveal.readme.md`
- `docs/main.improvement.22.recur.md`
- `docs/main.improvement.21.todo.future-plan.md`
- `docs/main.improvement.14.todo.future-plan.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `docs/main.recur.expert.recurring.md`
