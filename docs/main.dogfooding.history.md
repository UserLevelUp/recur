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

### 2026-02-16
- area: `main.improvement.7.phase3` session close
- change: closed the day after activating and validating `sort-by depth`; left resume point explicitly on `sort-by risk`.
- state transition: no phase transition; phase 3 remains active and paused at end-of-day.
- evidence: `docs/main.improvement.7.phase3.todo.current.md`, `docs/main.command.trace-stats.metrics.todo.current.md`, command: `julia julia-tests/main.command.trace-stats.test.jl` (`69 pass`, `7 broken`).
- commit: pending

### 2026-02-16
- area: `main.improvement.7.phase3` / `main.command.trace-stats` sub-lane `sort-by depth`
- change: activated `sort-by depth` ordering assertion in `julia-tests/runtests.trace-stats.jl` and validated it passing in the phase3 test module.
- state transition: placeholder lane for depth ordering moved from skipped to active+passing; phase 3 remains active with risk ordering and other lanes still pending.
- evidence: `julia-tests/runtests.trace-stats.jl`, `docs/main.improvement.7.phase3.todo.current.md`, `docs/main.command.trace-stats.metrics.todo.current.md`, command: `julia julia-tests/main.command.trace-stats.test.jl` (`69 pass`, `7 broken`).
- commit: pending

### 2026-02-16
- area: `main.improvement.7.phase3` / `main.command.trace-stats`
- change: refreshed phase3 eventness with a concrete validation snapshot and aligned remaining-work lanes to current placeholder tests.
- state transition: no phase movement; phase 3 remains active with metrics hardening as current focus.
- evidence: `docs/main.improvement.7.phase3.todo.current.md`, `docs/main.command.trace-stats.metrics.todo.current.md`, `docs/main.improvement.7.phase3.todo.current.reference.md`, `docs/main.improvement.7.todo.future-plan.md`, command: `julia julia-tests/main.command.trace-stats.test.jl` (`66 pass`, `8 broken`).
- commit: pending

### 2026-02-16
- area: `main.improvement.15` lane policy
- change: explicitly parked Improvement 15 as long-distance `todo.future-plan` and clarified that active implementation remains Improvement 7 Phase 3.
- state transition: no active lane opened for improvement 15; status remains future backlog only.
- evidence: `README.CORE.IMPROVEMENT15.md`, `docs/main.improvement.15.todo.future-plan.md`, `docs/main.improvement.readme.md`.
- commit: pending

### 2026-02-16
- area: `main.improvement.15` / `main.command.unflatten`
- change: froze Improvement 15 v1 contracts and added expected-broken Julia tests to lock the future command surface before implementation.
- state transition: `improvement.15` moved to explicit contract-frozen state; phase A marked complete, phase C left as planned implementation.
- evidence: `docs/main.improvement.15.contract.flat-record.v1.md`, `docs/main.command.merge.flat-format.contract.v1.md`, `docs/main.command.unflatten.contract.v1.md`, `docs/main.improvement.15.todo.future-plan.md`, `julia-tests/main.command.unflatten.test.jl`, `julia-tests/runtests.unflatten.jl`, command: `julia julia-tests/main.command.unflatten.test.jl` (16 broken expected).
- commit: pending

### 2026-02-07
- area: `main.command.trace`
- change: extracted trace execution from `src/main.rs` into `src/main_command_trace_impl.rs`.
- state transition: removed `trace.todo.current` leaves and moved cursor to `tree.todo.current`.
- evidence: `src/main_command_trace_impl.rs`, `src/main.rs`, `docs/main.command.tree.todo.current.md`, `src/main_command_tree_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: `main.command.callees`
- change: extracted callees execution from `src/main.rs` into `src/main_command_callees_impl.rs`.
- state transition: removed `callees.todo.current` leaves and moved cursor to `trace.todo.current`.
- evidence: `src/main_command_callees_impl.rs`, `src/main.rs`, `docs/main.command.trace.todo.current.md`, `src/main_command_trace_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: `main.command.callers`
- change: extracted callers execution from `src/main.rs` into `src/main_command_callers_impl.rs`.
- state transition: removed `callers.todo.current` leaves and moved cursor to `callees.todo.current`.
- evidence: `src/main_command_callers_impl.rs`, `src/main.rs`, `docs/main.command.callees.todo.current.md`, `src/main_command_callees_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: workflow trigger convention
- change: added manual `todo.trigger.event` leaves for the active callers lane and documented start/complete trigger steps.
- state transition: no cursor move; `callers.todo.current` remains active with explicit trigger checklist files.
- evidence: `docs/main.command.callers.todo.trigger.event.md`, `src/main_command_callers_todo_trigger_event.md`, `docs/main.dogfooding.readme.md`.
- commit: pending

### 2026-02-07
- area: `main.command.find`
- change: extracted find execution from `src/main.rs` into `src/main_command_find_impl.rs`.
- state transition: removed `find.todo.current` leaves and moved cursor to `callers.todo.current`.
- evidence: `src/main_command_find_impl.rs`, `src/main.rs`, `docs/main.command.callers.todo.current.md`, `src/main_command_callers_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: `main.command.id`
- change: extracted id execution from `src/main.rs` into `src/main_command_id_impl.rs`.
- state transition: removed `id.todo.current` leaves and moved cursor to `find.todo.current`.
- evidence: `src/main_command_id_impl.rs`, `src/main.rs`, `docs/main.command.find.todo.current.md`, `src/main_command_find_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

### 2026-02-07
- area: `main.command.related`
- change: extracted related execution from `src/main.rs` into `src/main_command_related_impl.rs`.
- state transition: removed `related.todo.current` leaves and moved cursor to `id.todo.current`.
- evidence: `src/main_command_related_impl.rs`, `src/main.rs`, `docs/main.command.id.todo.current.md`, `src/main_command_id_todo_current.md`, `recur tree "main"`, `recur tree "main" -d src/ --sep _`.
- commit: pending

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

### 2026-02-08
- area: checkpoint workflow
- change: removed built-in `recur checkpoint` command; switched to shell-composed workflow (`recur` + `git` + PowerShell wrapper).
- state transition: recur CLI is hierarchy-only; checkpoint orchestration moved to `scripts/dogfooding_checkpoint.ps1`.
- evidence: `src/main.rs`, `scripts/dogfooding_checkpoint.ps1`, `docs/main.git.checkpoint.readme.md`.
