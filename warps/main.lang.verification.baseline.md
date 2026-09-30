# Verification lab: observed tests-first baseline

defines: recur.lang.verification.baseline observed faults and controls before implementation
consumes: recur.lang.verification.v1 frozen repair expectations

Repository basis: `recur-lang` at `9f4a4de`, plus the new tests, catalog and plans.
Product sources were unchanged. Installed core and companion version 0.2.8 were
selected explicitly; their SHA-256 fingerprints and all test inputs appear in
[attempt-4.json](lang-verification/attempt-4.json).

The successful full observation used Juliaup Julia 1.12.7 with
`--startup-file=no -O0 -C generic --project=demos/web-evidence-lab`, invoking
`demos/lang-verification/main.lang.verification.jl demo --json-output ...`.
It completed with exit **1**, because these ordinary assertions deliberately
expose missing behavior. There were **58 cases: 44 passed, 14 failed, no errors,
broken or empty cases**. No test is marked expected-broken or skipped.
Across those cases, **561 assertions passed and 25 failed**. The launcher also
rejected zero-case selection and refused to overwrite an existing observation;
the original observation hash remained intact. All seven recorded test-input
hashes matched a staged Git checkout, including across line-ending attributes.

| Failing cases | Observed behavior | Owning slice |
| --- | --- | --- |
| `query.suffix._complete_md`, `query.suffix._todo_complete_md` | Full configured suffix selects zero matching headers | slice-query |
| `query.cir.empty.eventness` | CIR list emits scope objects instead of exactly `[]` | slice-query |
| `parser.comments.are.inert` | Commented fake scope interferes with source discovery | slice-parser |
| Four `checked.status.*` cases | Rewritten source hash, before/after identity or reduced input map still qualifies as currently accepted | slice-status |
| `checked.conflict.prepared.scope`, `.json` | Confirmation and recovery acknowledge despite a conflicting/corrupt prepared record beside accepted history | slice-records |
| `path.oracle.adjacent`, `.reverse`, `.empty`, `.singleton` | Existing helper rejects valid paths and accepts zero/one-vertex paths against the selected helper contract | slice-path |

The positive controls cover actual checked fixture publication/replay, accepted
record corruption rejection, all ten input-role drift cases, ID boundaries,
graph cycles and independent topology, query purity/containment, seven binary
help/version checks and the complete todo catalog. This is targeted coverage,
not proof that all backlog criteria have tests yet. The ledger retains those gaps.

## Existing suites and toolchain limitations

- Existing `main.command.lang.baseline.test.jl`: **15 assertions passed**, exit 0,
  through the demo launcher's `suite baseline` command.
- Existing runtime-evidence loader: **159 assertions passed**, exit 0, with
  explicit installed `RECUR_BIN`. Its [raw log](lang-verification/loader-attempt-0.log)
  and [per-case observations](lang-verification/loader-attempt-0.json) are retained.
  This run does not establish API/browser integration or close its Warp.
- Catalog-only run: **183 assertions passed**, exit 0;
  [observation](lang-verification/catalog-attempt-0.json).
- Normal-optimization combined run: Julia compiler access violation; see
  [attempt-2.log](lang-verification/attempt-2.log). No complete result JSON exists.
- `--compile=min` combined run: Julia interpreter access violation after reaching
  the catalog; see [attempt-3.log](lang-verification/attempt-3.log). No complete
  result JSON exists. Neither process fault counts as a product regression or a
  passing gate.
- `julia` on PATH initially selected Chocolatey's Julia 1.12.0 and faulted.
  Use the explicit Juliaup executable in the demo guide; changing the global
  Julia installation was not part of this work.

No Cargo suite, full Julia suite, exhaustive poker gate, native Linux gate or
packaged release gate was run for this tests-and-planning change. Existing tests
were preserved. No product fix, formal acceptance layer, install or publication
is claimed. The new implementation gates remain pending.
