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
