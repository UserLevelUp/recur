# Reconciliation: tests before doc fixes

Initial run on 2026-09-05, base commit `837dee4`: **10 passed, 4 failed,
0 errored, 0 expected-broken**. Failures identify the three stale claims and the
not-yet-created claim matrix. The matrix failure is a new acceptance target,
not a discovered runtime bug. No Warp slice has been accepted by this run.

Reconciled 2026-09-06: expanded suite passes 49 assertions, including link and
historical-boundary checks. See the final verification receipt for all focused runs.

Run `julia julia-tests/main.command.warp.docs-reconciliation.test.jl`.
This acceptance suite is now included in the default regression runner after
reconciliation and can still run standalone. Failures are real
`@test` failures, not converted to expected-broken cases. No production code or
root proposal text is changed by this test-first step.

The initial targets are three known stale addendum claims: evolution as future-only
in the implementation summary and command table, and recursive domains/subscriptions
as awaiting a frozen schema. The existing companion CLI is checked with `--help`
only; no writer command is executed. Historical intent should be preserved with
explicit dated wording while removing these unqualified current-state claims.

Slice 1's planned claim matrix gets a minimal machine-readable counterpart:
`docs/main.command.warp.docs-reconciliation.claims.json`:

```json
{
  "schema": "warp-doc-claims-v1",
  "claims": [
    {
      "id": "evolve-collapse",
      "status": "partially-implemented",
      "summary": "REPLACE with the audited claim, not this example",
      "limitations": "REPLACE with confirmed limitations",
      "publication": "REPLACE with observed commit/release scope",
      "docs": [{"path": "relative/doc.md", "contains": "exact supporting text"}],
      "source": [{"path": "src/file.rs", "contains": "supporting symbol or code"}],
      "tests": [{"path": "julia-tests/file.test.jl", "contains": "supporting test"}]
    }
  ]
}
```

The example is not an accepted classification. Required families are scoring-config,
status-explain-next, maps-rings, complete-receipt, evolve-collapse, evidence-freshness,
reveal, discovery, milestones-temporal, and methodology. Classifications are
implemented, partially-implemented, proposed, superseded, or unresolved.
Every claim needs a summary, limitations, publication scope, and document evidence;
implementation claims additionally need source and focused-test references.
Paths must resolve within the repository and contain the cited text.

Synthetic validator tests catch missing families, duplicate IDs, unsupported status,
promotion without implementation evidence, escaping paths and nonexistent excerpts.
These enforce reference integrity, not semantic truth. Human/source review and the
existing focused behavior suites remain necessary: help text or a matching code
snippet alone does not prove a claim. The initial suite is not exhaustive acceptance
of the entire Warp; supporting-doc links, recovery and final closeout checks must
be added as the audit establishes their exact targets. Do not accept slices merely
because this suite turns green.
