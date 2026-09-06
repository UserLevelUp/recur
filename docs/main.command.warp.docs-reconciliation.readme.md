# Warp: Improvement 27 documentation reconciliation

Created: 2026-09-05
State: planned; Slice 0 current; no slices accepted.
Warp identity: main.command.warp.docs-reconciliation

## Goal

Make root and supporting Improvement 27 docs accurately distinguish implemented
Warp behavior, partial capabilities, optional proposals and historical decisions.
This is documentation reconciliation, not implementation of every proposed feature.

## Sequence

- Slice 0: Capture current documentation and implementation baseline.
- Slice 1: Build a claim-by-claim reconciliation matrix.
- Slice 2: Reconcile root Improvement 27 documents.
- Slice 3: Reconcile supporting notes and recovery surfaces.
- Slice final: Verify the reconciled documentation and close the Warp.

## Source inventory

- README.CORE.IMPROVEMENT27.md
- README.CORE.IMPROVEMENT27.Appendum.md
- docs/main.improvement.27.todo.future-plan.md
- docs/main.improvement.27.recur-ready.todo.future-plan.md
- docs/main.improvement.27.command-boundary.todo.future-plan.md
- docs/main.improvement.27.contract.warp-status-v1.todo.future-plan.md
- docs/main.improvement.27.epic.milestone.todo.future-plan.md
- docs/main.improvement.27.differential-execution.todo.current.md
- Root .recur-warp capability card, if present
- docs/main.command.warp.readme.md and docs/main.command.warp.evidence.readme.md
- Current Rust command/schema implementations and focused Julia/Rust tests

Improvement 29 and Improvement 30 are boundary references, not implementation scope.
The working tree already contains uncommitted project-discovery and administrative
Warp 10 closure changes. Preserve them; do not rewrite historical verification
records or treat their presence as proof of publication.

## Rules and non-goals

- Do not add scoring, scheduling, temporal forecasting, agents, or a recur-reveal executable.
- Do not convert methodology examples into live project backlog.
- Do not mark partially implemented proposal fields complete.
- Preserve historical intent with explicit status/date labels and references.
- Keep production behavior unchanged. Documentation/example tests may be adjusted only
  when needed to verify an actual documentation change.
- Do not commit, push, install, or mutate private capsules as part of creating this plan.
- Contract identifiers here are opaque versioned strings, not cryptographic hashes.
  Evidence gates use declared references to reviewed records, not fake checked receipts.

## Recovery commands

```powershell
recur reveal main.command.warp.docs-reconciliation
recur warp map main.command.warp.docs-reconciliation -d docs --json
recur warp merge main.command.warp.docs-reconciliation -d docs --json
recur files "main.command.warp.docs-reconciliation.**.current" -d docs
recur trace-id "recur.warp.docs.reconciliation.**" --scope "main.command.warp.docs-reconciliation.**" -d docs --ext .md --format full
```

Use target/release-safe/recur.exe if the installed executable does not match the
working source. Creating this plan does not complete any audit or gate.

defines: recur.warp.docs.reconciliation.contract root and supporting Warp docs match evidence without promoting proposals
