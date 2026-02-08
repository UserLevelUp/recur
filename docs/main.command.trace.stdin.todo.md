# Command: trace - Stdin Support TODO

Status: `todo`

## Task

Add stdin support to the `trace` command (transitive call graph).

## Current State

- ✅ Has `src/main_command_trace_impl.rs`
- ❌ Missing `src/main_command_trace_stdin.rs`
- ❌ Stdin tests failing

## Pattern

This is a **content search command** with special call graph logic, so it needs to:
1. Accept file paths from stdin
2. Filter paths by scope pattern
3. Build call graph from those specific files only

## References

- Working example: `src/main_command_find_stdin.rs` (once implemented)
- Test file: `julia-tests/main.command.trace.test.jl`
- Stdin tests: `julia-tests/main.command.stdin.test.jl`
