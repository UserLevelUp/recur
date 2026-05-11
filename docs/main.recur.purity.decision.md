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

### Companion Actors + Pure Query Surfaces

The same split applies beyond git.

Core `recur` should stay pure:

- read hierarchies
- merge configured hierarchy lanes
- inspect eventness
- explain state
- query status and history
- exit

Opinionated companion binaries may do narrower operational work:

- `recur-git` performs git-aware checkpoint operations
- `recur-watch` performs long-running watch/subscription loops
- future companions may perform versioning, tracing, approval proposals, or
  other implementation-specific workflows

The companion performs the action.
Core `recur` inspects the eventness left behind by that action.

This gives each command family a clean pair:

```text
recur <topic>        = pure query / inspection / explanation
recur-<topic>        = opinionated runner / writer / async actor
```

Examples:

```text
recur watch          = inspect watcher state, list active/stale watches, explain filters
recur-watch          = run the active subscription loop

recur git            = inspect git workflow state, checkpoint history, lane ACK/NAK records
recur-git            = perform git-aware checkpoint operations

recur version        = inspect version policy, manifests, history, diffs
recur-version        = save snapshots, update manifests, enforce write policy

recur trace          = inspect technical call/flow relationships already present
recur-trace          = future lineage/provenance actor if responsibility tracing
                       needs an opinionated writer
```

This preserves recur's ability to utilize any reasonable hierarchy without
absorbing every workflow engine into the core binary.

## ACK/NAK Eventness Rule

When a companion actor performs an operation, it should leave a small
machine-readable state record that core `recur` can inspect.

That record should include both ACK and NAK information:

- ACK: what request was accepted, what scope/filter/policy was used, and what
  state is now active
- NAK: what request was understood, why it was rejected or partial, and what
  the caller can inspect next

This is not only for `recur-watch`.
The same handshake pattern should apply to any companion that performs
narrow, opinionated work.

Example watcher state:

```text
id = docs-monkey
state = active
ack = accepted
nak_reason = ""
filter = monkey.**
dir = .recur/docs-monkey
mode = poll
poll_framing = 5
```

Example rejected versioning state:

```text
id = care.subject.routine
state = stopped
ack = rejected
nak_reason = "operator required for proposed -> approved promotion"
subject = care.subject.routine
requested_transition = proposed -> approved
```

Example git checkpoint state:

```text
id = checkpoint.2026-05-11.001
state = complete
ack = accepted
nak_reason = ""
branch = a.0.2.8
head = c73d6f3
dirty = false
lanes = main.choco.todo.current, main.command.tests.progress.current
```

The state may live in eventness files, and stable summary/config values may
also be reflected in `.recur/config.toml` when they are useful to query across
sessions.  The TOML file should store durable policy or latest-known summary,
not bulky logs.  Detailed histories belong in eventness files where recur can
tree, find, trace, and collapse them.

The point is that core `recur` can remain pure while still answering:

- what happened?
- what was accepted?
- what was rejected?
- which hierarchy, filter, policy, or subject did the actor use?
- what should the human or agent inspect next?

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
