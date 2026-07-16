# main.improvement.26.todo.future-plan

Status: `future-plan`
Date: 2026-05-03

## Purpose

Make Improvement 26 discoverable from the docs-side improvement eventness tree.

## Canonical Proposal

- `README.CORE.IMPROVEMENT26.md`

## Summary

Improvement 26 proposes companion governance tools around pure `recur`:

- `recur-watch` detects versioning-potential and proposes approval/version lanes
  when an artifact becomes operational, repeated, consequential, or dependent on
  human/operator approval.
- `recur-version` handles write-side artifact versioning: next-version
  selection, snapshot saves, manifest updates, privacy policy enforcement, and
  configured checkpoint behavior.
- `recur version` is the read/query surface inside pure `recur`: status,
  manifest inspection, policy/schema exposure, diffs, queryable history, and
  evidence-backed explanations over preserved artifact versions.
- `recur-trace` traces lineage, responsibility boundaries, provenance, and the
  approval or promotion process that led to an artifact's current state.

The core boundary is that `recur` remains the hierarchy and eventness truth
surface, while companion tools interpret when the tree is asking for a new
branch.

The proposal now also defines a privacy-preserving fixture rule: public tests
should use synthetic `care.subject.*` examples that preserve the versioning,
approval, ambiguity, and operator-handoff mechanics without copying real names,
contacts, medication names, dose/timing details, or private-root records.

The proposal now also calls out the stronger versioning goal: make history
queryable and precise while keeping the tool generic. Domain specificity should
come from `.recur/config.toml` definitions for identity fields, tracked fields,
states, transitions, and risk words rather than hardcoded topic knowledge.

It also clarifies the generic-engine boundary: Recur should not hardcode a
specific domain. `.recur/config.toml` should declare artifact semantics and
persona exposure rules, and `recur version` / `recur-trace` should surface those
rules to revealable personas during versioning and lineage work.

## 0.2.8 Slice

The first shipped slice now covers:

- `recur version status`, `manifest`, `policy`, `schema`, `query`, `explain`
- `recur-version next`
- `recur-version save`
- ACK/NAK status records under `.recur/version/`
- fixture tests in `julia-tests/main.command.version.test.jl`

The broader governance criteria below remain future work.

## Test Todo

Add synthetic fixture tests that cover the full Improvement 26 criteria:

- `docs/main.improvement.26.test.todo.criteria-coverage.md`

- versioning-potential detection
- config-driven version policy
- manifest updates and next-version selection
- queryable history with evidence-backed answers
- generic domain semantics loaded from `.recur/config.toml`
- persona exposure of artifact policy/schema
- ambiguous referent handling
- high-risk transition confirmation
- proposed-to-approved operator authorization
- item-level approval lanes
- privacy-preserving fixture enforcement
- command behavior for `recur-watch`, `recur version`, and `recur-trace`

## Discovery

```powershell
recur files "main.improvement.26.**" -d docs/
recur tree "main.improvement.26" -d docs/
recur files "README.CORE.IMPROVEMENT26" -d . --sep .
```
