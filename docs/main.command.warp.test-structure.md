# Warp eventness test structure

Status: `implemented`
Date: 2026-07-16

Warp fixtures express an eventness slice in the filename, rather than relying
on directories or private domain behavior:

```text
<domain>.warp.slice.<slice>.<evidence>.<state>.md
```

For example:

```text
demo.warp.slice.alpha.evidence.verified.md
demo.warp.slice.alpha.interface.needs-review.md
demo.warp.slice.alpha.approval.awaiting.md
```

The requested lane is a dot-separated boundary. Querying
`demo.warp.slice.alpha` may inspect only files whose eventness name begins
`demo.warp.slice.alpha.`. It must not absorb a lexical lookalike such as
`demo.warp.slice.alphabet.evidence.verified.md`.

## Command contract matrix

| Query | Read-only projection | Required test evidence |
|---|---|---|
| `warp status <lane>` | verdict, objective, files, groups, roles | exact scoped files and no lookalike leakage |
| `warp explain <lane>` | status evidence and residual paths | same verdict as status |
| `warp next <lane>` | suggested action packet | same action kinds as status |
| `warp collapse-plan <lane>` | `collapse_known`, `preserve_interesting`, `blockers`, `ambiguous` | every scoped file appears in exactly one bucket |
| `warp config` | active suffix policy | configured mapping and defaults are observable |

The fixture at `julia-tests/fixtures/warp-command-v1/` uses one slice with a
verified evidence file, a review-needed interface file, an awaiting approval
file, and an excluded sibling. Its Julia contract is
`main.command.warp.structure.test`.

Suffix policy, not filename prose, determines the initial bucket:

- `complete`/`verified` → `collapse_known`;
- `strange`/`needs-review` → `preserve_interesting`;
- `blocked`/`awaiting`, or a blocker marker → `blockers`;
- `current` → `ambiguous` until later evidence changes it.

All commands remain projections. A collapse plan identifies eventness that
could be collapsed; it neither renames nor deletes it.

Trace-id lines:

```text
defines: recur.warp.test.structure dot-separated synthetic lane fixture grammar and query contract matrix
defines: recur.warp.lane.boundary requested lane prefix requires a trailing dot boundary and excludes lexical lookalikes
defines: recur.warp.collapse-plan.read-only partition of scoped eventness into known interesting blockers and ambiguous buckets
consumes: recur.warp.status evidence-backed read-only lane verdict
produces: recur.warp.fixture.matrix reusable status explain next collapse-plan and config regression contract
```
