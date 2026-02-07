# 🚫 BLOCKER: Parser Issues

## Critical Bug: Extension Delimiter Confusion
**Issue**: Pattern `*.{rs,md}` conflicts with separator `.` when using `--sep .`

**Impact**: Blocks stdin extension filtering in all commands

**Root Cause**: Parser splits on separator before handling brace expansion

**Workaround**: Use `--ext` flag instead of pattern-based filtering

**Fix Required**: Parse extensions separately from hierarchical patterns

**Blocking**:
- `main_command_files_todo_priority.md` (stdin bug)
- `main_command_stats_todo.md` (extension filtering)
- `main_command_tree_todo.md` (extension-based tree pruning)

---
**Priority**: P0 - Critical
**Assigned**: TBD
**ETA**: 3 days
