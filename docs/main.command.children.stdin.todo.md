# Command: children - Stdin Support TODO

Status: `todo`

## Task

Add stdin support to the `children` command.

## Current State

- ✅ Has `src/main_command_children_impl.rs`
- ❌ Missing `src/main_command_children_stdin.rs`
- ❌ Stdin tests failing

## Pattern

This is a **standard command** that lists files, so it needs to:
1. Accept file paths from stdin
2. Filter paths by hierarchical pattern
3. Return matching immediate children

## References

- Working example: `src/main_command_files_stdin.rs`
- Test file: `julia-tests/main.command.children.test.jl`
- Stdin tests: `julia-tests/main.command.stdin.test.jl`
