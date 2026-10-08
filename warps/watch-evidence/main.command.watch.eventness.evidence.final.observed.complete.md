# Final Eventness: exact tested binaries retained

artifact.type = lane
eventness.role = observed-final
defines: main.command.watch.eventness.evidence.final.observed
consumer: main.command.watch.eventness.evidence.final.target
publish: main.command.watch.eventness.evidence.final.ready
warp.id = main.command.watch.eventness.evidence
warp.uuid = 01a11d10-5e20-7023-a567-6d2d57373cce
status = verified locally; live gates accepted
verification.bundle = watch-native-v9

This Warp attained its predicted local outcome. The recorded hashes match the
exact retained executable bytes used by the native and Julia checks. A subsequent
Cargo rebuild changed mutable `target/release-safe/recur.exe` from
`d6c4b0d0a5cb467cb7f1ea3fa4d8194df9e6f19be0ffe6f8dd84547cc5fd4dc4` to
`d57058fb8e80e6d17c53d259269019ad6e668aed5d32e6f0e21900384c9e61d8`.
The retained core kept the first hash, and all five preceding evidence assessments
remained checked. Final acceptance then passed against the retained artifacts.
The raw rebuild and overwrite-refusal proofs are in `observations/`.

## Observations and accepted results

- Rust: 314 passed, zero failed or skipped.
- Selected Julia compatibility checks: 172 passed, zero failed or skipped.
- Node: 15 native/scaffold + 27 integration + 11 binary-bundle + 7 final acceptance
  checks passed; 60 total, zero failed or skipped.
- Both `main.command.watch.eventness` and this repair Warp were complete in live
  show, list and merge queries, with 5/5 slices covered and no pending/blocked slices.
- Five immutable refresh receipts renew the original native-v6 references using
  current v9 evidence. Original accepted layers and observations were preserved.
- A repeated native verification command refused the existing prefix and left
  all five existing observation files unchanged.

The verifier retains normal core/Watch/Warp artifacts and the isolated fault-hook
artifact separately, records provenance, executes explicit retained paths, detects
mutation, and preserves failed postcheck diagnostics. Retained bytes, raw logs and
run metadata are bound into source evidence. Artifact copies preserve execution
permissions where supported. The optimized FNV calculation has the same identity
as the canonical algorithm, demonstrated by regression vectors and live Rust
evidence assessments.

Full-scope refresh retains every predecessor input path under the explicitly
selected 512-file budget. The 128-file default, 1024-file selection ceiling,
byte/receipt limits and contained-path rules remain. Legacy no-history evidence
assessment and declared in-memory Warp composition retain their prior behavior.
Invalid explicit policies and external evidence with a missing map fail closed.

## What failed and what we learned

The expected-red bundle tests recorded eight missing-behavior failures. Refresh
tests exposed the missing scope-selection behavior. Broader verification v8 then
found two pure in-memory composition regressions: policy validation tried to read
a nonexistent file. Their cause was identified, their failed output retained, and
the corrected full v9 run passed. This was an unsuccessful attempt within this
Warp, followed by a supported repair of the same predicted outcome.

The first v7 wrapper spent several minutes fingerprinting retained executables
with BigInt operations. It was stopped after its inputs were superseded; the
partial reservation and artifacts remain, with an interruption diagnostic. Its
buffered child output was not retained. The faster equivalent calculation makes
the subsequent source-bound runs practical. Stronger interruption-safe streaming
of wrapper output is useful future work; this Warp does not claim it exists.

Independent review also found lost Unix execution bits and missing diagnostics
on a failed postcheck. Both were corrected and regression-tested. Unix-specific
execution-bit behavior was reviewed but not executed on this Windows host.

## Attainable limits and clues for another Warp

The original v6 executable remains unavailable; its exact changed bytes cannot
be reconstructed from its hash. We retained its historical evidence and produced
truthful new observations instead of editing hashes to manufacture a match.

Artifact retention is local and ignored by Git. Keep `.recur/evidence-binaries/`
when inspecting this evidence. Cleanup or a clone without those files makes the
corresponding evidence missing/stale. Export, remote storage, interruption-safe
log streaming and automatic retention policy are possible next-Warp subjects.
Independent builds are not guaranteed bitwise reproducible. Further refreshes
must retain predecessor inputs and fit the existing aggregate bounds; retained
artifact growth may eventually require a new assessment strategy.

Watch and trace IDs remain optional. A Watch notification points to these findings;
it does not carry their body or automatically accept gates. No demo suite was selected.
The predicted final Eventness remains a record of the target; this observed residue
explains how closely this single Warp reached it and which limits remain.

pull.evidence = warps/watch-eventness/observations/watch-native-v9.binaries.json
pull.progress = warps/watch-evidence/observations/main.command.watch.eventness.evidence.watch-native-v9.complete.json
