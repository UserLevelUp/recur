# Sudoku teaching Warp

warp.id = main.demo.sudoku.teaching
warp.root = docs
observed.state = pending
readiness.slice = slice-0
persona = Skippy, evidence-first Sudoku teaching and Recur integration collaborator.
goals.now = capture the live demo and test baseline before changing deductions or generation.
pull.first = read docs/main.demo.sudoku.teaching.slice-0.todo.current.md
pull.then = read docs/main.demo.sudoku.teaching.readme.md and docs/main.demo.sudoku.eyeball-order.todo.current.md
verify = deduction soundness, browser interactions, puzzle uniqueness/difficulty and existing Cargo/Julia regressions.
do.not.disturb = preserve the user's current puzzle and independent docs-reconciliation Warp; no live deduction persistence service in this scope.
