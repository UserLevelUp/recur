# main.command.checkpoint.todo.current

Active cursor:
- Align checkpoint workflow dogfooding branches across `src/`, `docs/`, and `julia-tests/`.
- Preserve shell-composed checkpoint workflow behavior while closing naming/coverage gaps.
- Follow trigger checklist: `src/main_command_checkpoint_todo_trigger_event.md`.
- Keep next queue in `src/main_command_checkpoint_todo_next.md`.

Definition of done:
- `docs/main.command.checkpoint.readme.md` exists.
- `recur-git checkpoint --snapshot` runs from repo root.
- `recur tree "main"` and `recur tree "main" -d src/ --sep _` show coherent checkpoint branches.
