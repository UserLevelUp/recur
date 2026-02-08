# main.command.tree.todo.trigger.event

Purpose:
- Keep repetitive start/complete checks explicit for this active TODO.
- Avoid hidden automation; humans and LLMs run these steps directly.

Trigger on start:
- `recur tree "main"`
- `recur tree "main" -d src/ --sep _`
- `recur tree "main" -d julia-tests/`
- `recur files "main.command.*.todo.current" -d docs/`
- `recur files "main_command_*_todo_current" -d src/ --sep _`

Trigger on complete:
- `cargo test --quiet`
- `julia julia-tests/runtests.jl` (run on major checkpoints or cycle boundaries)
- `recur-git checkpoint --append-parallel --checkpoint-id <id>`
- move cursor leaves to next `<command>.todo.current`
