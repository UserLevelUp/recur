# Choco Package: Keep Current

Status: `todo.current` (ongoing maintenance task)

## Purpose

Keep the Chocolatey package metadata in sync as commands are added or updated. The nuspec description and command list should always reflect what `recur --help` actually shows.

## Current Gap

No current gap for command coverage (synced at v0.2.8, 2026-03-08).

`choco/recur.nuspec` currently lists all `recur --help` commands:
- `recur init`
- `recur files`
- `recur find`
- `recur tree`
- `recur related`
- `recur children`
- `recur id`
- `recur stats`
- `recur callers`
- `recur callees`
- `recur trace`
- `recur merge`
- `recur flatten`
- `recur trait`
- `recur trace-id`
- `recur trace-stats`

It also calls out the companion `recur-git checkpoint` command installed when `recur-git.exe` is present in the release zip.

## Trigger: When to Update

Update `choco/recur.nuspec` whenever:
- A new command is added to recur
- A command description changes
- Features are added that should be listed (for example, stdin composability for all commands)
- Tags should be updated (for example, add `flatten`, `xml`, `json`, `call-graph`)
- `recur-git` adds/removes commands that should be mentioned in package description

## Discovery

```bash
# Compare what recur has vs what nuspec says
recur --help
recur-git --help
cat choco/recur.nuspec

# Check current version
cat VERSION
cat Cargo.toml | head -3
```

## Files to Update
- `choco/recur.nuspec` - command list, description, tags
- `choco/tools/VERIFICATION.txt` - if verification steps change
