# Dogfooding Workflow Guide: Using Recur to Track Recur Development

## The Pattern: Eventness + Recur Queries

**Core idea:** Use recur commands to discover state instead of remembering it.

## Quick Reference Commands

### What am I working on right now?
```bash
recur files "**.current" -d docs/
```
Shows all active work items (3 files currently).

### What's my reference implementation?
```bash
recur files "**.reference" -d docs/
```
Points to working examples to follow.

### What commands should I run next?
```bash
recur files "**.trigger.event" -d docs/
```
Shows trigger event files with explicit commands for each workflow stage.

### What's left to do in Phase 3?
```bash
recur files "**.stdin.todo" -d docs/
```
Shows all stdin TODO files (7 total: 6 commands + phase3 plan).

### How's overall progress?
```bash
recur tree "main.improvement.6" -d docs/
recur tree "main.command.find.stdin" -d docs/
```

## Workflow: Implementing find stdin support

### 1. Start Work (Discovery Event)

Run commands from [main.command.find.stdin.todo.trigger.event.md](main.command.find.stdin.todo.trigger.event.md):

```bash
# See what I'm working on
recur files "**.current" -d docs/

# Check reference (files command)
recur files "main_command_files_*" -d src/ --sep _
cat src/main_command_files_stdin.rs

# Check current state
recur files "main_command_find_*" -d src/ --sep _
cat src/main_command_find_impl.rs

# Check test status
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin"
```

### 2. During Work (Progress Event)

```bash
# Verify files exist
recur files "main_command_find_*" -d src/ --sep _

# Quick syntax check
cargo check

# Run targeted tests
cd julia-tests && julia -e 'include("main.command.find.test.jl")'
```

### 3. Complete Work (Validation Event)

```bash
# Full validation
cargo test --quiet
cd julia-tests && julia runtests.jl

# Verify find stdin tests pass
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin" | grep PASS

# Clean up tracking files (they're ephemeral!)
rm docs/main.command.find.stdin.todo.current.md
rm docs/main.command.find.stdin.todo.current.reference.md
rm docs/main.command.find.stdin.todo.trigger.event.md

# Move to next command (children)
# Create new current/reference/trigger files for children
```

## File Hierarchy as State

### Persistent Files (Keep)
- `main.command.<name>.readme.md` - Documentation
- `main.command.<name>.stdin.todo.md` - High-level TODO
- `main.improvement.6.dogfooding.phase3.stdin.todo.md` - Phase plan

### Ephemeral Files (Delete When Done)
- `main.command.<name>.stdin.todo.current.md` - Active work marker
- `main.command.<name>.stdin.todo.current.reference.md` - Reference pointer
- `main.command.<name>.stdin.todo.trigger.event.md` - Event commands

**The presence/absence of files IS the state!**

Query with recur to discover:
- Current work: `recur files "**.current"`
- References: `recur files "**.reference"`
- Triggers: `recur files "**.trigger.event"`

## Philosophy: External Memory Pattern

1. **Don't remember** - Store context in files
2. **Don't hide** - Make every step explicit
3. **Don't automate** - Run commands manually at events
4. **Do query** - Use recur to discover state
5. **Do clean** - Delete ephemeral files when done

## Example: Complete Workflow

```bash
# Start: Working on find stdin
recur files "**.current" -d docs/
# Output: 3 current files (improvement.6, phase3, find)

# Reference: How did files command do it?
cat src/main_command_files_stdin.rs

# Work: Create src/main_command_find_stdin.rs
# ... implement ...

# Progress: Check syntax
cargo check

# Progress: Run tests
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin"

# Complete: Validate
cargo test && cd julia-tests && julia runtests.jl

# Complete: Clean up
rm docs/main.command.find.stdin.todo.current.md
rm docs/main.command.find.stdin.todo.current.reference.md
rm docs/main.command.find.stdin.todo.trigger.event.md

# Next: Start on children
recur files "**.stdin.todo" -d docs/
# Output: Now shows 6 files (find is no longer listed)
```

## Key Insight

**This workflow IS dogfooding!**

We're using recur's hierarchical file discovery to manage recur's own development. The hierarchy structure makes state visible and queryable.

When you want to know "what should I work on next?", just run:
```bash
recur files "**.current" -d docs/
```

The answer is in the files, not in your memory!
