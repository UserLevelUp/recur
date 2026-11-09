<!-- recur-trace-v1
defines: main.improvement.34.implementation.trace.review
defines: main.improvement.34.implementation.trace.review.separator
defines: main.improvement.34.implementation.trace.review.byte_budget
consumer: main.improvement.34.implementation.trace.contract
-->
# Trace Warp verification: two open defects

artifact.type = lane
eventness.role = observed-review
warp.id = main.improvement.34.implementation.trace
status = findings-open; implementation not changed by this review

Reviewed commit `447bfbfba74803a0393171a729290133e28c3803` on `recur-lang`.
This assessment covers the first Improvement 34 trace bubble, not the remaining
workflow, graph, Eventness, evidence or review implementations.

The primary paths, exact declarations and explicit revision-pin cases work in
the tested fixtures. Matching R04 bytes still fail an R06 revision expectation;
intentional R04 history passes. However, two defects prevent an unqualified
correctness claim. Historical acceptance and the observed counterexamples are
both retained.

## Findings

### P2: sentence punctuation hides IDs using a custom separator

Trace ID: `main.improvement.34.implementation.trace.review.separator`.

`src/trace_relationships.rs:188-192` always keeps periods inside candidate tokens,
but trims only the selected hierarchy separator. A file containing
`Reference capability_baseline.` therefore disappears from a path query for
`capability_baseline --sep _`. The same failure occurs with `--sep -`. Both
queries report complete coverage and exit 1 despite the matching ID being present.
The dot-separated control succeeds.

The independent [Julia counterexample](observations/review-v1.separator.test.jl)
ran against the retained implementation executable: **1 passed, 2 failed**;
see its [raw output](observations/review-v1.separator.log.txt). This is an
explicitly selected failing review fixture, not a new accepted gate.

Repair sentence-punctuation handling consistently for supported separators,
while retaining whole-token checks that prevent silently shortening invalid or
longer identifiers. Add regression coverage for punctuation, descendants and
underscore/hyphen suffixes before accepting the correction.

### P2: binary sniffing bypasses the declared byte budget

Trace ID: `main.improvement.34.implementation.trace.review.byte_budget`.

In `src/trace_relationships.rs:125`, the scanner calls
`is_likely_binary_file(path)` before calculating the remaining byte allowance.
That helper opens the file and reads up to 1,024 bytes in `src/search.rs:226-234`.
Those reads are absent from `coverage.bytes_read`, and this sampling continues
for later files even after the aggregate budget is exhausted. For example,
`--max-bytes 1` can still sample 1 KiB from each ordinary candidate before its
full read is rejected. Both path discovery and declared scans share this code.

This finding is established by source inspection; physical filesystem I/O was
not instrumented. The existing byte-limit test verifies an empty result and
incomplete coverage, so it cannot detect these extra reads. Check containment
and remaining budget before opening a candidate, and classify a bounded buffer
that is charged to the same budget and reused for parsing. Add a reader-level
test that counts bytes, including binary samples and exhausted-budget paths.

## Verification evidence

- [New trace suite](observations/review-v1.trace.log.txt): **43 passed**, no skips.
- [Baseline](observations/review-v1.baseline.log.txt): **13 passed**, no skips.
- [Legacy trace suite](observations/review-v1.legacy.log.txt): **88 passed**, with
  three existing broken/skipped placeholders.
- `cargo test --locked --bin recur main_command_trace_id -- --nocapture`:
  **9 passed**. These are existing trace-command unit tests; they do not provide
  direct unit coverage for the new scanner module. The existing unused `Element`
  warning remains.
- [Additional edge probes](observations/review-v1.edge-results.json): **8 passed,
  2 failed**. They exercise exact/family queries, authored case preservation,
  unterminated blocks, conflicting revisions, context isolation and truncated
  scans. The two failures are the separator finding reproduced by Julia above.
- [Executable checks and rerun summary](observations/review-v1.summary.json):
  all three retained executable SHA-256 values match the original binary record.
- [Live evidence inspection](observations/review-v1.live-evidence.json): all
  **101 named input fingerprints** and the external result remain checked.
- [Live child projection](observations/review-v1.live-progress.json): **5/5
  accepted** under the existing gates. This does not evaluate the new findings.
  The coordinator ring remains blocked with parent integration unaccepted.

The Julia tests use the `demos/web-evidence-lab` project only for dependencies;
no demo suite was selected. The full repository suites were not run.

## Regression coverage and next action

The new passing core suite is currently standalone; `julia-tests/runtests.jl`
does not include it. Also strengthen `trace34` so an expected successful JSON
response cannot be empty: its current early return can bypass semantic checks.
Add the repaired passing cases to ordinary core regression coverage; keep the
historical expected-red run and raw review observations intact.

Open a bounded follow-up repair under this trace lineage, fix both defects,
rerun the scoped checks, and publish fresh source-bound evidence for the changed
implementation. Reassess parent integration separately. Do not overwrite the
old layers or infer that their still-matching fingerprints resolve these findings.

Review artifacts are left uncommitted on `recur-lang`. No implementation, branch
ref, stash or remote was changed. The local `main` and `a.0.2.8` refs remain at
planning commit `48aca7d`. A Recur Git snapshot for the review is retained at
`.recur/improvement34-review-oJPdBt/checkpoint.snapshot.txt`.
