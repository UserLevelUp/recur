# SGR1 acceptance, 2026-09-08

Implementation commit: `7d042c4f7a77fd690728bafb900c39258f3a4c86`.
Contract reviewed: the original `main.improvement.30.static-graph.todo.current.md`,
SHA256 `3e71f9d25d999d5d87bd81520ab121918c314943b374e97dda1380c898df790a`.
Its exact pre-transition bytes are preserved as
`warps/main.lang.baseline.evidence/sgr1-contract-before.md`.

`src/recur_lang_graph.rs` consumes only `ConcurrentIr`. It preserves five authored
lane nodes, coordinator port owners, all typed message dependencies and three
ordered awaits for the Skippy fixture. It exposes the CIR1 schema, source identity,
FNV1a source hash, source spans, reachable lanes and deterministic findings.
No source parser, scheduler, worker, write or Eventness transition is part of SGR1.

## Observed gates

| Slice/gate | Evidence and applicability |
| --- | --- |
| slice-0 / baseline-and-report-contract | Existing WIR1/CIR1 baseline: 15 Rust tests passed. Isolated `c97e326` archive graph probe failed with E0432 for absent SGR1; the equivalent live graph fixture is green. Exact SGR1 contract reviewed as above. |
| slice-1 / source-bound-deterministic-topology | `exact_fixture_and_repeatable_json` checks exact lane order, three required-message waits, source hash and byte-identical repeated JSON. Source/schema binding is also tested at the query boundary. |
| slice-2 / positive-and-negative-soundness-findings | Graph tests prove dependency/wait cycles with closed explanatory paths, self-cycle, unknown entry/schema, typed identity mismatch, unreachable lane and missing join. The valid fixture has no findings. |
| slice-final / sgr1-regression-and-contract-acceptance | Full native Linux Cargo regression: 200 passed, 7 existing doc tests ignored. Focused packaged Windows Julia language suites pass, preserving prototype broken cases. No new Clippy diagnostics in touched Lang modules; rustfmt and diff checks pass. Implementation commit recorded before manual lifecycle transition. |

Durable test logs: `warps/main.lang.baseline.evidence/rust-ci.txt`,
`clippy-ci.txt`, `julia-packaged.txt`, `graph-baseline-red.txt`, `rust-baseline.txt`.
Native run https://github.com/UserLevelUp/recur/actions/runs/34216797900 used Rust
1.85.0. Its Rust results and archive smoke tests passed. Its *full* Julia run found
pre-existing portability/fixture failures and is not accepted as passing evidence;
SGR1 requires focused Julia fixtures, which passed separately with Julia 1.12.7.
Baseline integration is separately gated on a corrected full regression run.

The corrected run https://github.com/UserLevelUp/recur/actions/runs/34217859504
subsequently passed 200 Rust tests and 3,727 Julia assertions (73 existing broken).
Its `cc7adee` revision leaves the Rust/Cargo implementation identical to `7d042c4`.
Final logs are preserved beside the initial evidence as `rust-ci-final.txt`,
`clippy-ci-final.txt` and `julia-ci-final.txt`.

SGR1 retains its bounded CIR1 semantics: mixed await contracts are a finding;
unknown coordinator input signatures and runtime state are outside coverage. Its
report is a static answer, not receipt acceptance or runtime success.

All Warp gate bindings use declared evidence. The logs and applicability above
were reviewed; the generic Warp engine does not independently run these tests.
Baseline slice-1 must separately review this accepted child and source contract.

defines: recur.lang.static.graph.verification accepted bounded SGR1 evidence
consumes: recur.lang.concurrent.ir.v1 source-bound communication facts
produces: main.improvement.30.static-graph.contract.complete manual acceptance after implementation commit
