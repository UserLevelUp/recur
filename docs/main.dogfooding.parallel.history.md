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
