# main.dogfooding.history

Purpose:
- Keep a compact, append-only timeline of dogfooding progress.
- Reduce context reconstruction for humans and LLMs between sessions.

Entry format:
- `date` (YYYY-MM-DD)
- `area` (command or workflow branch)
- `change`
- `state transition`
- `evidence` (file path or command)
- `commit` (short hash + subject, if committed)

Template for new entries:
```markdown
### YYYY-MM-DD
- area: `main.command.<name>` or workflow topic
- change: short description of what changed
- state transition: leaf/status movement (`todo.current` -> removed, etc.)
- evidence: path(s) or exact `recur` command used to validate
- commit: `abc1234` short commit subject (optional until committed)
```

## Log

### 2026-02-07
- area: `main.command.children`
- change: extracted children execution from `src/main.rs` into `src/main_command_children_impl.rs`.
- state transition: removed `children.todo.current` leaves and moved cursor to `related.todo.current`.
- evidence: `src/main_command_children_impl.rs`, `src/main.rs`, `docs/main.command.related.todo.current.md`, `src/main_command_related_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: `main.command.stats`
- change: extracted stats execution from `src/main.rs` into dedicated modules.
- state transition: `stats.todo` removed from docs; branch now represented by `impl` + `stdin`.
- evidence: `src/main_command_stats_impl.rs`, `src/main_command_stats_stdin.rs`.

### 2026-02-07
- area: separator workflow
- change: CLI now accepts repeated `--sep`; last value wins.
- state transition: separator switches can be passed inline during ad hoc queries.
- evidence: `src/main.rs` (`sep: Vec<String>`), command: `recur tree main -d src --sep "." --sep "_"`.

### 2026-02-07
- area: `main.command.files`
- change: extracted files command from `src/main.rs` into `main_command_files_impl/stdin`.
- state transition: removed `files.todo*` leaves after completion.
- evidence: `src/main_command_files_impl.rs`, `src/main_command_files_stdin.rs`, `src/main.rs`.

### 2026-02-07
- area: active cursor
- change: moved cursor from `files` to `children`.
- state transition: added `children.todo.current` leaves in `src/` and `docs/`.
- evidence: `src/main_command_children_todo_current.md`, `docs/main.command.children.todo.current.md`.

### 2026-02-07
- area: checkpoint workflow
- change: added optional built-in `recur checkpoint` command with explicit flags.
- state transition: logging is now opt-in via `--emit-parallel` / `--append-parallel` (no default side effects).
- evidence: `src/main_command_checkpoint_impl.rs`, `src/main.rs`, `recur checkpoint --help`.
