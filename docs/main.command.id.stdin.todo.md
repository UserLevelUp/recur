# Command: id - Stdin Support TODO

Status: `todo`

## Task

Add stdin support to the `id` command (identifier search).

## Current State

- ✅ Has `src/main_command_id_impl.rs`
- ❌ Missing `src/main_command_id_stdin.rs`
- ❌ Stdin tests failing

## Pattern

This is a **content search command**, so it needs to:
1. Accept file paths from stdin
2. Filter paths by scope pattern
3. Search for identifiers within those specific files

## References

- Working example: `src/main_command_find_stdin.rs` (once implemented)
- Test file: `julia-tests/main.command.id.test.jl`
- Stdin tests: `julia-tests/main.command.stdin.test.jl`
