# Improvement 21: Directory Projection / Namespace Mapping

Status: `todo.future-plan` (proposal / future direction)

## Objective

Keep the directory-projection idea visible in eventness so later work can
rediscover it through the normal improvement tree.

## Current Posture

- proposal only
- no implementation lane should open from this note alone
- use the root proposal as the longform design reference

## What This Improvement Is About

- projecting physical paths into recur namespaces
- letting directories contribute stable semantic prefixes
- reducing how much docs-side naming must manually restate the filesystem

## Discovery

```powershell
recur files "main.improvement.21.**" -d docs/
recur files "README.CORE.IMPROVEMENT21" -d ./
recur find "projection" --scope "main.improvement.21.**" -d docs/ -i
```

## Related

- `README.CORE.IMPROVEMENT21.md`
- `README.CORE.IMPROVEMENT18.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
