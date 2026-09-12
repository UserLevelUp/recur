# Checked Lang transition

Implementation and verification: see the [current handoff](main.lang.checked-transition.current.md)
and query live progress with `recur warp show main.lang.checked-transition -d .`.
Read [usage and recovery](../docs/main.command.lang.checked-transition.readme.md)
for the implemented local binaries. This recorded page does not authorize a
transition in another project.

Connect an exact WIR1 specification to current checked external test evidence
and an explicitly confirmed `recur-lang` E0-to-Ef transition. The production
assessment belongs in a shared pure Rust module, using the existing external
evidence checker. Julia and HTTP are not dependencies of this CLI deliverable.

This is a separate bubble from
[runtime-evidence](main.lang.runtime-evidence.readme.md), not its successor or
completion. That bubble's two accepted slices, unaccepted Julia loader, crash
logs and remaining inspector/browser gates stay intact. No accepted layer is
carried across: historical observations are context, not new acceptance.

## Desired behavior

One selected function keeps its exact contracts, aliases and dependency edges.
An external runner produces explicit case results and fingerprints of actual
inputs. A pure assessment explains relevance and freshness. Only a confirmed
companion action may advance its exact E0 artifact, and only when the new
checked contract is satisfied. Missing, wrong-scope, failed or stale evidence
leaves E0 unchanged. A desired Ef or legacy ACK does not establish checked tests.

Keep Improvement 30's anatomy:

- Header: exact input/output contracts, qualified symbols and behavioral cases.
- Body: compact transformations, canonical aliases and visible boundaries.
- Footer: intended E0/dE/Ef, recorded state, static findings, historical tests,
  current evidence assessment and authorized transition as separate facts.

## Dependency order

| Slice | Deliverable | Gates |
| --- | --- | --- |
| slice-0 | Freeze the shared assessment/transition contract and test matrix | contract-and-baseline (declared) |
| slice-1 | Pure Rust assessment exposed through a bounded Lang query surface | assessment-and-query (checked tests) |
| slice-2 | Explicit checked-mode companion transition with preserved legacy behavior | confirmed-transition (checked tests) |
| slice-final | Drift, restart and interrupted-write recovery; final regression | drift-and-recovery, regression-and-reentry (checked tests) |

All implementation slices are tests-first. Preserve red observations separately;
failed or zero-test results never satisfy a checked gate. The complete gates and
non-goals are in [the contract](main.lang.checked-transition.contract.md).

## Evidence root and recovery

The single live map is `main.lang.checked-transition.warp-map.json` at the
repository root. The writer created its scaffold under configured `warps/`,
then it was relocated with its generated UUIDs preserved. Global creation
defaults are unchanged. Documentation stays here; future immutable results go
under `warps/checked-transition/`. Paths in checked evidence name actual inputs
under the map root; neither parent traversal nor copied-source substitution is
allowed. Slice-0 must exercise the layout with checker and live projection.

```powershell
recur reveal main.lang.checked-transition -d warps
recur warp show main.lang.checked-transition -d . --json
recur warp slices main.lang.checked-transition -d . --json
recur tree main.lang.checked-transition -d warps --sep .
recur files "main.lang.checked-transition.**" -d warps --sep .
recur trace-id "main.lang.checked-transition.**" --scope "**" -d warps --format full
```

Read the [current handoff](main.lang.checked-transition.current.md), preserving
[slice-0](main.lang.checked-transition.slice-0.todo.current.md) as historical
planning. Refine through source trace, recorded context recovery and scoped Lang.
Check selected executable help; the installed binaries may lag the local build.
No recall command is exposed by the inspected help. Reveal and bounded recovery
packets provide pointers; inspect truncation and full-root progress separately.

defines: main.lang.checked-transition bounded checked evidence to transition
consumes: main.lang.runtime-evidence.runtime-blocker recorded runtime limitation
consumes: recur.lang.query.v1 pure scoped WIR1 contracts
consumes: recur.warp.evidence.integrity.external existing evidence checks
produces: main.lang.checked-transition.acceptance qualified CLI acceptance
