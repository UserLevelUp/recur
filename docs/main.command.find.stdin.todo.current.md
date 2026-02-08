# Command: find - Stdin Support (CURRENT WORK)

Status: `todo.current`

## Active Task

Implementing stdin support for the `find` command (content search).

## Context

This is a **content search command** that needs to:
1. Accept file paths from stdin
2. Filter paths by scope pattern
3. Search content within those specific files (not filesystem)

## Reference Implementation

See working example: [main.command.files.stdin.todo.current.reference.md](main.command.files.stdin.todo.current.reference.md)

## Files to Modify/Create

- Create: `src/main_command_find_stdin.rs`
- Modify: `src/main_command_find_impl.rs` (integrate stdin logic)
- Verify: `julia-tests/main.command.find.test.jl`
- Verify: `julia-tests/main.command.stdin.test.jl`

## Trigger Events

See: [main.command.find.stdin.todo.trigger.event.md](main.command.find.stdin.todo.trigger.event.md)
