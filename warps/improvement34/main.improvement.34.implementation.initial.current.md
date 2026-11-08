# Improvement 34 implementation: initial Eventness

artifact.type = lane
defines: main.improvement.34.implementation.initial
consumer: main.improvement.34
warp.id = main.improvement.34.implementation
warp.uuid = 01a1ba42-9e21-7c7b-8297-4909f211ef5b
status = planned; no implementation or acceptance

Date: 2026-11-08. The [root proposal](../../README.CORE.IMPROVEMENT34.md) is the requirements source. Existing trace queries are heuristic, scope is currently mandatory, and evidence freshness does not prove an applicable revision or engineering correctness. The existing Julia baseline is `julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.baseline.test.jl`.

| Bubble | Review prerequisites | Predicted deliverable |
| --- | --- | --- |
| [trace](trace/main.improvement.34.implementation.trace.contract.md) | Independent | Trace discovery, declarations and pinned revisions |
| [workflow](workflow/main.improvement.34.implementation.workflow.contract.md) | Independent | Workflow guidance and settled attention |
| [graph](graph/main.improvement.34.implementation.graph.contract.md) | trace | Contextual revisions and bounded relationship graph |
| [eventness](eventness/main.improvement.34.implementation.eventness.contract.md) | trace, workflow | Grounded Eventness and contract revision |
| [evidence](evidence/main.improvement.34.implementation.evidence.contract.md) | eventness | Corrections and evidence scope |
| [review](review/main.improvement.34.implementation.review.contract.md) | eventness, evidence | Opted-in expansion and acceptance review |

Start the trace and workflow bubbles independently. In trace, prove one end-to-end declaration/pin case before graph traversal. After trace, graph can proceed separately. Eventness establishes the revision and handoff rules before evidence adds correction and scope views. Review consumes both. Child maps track local slice order; this plan states cross-bubble prerequisites. The coordinator map orders parent acceptance explicitly. Core structural readiness is not authority to bypass the prerequisites or to dispatch workers.

The [ring](../../main.improvement.34.implementation.warp-ring.json) contains six worker domains and one coordinator with no subscriptions. Watch can optionally announce trace IDs while findings and verification remain in Eventness. Neither Watch nor Lang is required. Root-scoped native projections keep each child's live completion separate from parent acceptance.

```powershell
recur warp show main.improvement.34.implementation.trace -d . --json
recur warp slices main.improvement.34.implementation.workflow -d . --json
recur warp merge main.improvement.34.implementation -d . --json
```

Use the repository root because the maps bind source evidence there. During implementation, use the matching local binaries. See the [coordinator contract](main.improvement.34.implementation.contract.md) for integration gates and predicted final Eventness.
