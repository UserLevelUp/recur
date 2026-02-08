# Trigger Events: find stdin implementation

Purpose:
- Explicit commands to discover context and verify progress
- Run these at key moments (start of work, checkpoints, completion)
- Avoid hidden automation; humans and LLMs run these steps directly

## Trigger on Start

Discover current state and context:

```bash
# 1. See what we're working on
recur files "**.todo.current" -d docs/

# 2. Check reference implementation
recur files "main_command_files_*" -d src/ --sep _
cat src/main_command_files_stdin.rs

# 3. Check current find implementation
recur files "main_command_find_*" -d src/ --sep _
cat src/main_command_find_impl.rs

# 4. Check test status
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin"

# 5. View stdin TODO structure
recur tree "main.command.find.stdin" -d docs/
```

## Trigger During Work

Check progress:

```bash
# 1. Verify files exist
recur files "main_command_find_*" -d src/ --sep _

# 2. Run quick syntax check
cd .. && cargo check

# 3. Run targeted tests
cd julia-tests && julia -e 'include("main.command.find.test.jl")'
```

## Trigger on Complete

Validate implementation:

```bash
# 1. Run all Rust tests
cargo test --quiet

# 2. Run full Julia test suite
cd julia-tests && julia runtests.jl

# 3. Verify find stdin tests pass
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin" | grep PASS

# 4. Update status files
#    - Remove main.command.find.stdin.todo.current.md
#    - Remove main.command.find.stdin.todo.current.reference.md
#    - Remove main.command.find.stdin.todo.trigger.event.md
#    - Mark find as complete in phase3 tracking

# 5. Move to next command
#    - Create main.command.children.stdin.todo.current.md
#    - Update phase3 todo with progress
```

## Discovery Commands (Eventness Pattern)

Use recur to discover what's interesting:

```bash
# What am I working on right now?
recur files "**.current" -d docs/

# What's my reference?
recur files "**.reference" -d docs/

# What triggers exist?
recur files "**.trigger.event" -d docs/

# What's left in phase 3?
recur files "**.stdin.todo" -d docs/

# How many stdin commands work?
recur files "main_command_*_stdin" -d src/ --sep _ --count

# Overall improvement status
recur tree "main.improvement.6" -d docs/
```

## Eventness Philosophy

**Event = Key moment in workflow**
- Starting work
- Hitting a checkpoint
- Completing a task
- Getting blocked
- Switching context

**Response = Explicit command to run**
- Discover state with recur queries
- Validate with tests
- Update tracking files

**No hidden automation** - make every step visible and queryable!
