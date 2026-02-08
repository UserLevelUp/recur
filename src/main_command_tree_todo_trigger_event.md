# main.command.tree.todo.trigger.event

Manual trigger checklist for current tree extraction lane.

Start:
- `recur tree "main" -d src/ --sep _`
- `recur files "main_command_*_todo_current" -d src/ --sep _`

Complete:
- `cargo test --quiet`
- `recur-git checkpoint --append-parallel --checkpoint-id <id>`
- rotate `main_command_*_todo_current` to next command lane
