# Improvement 34 trace: observed final Eventness

artifact.type = lane
eventness.role = observed-final
defines: main.improvement.34.implementation.trace.final.observed
consumer: main.improvement.34.implementation.trace.contract
warp.id = main.improvement.34.implementation.trace
warp.uuid = 01a1ba42-9d49-7012-b9c1-7b956e58aa54
status = verified locally; all five child slices accepted

The trace bubble reached its predicted first relationship. An unscoped query
finds only matching paths; adding a scope preserves the heuristic view.
`--declared` opts into exact Markdown records and independently checks revision
expectations and SHA-256. In the synthetic delivery fixture R04 has the expected
bytes, but it fails the R06 pin. Intentional R04 history passes, and a reference
outside the root cannot establish integrity. Discovery and declaration queries
remain read-only.

## Evidence and failures

- [Trace suite](observations/trace-v2.trace.log.txt): 43 passed, zero failed or
  skipped. [Checked evidence](observations/trace-v2.evidence.json) binds the actual
  results, sources and retained executables. The four checked gates use this
  combined trace suite.
- [Julia baseline](observations/trace-v2.baseline.log.txt): 13 passed, zero skipped.
  [Legacy compatibility](observations/trace-v2.legacy.log.txt): 88 passed with
  three pre-existing skipped placeholders, outside the no-skips checked gate.
- Nine targeted Rust trace tests passed. No demo suite ran.
- [Expected red](observations/trace-v2.expected-red.log.txt): 9 failed and 9 passed
  against the pre-implementation CLI. [First verification attempt](observations/trace-v1.baseline.log.txt)
  reached the trace tests, then failed to launch the baseline checkpoint because
  its local bundle omitted `recur-git`. That attempt did not cover a gate. V2
  retained `recur`, `recur-warp` and `recur-git` and passed the selected checks.

[Live child projection](observations/trace-v2.accepted-progress.json) showed
5/5 slices complete. The initial baseline is a declared review; the four
remaining gates have checked test evidence. Re-query from the repository root
for current freshness. The retained executables are local under
`.recur/evidence-binaries/improvement34-trace-v2/`. A checkout without those bytes
cannot verify this evidence. Historical observations are preserved.

## Handoff

The [trace CLI reference](../../../docs/main.command.trace-id.readme.md) defines
the current path and explicit declaration output. The next graph bubble can
build on that contract after inspecting the live trace handoff. Current-revision
selection and graph traversal remain separate. The workflow bubble can proceed
independently. Child completion does not accept the coordinator's integration
slice, and no Watch subscription or Lang dependency is needed.
