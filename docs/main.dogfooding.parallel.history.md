# main.dogfooding.parallel.history

Purpose:
- Record progression in parallel lanes so state and commits stay synchronized.
- Make session handoff easier with one checkpoint ID per major transition.

Parallel lanes:
- `lane.state`: leaf and cursor transitions (`todo.current`, branch completion).
- `lane.git`: branch, HEAD commit, and dirty/clean status at checkpoint time.
- `lane.separator`: active delimiter policy (`docs`/`tests` vs `src`).

Checkpoint template:
```markdown
### ck-YYYYMMDD-HHMMSS
- date: YYYY-MM-DD HH:MM:SS +/-TZ
- lane.state.docs.current: docs/main.command.<name>.todo.current.md
- lane.state.src.current: src/main_command_<name>_todo_current.md
- lane.git.branch: <branch>
- lane.git.head: <short-hash> <subject>
- lane.git.worktree: dirty=<n> (or clean)
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence.docs_tree_cmd: recur tree "main" -d docs/
- evidence.src_tree_cmd: recur tree "main" -d src/ --sep _
```

## Checkpoints

### ck-20260216-session-close
- date: 2026-02-16 18:12:36 -08:00
- lane.state.docs.current: `docs/main.improvement.7.phase3.todo.current.md` (active, paused for day)
- lane.state.docs.next: `sort-by risk` activation lane
- lane.state.docs.patch: `docs/main.command.trace-stats.metrics.todo.current.md`
- lane.state.tests.patch: `julia-tests/runtests.trace-stats.jl`
- lane.state.src.current: `src/main_command_trace_stats_impl.rs` (no source change in close checkpoint)
- lane.git.branch: `flatten-init`
- lane.git.head: `d55d19b Add traits.stdin policy and shared stdin path resolver`
- lane.git.worktree: `dirty=72`
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main.improvement.7.phase3" -d docs/`
- evidence.tests_cmd: `julia julia-tests/main.command.trace-stats.test.jl` (`69 pass`, `7 broken`)

### ck-20260216-improvement7-phase3-sort-depth-activated
- date: 2026-02-16 17:27:07 -08:00
- lane.state.docs.current: `docs/main.improvement.7.phase3.todo.current.md`
- lane.state.docs.patch: `docs/main.command.trace-stats.metrics.todo.current.md`
- lane.state.tests.patch: `julia-tests/runtests.trace-stats.jl`
- lane.state.src.current: `src/main_command_trace_stats_impl.rs` (no source change in this checkpoint)
- lane.git.branch: `flatten-init`
- lane.git.head: `d55d19b Add traits.stdin policy and shared stdin path resolver`
- lane.git.worktree: `dirty=72`
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main.improvement.7.phase3" -d docs/`
- evidence.tests_cmd: `julia julia-tests/main.command.trace-stats.test.jl` (`69 pass`, `7 broken`)

### ck-20260216-improvement7-phase3-eventness-refresh
- date: 2026-02-16 15:25:34 -08:00
- lane.state.docs.current: `docs/main.improvement.7.phase3.todo.current.md`
- lane.state.docs.patch: `docs/main.command.trace-stats.metrics.todo.current.md`
- lane.state.src.current: `src/main_command_trace_stats_impl.rs` (no source change in this checkpoint)
- lane.git.branch: `flatten-init`
- lane.git.head: `d55d19b Add traits.stdin policy and shared stdin path resolver`
- lane.git.worktree: `dirty=71`
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main.improvement.7.phase3" -d docs/`
- evidence.tests_cmd: `julia julia-tests/main.command.trace-stats.test.jl` (`66 pass`, `8 broken`)

### ck-20260216-improvement15-parked
- date: 2026-02-16 15:23:52 -08:00
- lane.state.docs.current: `docs/main.improvement.7.phase3.todo.current.md` (active)
- lane.state.docs.parked: `docs/main.improvement.15.todo.future-plan.md` (long-distance backlog)
- lane.state.src.current: `n/a (no implementation lane opened for improvement 15)`
- lane.git.branch: `flatten-init`
- lane.git.head: `d55d19b Add traits.stdin policy and shared stdin path resolver`
- lane.git.worktree: `dirty=67`
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main.improvement" -d docs/`
- evidence.policy: `README.CORE.IMPROVEMENT15.md`, `docs/main.improvement.15.todo.future-plan.md`

### ck-20260216-improvement15-contract-freeze
- date: 2026-02-16 15:10:46 -08:00
- lane.state.docs.current: `docs/main.improvement.15.todo.future-plan.md`
- lane.state.src.current: `n/a (docs/tests contract freeze checkpoint)`
- lane.git.branch: `flatten-init`
- lane.git.head: `d55d19b Add traits.stdin policy and shared stdin path resolver`
- lane.git.worktree: `dirty=67`
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main.improvement.15" -d docs/`
- evidence.tests_cmd: `julia julia-tests/main.command.unflatten.test.jl`

### ck-20260207-bootstrap
- date: 2026-02-07
- lane.state.docs.current: `docs/main.command.children.todo.current.md`
- lane.state.src.current: `src/main_command_children_todo_current.md`
- lane.git.branch: `dogfooding`
- lane.git.head: `67a6075 docs: clarify dogfooding and add self-reflective improvement status tree`
- lane.git.worktree: dirty
- lane.separator.docs_tests: `.`
- lane.separator.src: `_`
- evidence.docs_tree_cmd: `recur tree "main" -d docs/`
- evidence.src_tree_cmd: `recur tree "main" -d src/ --sep _`

### ck-20260207-children-complete
- date: unix:1770503852
- lane.state.docs.current: docs\main.command.related.todo.current.md
- lane.state.src.current: src\main_command_related_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 67a6075 docs: clarify dogfooding and add self-reflective improvement status tree
- lane.git.worktree: dirty=29
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-related-complete
- date: unix:1770504208
- lane.state.docs.current: docs\main.command.id.todo.current.md
- lane.state.src.current: src\main_command_id_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=8
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-id-complete
- date: unix:1770504294
- lane.state.docs.current: docs\main.command.find.todo.current.md
- lane.state.src.current: src\main_command_find_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=10
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-find-complete
- date: unix:1770504379
- lane.state.docs.current: docs\main.command.callers.todo.current.md
- lane.state.src.current: src\main_command_callers_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=11
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-callers-complete
- date: unix:1770505481
- lane.state.docs.current: docs\main.command.callees.todo.current.md
- lane.state.src.current: src\main_command_callees_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=16
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-callees-complete
- date: unix:1770505691
- lane.state.docs.current: docs\main.command.trace.todo.current.md
- lane.state.src.current: src\main_command_trace_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=17
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260207-trace-complete
- date: unix:1770506012
- lane.state.docs.current: docs\main.command.tree.todo.current.md
- lane.state.src.current: src\main_command_tree_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=18
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260208-tree-complete
- date: unix:1770511634
- lane.state.docs.current: docs\main.command.tree.todo.current.md
- lane.state.src.current: src\main_command_tree_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=19
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _

### ck-20260208-checkpoint-lane-start
- date: unix:1770511794
- lane.state.docs.current: docs\main.command.checkpoint.todo.current.md
- lane.state.src.current: src\main_command_checkpoint_todo_current.md
- lane.git.branch: dogfooding
- lane.git.head: 44f435f dogfooding: complete main.command.children; cursor children -> related
- lane.git.worktree: dirty=21
- lane.separator.docs_tests: .
- lane.separator.src: _
- evidence:
- recur tree "main" -d docs/
- recur tree "main" -d src/ --sep _
