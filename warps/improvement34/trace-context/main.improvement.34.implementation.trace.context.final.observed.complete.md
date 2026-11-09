<!-- recur-trace-v1
defines: main.improvement.34.implementation.trace.context.final.observed
consumer: main.improvement.34.implementation.trace.context.contract
consumer: main.improvement.34.implementation.trace.review.separator
consumer: main.improvement.34.implementation.trace.review.byte_budget
-->
# Trace sets and contextual inspection: observed final Eventness

artifact.type = lane
eventness.role = observed-final
warp.id = main.improvement.34.implementation.trace.context
status = five checked child slices accepted locally

The predicted workflow is implemented: discover paths, explicitly inventory
observed IDs, select exact IDs or families, then extract bounded nearby source
text. Both scanner review defects are repaired. The original trace bubble has
fresh evidence under its unchanged slice contracts; its five historical accepted
layers were checked byte-for-byte against `c9887ed` and preserved.

## Use

```powershell
recur trace-id 'main.improvement.34.**'
recur trace-id 'main.improvement.34.**' --scope '**' --list-ids --ext .md
recur trace-id 'main.improvement.34.**' --scope '**' --context 3 --select main.improvement.34.implementation.trace.context.contract --ext .md
```

Inventory reports observed tokens, file counts and occurrence counts without
inferring declaration authority. Selectors form a union within the primary
pattern. Context includes source paths and one-based line ranges, merges adjacent
or overlapping windows, and preserves the actual text. Output and scan coverage
are separate; `complete` requires both. Defaults remain paths-only without scope.
See the [command reference](../../../docs/main.command.trace-id.readme.md).

## Observed checks

The [current checked manifest](observations/context-v3.evidence.json) binds **127
explicit input files** and **367 passing checks with no skips**:

- [122 Julia inventory/context checks](observations/context-v3.context.log.txt).
- [43 original trace checks](observations/context-v3.trace.log.txt).
- [3 separator regression checks](observations/context-v3.separator.log.txt),
  including the two previously failing custom separators.
- [199 Rust core tests](observations/context-v3.rust.log.txt): 115 library and
  84 CLI tests. Seven new reader/scanner tests cover physical byte accounting,
  exhausted budgets, partial errors, growing input limits, binary samples,
  identifier suffixes and contained digest reads.

Supplemental [baseline checks](observations/context-v3.baseline.log.txt) passed
13 assertions. [Legacy checks](observations/context-v3.legacy.log.txt) passed
88, with three existing broken/skipped placeholders outside the checked gate.
Passing trace suites are now registered in the core Julia runner. No demo suite
was selected, and the full repository Julia/integration suite was not run.

[New bubble projection](observations/context-v3.accepted-progress.json) and
[original trace projection](observations/context-v3.predecessor-progress.json)
both showed **5/5 covered, zero blocked, conflicting or stale-contract slices**.
The [coordinator ring](observations/context-v3.ring.json) remains blocked: its
other planned work and parent acceptance are separate. This follow-up is linked
through trace lineage and capsules, not silently added to the original ring's
acceptance contract.

## Retained failures and evidence maintenance

The [initial test draft](observations/context.expected-red.log.txt) produced
24 passes and 38 failures against the previous CLI: inspection flags were absent
and custom separators still missed punctuation. Fixture expectations were then
aligned with existing `**` semantics (zero or more segments) and complete-empty
exit behavior. The [first implementation check](observations/context.first-check.log.txt)
passed 108 assertions; additional boundary tests brought the final suite to 122.

V1 verification passed all selected checks. Publication accepted the new slices
and refreshed two predecessor gates, then hit the existing **128 unique-file
limit**. The partial [publication record](observations/context-v1.publication.json)
is retained. Both maps now select the documented 256-file allowance. No gate rule,
contract identity, source-path preservation requirement or byte limit was relaxed.

V2 tests passed, but its evidence runner rejected rebuilt Rust executable bytes
that differed from the retained originals. No V2 accepted manifest was produced.
[The binary comparison](observations/context-v2.binary-drift.json) records both
hashes and retained failed-run copies: each executable has the same length but
nine differing bytes, including PE timestamps. This is direct evidence of binary
metadata drift; matching source files do not imply identical executable hashes.

V3 re-executed the retained Rust harnesses after checking unchanged Rust build
inputs, and reran the Julia suites with the exact retained Recur executable.
It preserved every predecessor input path and reused the same binary paths,
avoiding duplicate binary copies in the bounded evidence chain. The
[run record](observations/context-v3.runs.json) names actual commands and hashes.
[Immutable refresh records](observations/context-v3.publication.json) make V3
the current evidence leaf for all five new gates and all four original checked
gates. Original layers, manifests and failure logs remain unchanged.

Executable evidence is local under `.recur/evidence-binaries/`; a fresh clone
without those bytes cannot verify these records. Re-query live state from the
repository root after any source change. Recorded snapshots are historical.

## Next useful work

- Add evidence-publication preflight that estimates the complete refresh chain's
  file/byte allowance before the first write, while retaining exact inputs.
- Make retained-harness reuse a clearer runner option with source/build-input
  validation. Study reproducible executable metadata separately; never replace
  observed hashes with rebuilt ones.
- Continue the planned workflow and graph bubbles. Per-file omission explanations,
  path exclusions and saved discovery diffs remain optional follow-up proposals.

The current CLI does not infer semantic explanations, execute embedded commands,
require a Warp/Watch/Lang setup, or implement the pending relationship graph.
