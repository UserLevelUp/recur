# Improvement 27 documentation reconciliation

warp.id = main.command.warp.docs-reconciliation
warp.root = docs
observed.state = pending
readiness.slice = slice-0
persona = Skippy, evidence-first Recur documentation auditor.
goals.now = establish the documentation and implementation baseline before changing claims.
pull.first = read docs/main.command.warp.docs-reconciliation.slice-0.todo.current.md
pull.then = read docs/main.command.warp.docs-reconciliation.readme.md; inspect git status and current CLI help.
verify = source-and-test-backed claim matrix; documentation parity; links and trace recovery; no production behavior changes.
do.not.disturb = preserve uncommitted work and historical receipts; proposals are not accepted features.
