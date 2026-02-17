# Improvement Status Index

This branch makes improvement status visible in `recur tree "main"`.

Naming:
- `main.improvement.<n>.complete.md` = completed improvement
- `main.improvement.<n>.todo.md` = active TODO
- `main.improvement.<n>.todo.future-plan.md` = TODO planned for future
- `main.improvement.<n>.<topic>.todo.current.md` = active cursor (single current focus)

Current status:
- Complete: `1`, `2`, `3`, `4`, `5`
- TODO now: `6`, `6.dogfooding`
- TODO future plan: `7`, `8`, `9`, `14`, `15`
- Current cursor: `6.dogfooding.todo.current`

Notes:
- `main.improvement.15.todo.future-plan.md` is long-distance backlog only (parked, not active).
