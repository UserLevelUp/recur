# Improvement 22: Reveal Doctrine / Lane-Scoped Rehydration

Status: `todo.future-plan` (proposal / future direction)

## Objective

Keep the reveal-doctrine idea visible in eventness so the lane-local gift /
`*.recur.md` concept can be rediscovered through the normal improvement tree.

## Current Posture

- first shippable slice landed:
  - `[reveal]` defaults now scaffold through `recur init`
  - `recur reveal` can list and open lane-local `*.recur.md` files
  - repo dogfoods reveal capsules in active lanes
- broader doctrine remains future-plan
- treat the root proposal as the longform design note
- do not treat this one landed slice as the whole reveal doctrine

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

## Related

- `README.CORE.IMPROVEMENT22.md`
- `README.CORE.IMPROVEMENT21.md`
- `docs/main.command.reveal.readme.md`
- `docs/main.improvement.22.recur.md`
- `docs/main.improvement.21.todo.future-plan.md`
- `docs/main.improvement.14.todo.future-plan.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `docs/main.recur.expert.recurring.md`
