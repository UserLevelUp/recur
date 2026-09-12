# Lang runtime evidence: precise specifications from E0 to Ef

Date: 2026-09-12. Status: planned; implementation has not started.
Release target: a.0.2.8, not authorization to publish or install anything.

This is Option A from the Lang readiness assessment. It connects the compact
specification to observed tests and implementation evidence, rather than adding
another grammar or starting the live coordination grid. Its governing intent is
[Improvement 30](../README.CORE.IMPROVEMENT30.md): Lang provides the highest local
level of specificity while the surrounding project remains loosely structured.

## Refine only as far as the work needs

| Step | Purpose |
| --- | --- |
| `recur tree` | See the hierarchy and where attention is concentrated. |
| `recur files` | Select the bounded artifacts worth reading. |
| `recur trace-id` | Follow explicitly declared definitions, producers, consumers and triggers. |
| `recur trace`, `callers`, `callees` | Inspect source-level relationships; these are not observed runtime traces. |
| Recall/context recovery | Recover recorded intent, decisions and next actions. Use `recur recall` only if the selected executable supports it; current help does not advertise it. Today use `recur reveal` and read the returned artifacts. |
| `recur lang` | Inspect exact scoped contracts, functional connections and qualified evidence. |

This is a progressive refinement workflow, not mandatory ceremony for every
query. Unrecorded rationale remains unknown. A source trace cannot supply it.

## Specification and lifecycle

- Header: exact input/output bundles, scoped symbols, meanings, constraints and
  linked behavioral requirements. Expand details without changing identities.
- Body: compact `i(a) -> f(a) -> o(b)` transformations, aliases and boundary
  connections. Do not hide dependencies when a view is narrowed.
- Footer: requested E0/dE/Ef, recorded Eventness, static findings, historical
  test observations, current evidence assessment and acceptance as distinct facts.

The initial state is the existing WIR1 inspector with no observed evidence in
its API. The desired final state is one tests-first capability whose specification,
test cases, implementation inputs and evidence can be inspected together and
recovered in a later session. A changed input must not retain current acceptance.
The "null surface" metaphor describes continuity of intent through this change;
it introduces neither mathematical guarantees nor new Lang syntax.

## Slices

| Slice | Deliverable | Evidence mode |
| --- | --- | --- |
| slice-0 | Baseline, exact association/API contract, supported evidence-root layout | Declared planning observations |
| slice-1 | Requirement-to-test matrix and expected red observations before implementation | Declared red-first observations |
| slice-2 | Bounded source/scope/slice evidence association and read-only assessment | Checked tests |
| slice-3 | Clear header/body/footer evidence presentation and real HTTP/browser checks | Checked tests |
| slice-final | End-to-end regression, drift detection, fresh-session recovery and useful residue | Checked tests |

The [contract](main.lang.runtime-evidence.contract.md) defines the gates and
non-goals. Begin at [slice-0](main.lang.runtime-evidence.slice-0.todo.current.md).
No slice is accepted merely because this plan or a tests-first specification exists.

## Discover and resume

```powershell
recur warp show main.lang.runtime-evidence -d warps --json
recur warp slices main.lang.runtime-evidence -d warps --json
recur reveal main.lang.runtime-evidence -d warps
recur tree main.lang.runtime-evidence -d warps --sep .
recur files "main.lang.runtime-evidence.**" -d warps --sep .
recur trace-id "main.lang.runtime-evidence.**" --scope "main.lang.runtime-evidence.**" -d warps --ext .md --format full
```

The earlier baseline and dogfood maps report completion with declared evidence.
The shared web-lab currently reports stale evidence; neither is an automatic
acceptance baseline for this work. Preserve previous receipts and user changes.
See the [dogfood gap assessment](main.lang.dogfood.gap-assessment.md) and
[current query contract](../docs/main.command.lang.query.readme.md).

defines: main.lang.runtime-evidence tests-first specificity and evidence association
consumes: recur.lang.query.v1 existing header body footer projection
consumes: main.lang.dogfood.gap-assessment scoped runtime-evidence gap
produces: main.lang.runtime-evidence.acceptance planned source-bound capability evidence