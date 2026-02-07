# main.command.related.todo.current

Active cursor:
- Extract `cmd_related(...)` from `src/main.rs` into `src/main_command_related_impl.rs`.

Definition of done:
- `Commands::Related` dispatches to `main_command_related_impl::execute(...)`.
- Existing related-command behavior is unchanged.
- `cargo test` remains green.
