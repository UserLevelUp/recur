# Command: callees - Stdin Support TODO

Status: `todo`

## Task

Add stdin support to the `callees` command.

## Current State

- ✅ Has `src/main_command_callees_impl.rs`
- ❌ Missing `src/main_command_callees_stdin.rs`
- ❌ Stdin tests failing

## Pattern

This is a **content search command**, so it needs to:
1. Accept file paths from stdin
2. Filter paths by scope pattern
3. Search for function callees within those specific files

## References

- Working example: `src/main_command_find_stdin.rs` (once implemented)
- Test file: `julia-tests/main.command.callees.test.jl`
- Stdin tests: `julia-tests/main.command.stdin.test.jl`
