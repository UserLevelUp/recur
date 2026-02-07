# main.command.related.todo.current

Status: active

Current focus:
- Move `cmd_related(...)` out of `src/main.rs` into `src/main_command_related_impl.rs`.

Exit criteria:
- `Commands::Related` dispatches to module implementation.
- Existing behavior stays stable.
- Tests pass.
