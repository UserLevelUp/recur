# Improvement 34 coordinator contract

artifact.type = contract
defines: main.improvement.34.implementation.contract
consumer: main.improvement.34.implementation.initial
warp.id = main.improvement.34.implementation
warp.uuid = 01a1ba42-9e21-7c7b-8297-4909f211ef5b
status = planned; no acceptance

## Parent acceptance

- `accept-trace`: after live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.
- `accept-workflow`: after live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.
- `accept-graph`: after `accept-trace` and live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.
- `accept-eventness`: after `accept-trace` and `accept-workflow` and live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.
- `accept-evidence`: after `accept-eventness` and live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.
- `accept-review`: after `accept-eventness` and `accept-evidence` and live child completion, review its exact map, accepted layers, checked inputs and limits. Run its representative Julia integration case at the candidate revision. Bind the child's current contract and accepted evidence to the parent test manifest.

Each parent acceptance slice has a distinct checked Julia test gate. No child or coordinator result has been accepted. Inspect child projection state as well as the parent layer. A parent receipt cannot compensate for a missing, incomplete, stale or conflicting child. No speculative child result hash is pinned at planning time; bind actual sources and receipts when accepting.

## Integration

Gate `integrate-julia` requires the six accepted child integrations, a fresh-root Julia walkthrough spanning discovery, declaration, revision mismatch, grounded expansion, acceptance, correction and rehydration, and compatibility checks for scoped output and existing receipt/gate behavior. Confirm queries are read-only and distinguish incomplete discovery and unperformed validation from success. Test an opted-in review refusal and a legacy acceptance without that policy.

Proposed integration command: `julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.integration.test.jl`. Its entrypoint is created during implementation. Run only the appropriate focused Julia and Rust suites; demo tests remain opt-in. Record exact build/test inputs, output, results and reviewed residual limits. Preserve prior observations, including failed attempts.

## Predicted final Eventness

All six child outcomes are inspected and accepted independently, the combined behavior and compatibility checks pass at a recorded revision, and the root proposal points to the observed integration record. If the result diverges, record the failed criterion and remaining uncertainty instead of declaring completion or rewriting old evidence.
