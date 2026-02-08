# main.command.checkpoint.todo.next

Status: ready-next

Next queue for checkpoint lane:
- preserve checkpoint extension behavior in `src/recur_git_main.rs`
- execute recurring completion checklist before lane rotation
- append checkpoint record and move `main_command_*_todo_current` to the next lane

Source-side checks:
- `recur files "main_command_checkpoint_*" -d src/ --sep _`
- `recur files "main_command_*_todo_current" -d src/ --sep _`
