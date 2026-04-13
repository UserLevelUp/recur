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

## Related

- `README.CORE.IMPROVEMENT22.md`
- `docs/main.command.reveal.readme.md`
- `docs/main.improvement.22.recur.md`
- `docs/main.improvement.14.todo.future-plan.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `docs/main.recur.expert.recurring.md`
