# main.command.checkpoint.todo.next

Status: ready-next

Next queue for `main.command.checkpoint`:
- run recurring completion checklist in `docs/main.command.checkpoint.todo.trigger.event.md`
- append a checkpoint entry (`recur-git checkpoint --append-parallel --checkpoint-id <id>`)
- rotate active cursor to the next lane after checkpoint closure

Quick verification from repo root:
- `recur files "main.command.checkpoint.readme" -d docs/`
- `recur-git checkpoint --emit-parallel --checkpoint-id ck-dry-run`
- `recur files "main.command.checkpoint.**" -d docs/`
