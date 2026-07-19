# recur-git Artifact

Status: `implemented`

## Overview

`recur-git` is a separate Git workflow extension binary that composes recur-aware operations with Git state management.

It exists to **keep recur pure** - focused solely on hierarchical file semantics while providing Git integration capabilities as a separate tool.

## Architecture Decision

**Separation of Concerns:**

| Tool | Purpose | Dependencies |
|------|---------|--------------|
| `recur` | Hierarchical file operations | No Git dependencies |
| `recur-git` | Git-aware workflows | Uses recur as library + Git |

This follows the Unix philosophy: **do one thing well**.

## What recur-git Provides

### Checkpoint Command

Captures parallel-lane checkpoint entries combining:
- Git state (branch, head, worktree status)
- Active `current` leaves across configured lanes
- Separator configuration
- Optional test execution

**Usage:**

```bash
# Snapshot only (no side effects)
recur-git checkpoint --snapshot

# Snapshot + run tests
recur-git checkpoint --snapshot --run-tests --run-julia-tests

# Emit checkpoint entry to stdout
recur-git checkpoint --emit-parallel --checkpoint-id ck-children-01

# Append to parallel history log
recur-git checkpoint --append-parallel --checkpoint-id ck-children-01
# (or provide a file explicitly)
recur-git checkpoint --append-parallel --checkpoint-id ck-children-01 -f docs/main.dogfooding.parallel.history.md
```

## Implementation

**Source:** [src/recur_git_main.rs](src/recur_git_main.rs)

**Binary Name:** `recur-git`

**Key Features:**
- Queries current-work files using lane-aware hierarchical matching from `.recur/config.toml`
- Captures git state via `git rev-parse` and `git status`
- Formats structured checkpoint entries
- Appends to parallel history logs (`--file` or `[checkpoint].file`)
- Optionally runs cargo tests and julia tests

### Test Receipt Command

Runs one bounded Cargo or Julia target against a clean committed worktree and
writes an immutable eventness result under `.recur/tests/`. Passing targets
produce `.passed.complete.md`; failing targets produce `.failed.strange.md`
before the command exits nonzero. Checkpoints include both receipt groups.

## Build Configuration

Defined in [Cargo.toml](Cargo.toml):

```toml
[[bin]]
name = "recur-git"
path = "src/recur_git_main.rs"
```

**Build:**

```bash
cargo build --release --bin recur-git
```

**Output:** `target/release/recur-git` (or `recur-git.exe` on Windows)

## Dogfooding Workflow Integration

When checkpointing during dogfooding:

1. **Before switching lanes:**
   ```bash
   recur-git checkpoint --snapshot
   ```

2. **After completing work:**
   ```bash
   # Create Git commit
   git add -A
   git commit -m "dogfooding: complete <branch>; cursor <from> -> <to>"

   # Capture checkpoint
   recur-git checkpoint --append-parallel --checkpoint-id ck-<name>-<num>
   ```

3. **Checkpoint includes:**
   - Git commit hash
   - Active `current` files in configured lanes
   - Per-lane separator configuration
   - Worktree cleanliness

## Purity Philosophy

**Why separate binaries?**

1. **Focused scope** - recur doesn't need Git knowledge
2. **Minimal dependencies** - recur stays lightweight
3. **Clear responsibility** - hierarchy vs. workflow
4. **Easier testing** - test hierarchical logic separately from Git logic

See: [docs/main.recur.purity.decision.md](docs/main.recur.purity.decision.md)

## Verification

```bash
# Recur does NOT have checkpoint
$ recur checkpoint --snapshot
error: unrecognized subcommand 'checkpoint'

# recur-git DOES have checkpoint
$ recur-git checkpoint --snapshot
== Checkpoint Snapshot ==
git.branch: dogfooding
git.head: 57be01a docs: add practical development workflow example to agent prompt
...
```

## References

- [docs/main.git.checkpoint.readme.md](docs/main.git.checkpoint.readme.md) - Checkpoint workflow guide
- [docs/main.command.checkpoint.out-of-scope.md](docs/main.command.checkpoint.out-of-scope.md) - Why checkpoint is not in recur
- [docs/main.recur.purity.decision.md](docs/main.recur.purity.decision.md) - Separation rationale
- [src/recur_git_main.rs](src/recur_git_main.rs) - Implementation

## Future Extensions

Potential future `recur-git` capabilities:
- `recur-git diff` - Show hierarchical diff of changed files
- `recur-git blame` - Hierarchical blame view
- `recur-git log` - Hierarchical commit history

**Core principle:** Keep hierarchy semantics in recur library, compose with Git in recur-git.
