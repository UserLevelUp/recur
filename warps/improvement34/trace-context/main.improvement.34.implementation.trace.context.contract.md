<!-- recur-trace-v1
defines: main.improvement.34.implementation.trace.context.contract
consumer: main.improvement.34.implementation.trace.review
-->
# Trace sets and contextual inspection

artifact.type = contract
warp.id = main.improvement.34.implementation.trace.context
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `c9887ed`, path discovery and explicit declarations/pins exist. The original
trace bubble was locally accepted under its tests, but later review reproduced
custom-separator punctuation failures and identified binary sampling outside the
byte budget. ID inventory and bounded excerpts are not implemented. The passing
trace suite is not yet included in ordinary core regressions, and its helper
can skip semantic assertions on empty successful output.

Preserve [the review and its counterexamples](../trace/main.improvement.34.implementation.trace.verification.findings.md).
This is a follow-up bubble; it does not rewrite the original contract or accept
the Improvement 34 coordinator. The existing graph bubble remains separate.

## Interface and boundaries

Unscoped `trace-id PATTERN` continues to print paths. Scoped legacy detail and
`--declared` retain their schemas. New explicit scoped views are:

```powershell
recur trace-id 'capability.**' --scope '**' --list-ids
recur trace-id 'capability.**' --scope '**' --list-ids --select capability.baseline
recur trace-id 'capability.**' --scope '**' --context 3 --select capability.baseline --select capability.delivery
```

Inventory reports distinct observed tokens, their namespace, files and occurrence
counts. It does not infer declarations, authority or runtime behavior. Selectors
are a union of exact/family patterns intersected with the primary pattern. Matching
can ignore case while preserving authored spelling. Root, extension and file
hierarchy filters apply before collecting context. No configuration or manifest
is needed in the inspected project.

Context returns matching lines plus the requested number of neighboring lines
(0–20), exact root-relative paths and one-based line spans. Overlapping or adjacent
windows in one file merge so a passage is not repeated. Text is extracted, not
summarized, interpreted or executed. IDs within the excerpt identify which selected
matches caused it. Read-only output can be text or JSON from the same projection.

`--max-results` bounds emitted inventory entries or excerpts (default 200, maximum
10,000). `--max-output-bytes` bounds serialized result-item bytes (default 32 KiB,
maximum 1 MiB); report metadata has separate fixed overhead. Scan coverage and
output omissions are both visible. Truncated counts describe the returned data,
not global absence. A complete empty projection exits 1; incomplete projections
remain inspectable. Detail-only flags without scope, incompatible views, and invalid
budgets fail clearly. Watch, Lang and Warp metadata remain optional.

## Slices and acceptance

1. `repair`: pass the historical separator counterexample; preserve whole-token
   suffix handling. Enforce aggregate read limits including binary detection and
   digest reads. Reader-level Rust tests count physical bytes, including exhausted
   budgets and growing input. Canonical containment precedes file reads.
2. `inventory`: distinct stable IDs, namespace, file/occurrence counts; repeated
   tokens, several files, incidental prose and generator literals remain identified
   as occurrences. Bounded output must never claim omitted data is absent.
3. `select`: exact, family and union selection within the primary pattern, with
   scope, extension, case and root confinement. Invalid option combinations fail.
4. `context`: source-linked neighboring passages, stable ordering, deduplicated
   windows, UTF-8-safe output limits, visible clipping and no file mutation.
5. `verify`: synthetic unfamiliar project walkthrough from path hunt to inventory
   and selected context. Register passing core suites, strengthen empty-response
   assertions, test old declaration/pin and scoped interfaces, and capture exact
   source/executable identities. No demo suite is selected.

Each slice has a checked test gate with no skipped checks. A combined reviewed
Julia/Rust result may bind all five gates if it covers the full contract. Retain
expected-red runs separately. Publish immutable result/evidence artifacts and
accept slices only after observing passing checks under stable named inputs.

## Predicted final Eventness

A human or coordinator can locate a capability family, inspect its actual ID set,
and read selected explanations with bounded context and explicit omissions. Known
scanner defects are repaired. Plain discovery and existing scoped callers remain
compatible. Observed results and limitations will be recorded separately.
