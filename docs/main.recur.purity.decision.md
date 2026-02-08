# Recur Purity Decision

Date: 2026-02-07

## Decision

**Recur will remain pure** - focused only on hierarchical file management.

## What Recur Does

✅ **Core functionality:**
- Hierarchical file pattern matching
- Tree visualization
- File discovery and listing
- Content search within hierarchical scopes
- Related file discovery

## What Recur Does NOT Do

❌ **Out of scope:**
- Git integration (moved to `recur-git`)
- Checkpoint functionality (moved to `recur-git`)
- File content manipulation
- Build system integration

## Separation of Concerns

### recur (Pure Hierarchy Tool)

```bash
recur files "main_command_*" -d src/ --sep _
recur tree "main.improvement" -d docs/
recur find "async" --scope "**"
```

**Focus:** Understanding and querying hierarchical file structures.

### recur-git (Git Workflow Extension)

```bash
recur-git checkpoint --snapshot
recur-git checkpoint --append-parallel
```

**Focus:** Git-aware workflows, lane tracking, checkpoint snapshots.

## Implementation Status

✅ **Already separated!**
- Built as two separate binaries
- `recur` has NO git dependencies
- `recur-git` handles all git operations

## Cleanup Done

Removed checkpoint TODO files from src/:
- ❌ `src/main_command_checkpoint_todo_current.md` (deleted)
- ❌ `src/main_command_checkpoint_todo_next.md` (deleted)
- ❌ `src/main_command_checkpoint_todo_trigger_event.md` (deleted)

Documented in docs/:
- ✅ `docs/main.command.checkpoint.out-of-scope.md`
- ✅ `docs/main.git.checkpoint.readme.md` (for recur-git)

## Future Scope

### Improvement 9-10 (Distant Future)

Recur will add support for **hierarchical lists within files**:
- Parse dot-separated lists inside file content
- Query nested structures within files
- Still pure - no git, no external integrations

Example:
```
main.task.1.todo
main.task.1.complete
main.task.2.todo
```

Stored in a file, queryable with recur.

## Verification

```bash
# Recur does NOT have checkpoint
recur checkpoint --snapshot
# Output: error: unrecognized subcommand 'checkpoint'

# Recur-git DOES have checkpoint
recur-git checkpoint --snapshot
# Output: == Checkpoint Snapshot == ...
```

## Philosophy

**Do one thing well.**

Recur understands hierarchies. That's its job. Git workflows are a separate concern, handled by a separate tool.

This keeps:
- Code simple
- Dependencies minimal
- Focus clear
- Testing easier
