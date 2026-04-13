# Improvement 20: Pipe-Friendly Filter Command

Status: `todo.future-plan` (proposal / future direction)

## Objective

Track the future `recur filter` idea in the improvement tree so pipeline
composition work stays discoverable even while implementation remains deferred.

## Current Posture

- proposal only
- preserve the idea as composition doctrine, not as active build-now work
- use the root proposal as the canonical longform reference

## What This Improvement Is About

- a native filter stage for recur pipelines
- stdin-first narrowing of path or JSON streams
- less shell-specific glue around `files`, `tree`, and `merge`

## Discovery

```powershell
recur files "main.improvement.20.**" -d docs/
recur files "README.CORE.IMPROVEMENT20" -d ./
recur find "filter" --scope "main.improvement.20.**" -d docs/ -i
```

## Related

- `README.CORE.IMPROVEMENT20.md`
- `docs/main.command.compose.readme.md`
- `docs/main.capability.stdin-stdout-piping.md`
