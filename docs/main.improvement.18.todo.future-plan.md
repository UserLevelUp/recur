# Improvement 18: recur-map

Status: `todo.future-plan` (brainstorm / mapping direction only)

## Objective

Keep the `recur-map` direction visible in the normal improvement tree without
pretending it is an active implementation lane.

## Current Posture

- concept and mapping vocabulary are worth preserving
- do not open `todo.current` from this note alone
- use the root proposal doc as the longform design reference

## What This Improvement Is About

- mapping identifiers across namespaces
- projecting one tree into another
- making cross-domain relationships queryable instead of memory-only

## Discovery

```powershell
recur files "main.improvement.18.**" -d docs/
recur files "README.CORE.IMPROVEMENT18" -d ./
recur find "recur-map" --scope "main.improvement.18.**" -d docs/ -i
```

## Related

- `README.CORE.IMPROVEMENT18.md`
- `docs/main.improvement.9.trace-id.complete.md`
- `docs/main.dogfooding.trace-id.todo.md`
