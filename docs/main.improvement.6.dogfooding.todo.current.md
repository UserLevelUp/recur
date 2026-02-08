# Improvement 6 Dogfooding (Current)

Status: `todo.current`

This is the active cursor for ongoing work.

## Phase 2: Command Extraction ✅ COMPLETE

All 10 commands extracted from `src/main.rs` into `src/main_command_*_impl.rs` modules:
- ✅ `main.command.stats`
- ✅ `main.command.files`
- ✅ `main.command.children`
- ✅ `main.command.related`
- ✅ `main.command.id`
- ✅ `main.command.find`
- ✅ `main.command.callers`
- ✅ `main.command.callees`
- ✅ `main.command.trace`
- ✅ `main.command.tree`

## Phase 3: Stdin Support (IN PROGRESS)

**Stdin Working (4/10):**
- ✅ files (has `main_command_files_stdin.rs`)
- ✅ stats (has `main_command_stats_stdin.rs`)
- ✅ tree
- ✅ related

**Stdin Needed (6/10):**
- ⏳ find
- ⏳ children
- ⏳ id
- ⏳ callers
- ⏳ callees
- ⏳ trace

Next focus: Add stdin support to remaining 6 commands

## Out of Scope

- ❌ `main.command.checkpoint` - Will be handled by future `recur-git` extension, NOT part of core recur

Reference:
- `README.CORE.IMPROVEMENT6.Dogfooding.md`
