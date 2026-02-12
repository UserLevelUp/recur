# Choco Package: Keep Current

Status: `todo.current` (ongoing maintenance task)

## Purpose

Keep the Chocolatey package metadata in sync as commands are added or updated. The nuspec description and command list should always reflect what `recur --help` actually shows.

## Current Gap

**`recur --help` shows 12 commands. `choco/recur.nuspec` lists 8.**

### Missing from nuspec:
- `recur callers` — find all places where a function/method is called
- `recur callees` — find all functions/methods that a given function calls
- `recur trace` — multi-level call graph visualization
- `recur flatten` — flatten structured files (XML, JSON) into hierarchical dot-paths

### Currently listed in nuspec:
- `recur files` — find files by hierarchical pattern
- `recur find` — search text within a hierarchy scope
- `recur tree` — visualize hierarchy as a tree
- `recur related` — find sibling files
- `recur children` — find child files
- `recur id` — search for hierarchical identifiers
- `recur stats` — analyze hierarchy statistics
- `recur merge` — merge results across separators

## Trigger: When to Update

Update `choco/recur.nuspec` whenever:
- A new command is added to recur
- A command description changes
- Features are added that should be listed (e.g., stdin composability for all commands)
- Tags should be updated (e.g., add `flatten`, `xml`, `json`, `call-graph`)

## Discovery

```bash
# Compare what recur has vs what nuspec says
recur --help
cat choco/recur.nuspec

# Check current version
cat VERSION
cat Cargo.toml | head -3
```

## Files to Update
- `choco/recur.nuspec` — command list, description, tags
- `choco/tools/VERIFICATION.txt` — if verification steps change
