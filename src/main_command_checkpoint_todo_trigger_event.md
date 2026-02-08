# main.command.checkpoint.todo.trigger.event

Manual recurring trigger checklist for current checkpoint lane.

Start:
- `recur tree "main" -d src/ --sep _`
- `recur files "main_command_*_todo_current" -d src/ --sep _`

Complete:
- `cargo test --quiet`
- update docs/history for the lane
- `git commit` with command-specific summary
- `git push origin <branch>`
- `recur-git checkpoint --append-parallel --checkpoint-id <id>`
- rotate `main_command_*_todo_current` to next lane
