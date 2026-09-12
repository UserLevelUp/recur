# Slice-0 observations and frozen boundary

2026-09-12, C:/src/recur. Initial git status was clean. No commits or installation.
Both installed skills, repository copies and full reveal capsules were explicitly
read, followed by readme/map/current task/contract and Improvement 30 guidance.
Live queries showed five pending slices, only slice-0 ready. Help has no recall;
reveal and recorded dogfood verification/gap artifacts supplied context recovery.
Tree/files selected this lane. Trace-id for the lane returned no sites (exit 1);
the explicit `recur.warp.evidence.integrity.external` source trace returned its
definition at src/warp_evidence.rs:2. Source trace `assess` exposed `check` with
an ambiguous source match; reading the file resolved the actual call. Julia
`respond` source trace had no recognized callees, not proof of no dependencies.
Scoped Lang query query.q retained select.o(b), boundary edge and not-run footer.

Observed commands:

- `cargo test --locked --test lang_query`: exit 0, 3 passed.
- `cargo test --locked --lib warp_evidence::tests`: exit 0, 1 passed.
- Julia 1.12.7 WindowsApps launcher, `--startup-file=no -C generic --compile=min
  --project=demos/web-evidence-lab`, include main.demo.lang-inspector.test.jl then
  main.demo.lang-dogfood.test.jl: exit 0, 333 passed (33+33+111+103+53).
- Cargo 1.97.1; Julia used target/release-safe/recur.exe via RECUR_BIN. Rust
  integration used the freshly built debug executable. Both Recur builds say 0.2.8.
- Existing unused Element warning; no baseline test failures. Evidence recording
  initially referenced nonexistent src/lang_query.rs; corrected to discovered
  src/recur_lang_query.rs. This recording error was not a behavioral red test.
- main.web-lab: ten browser gates stale, each says source changed: main.server.jl.
  Historical receipts untouched; relevant shared HTTP behavior passed above.

Frozen [association/API contract](main.lang.runtime-evidence.association.contract.md)
uses a separate endpoint, exact scope/alias/attempt identities and bounded reads.
Development map moved to repository root with unchanged UUIDs and one live map.
`recur warp show main.lang.runtime-evidence -d . --json` locates that map and
retains all five slices. `recur warp evidence
warps/runtime-evidence/slice-0-rust.evidence.json -d . --json` returned checked
against seven explicit actual inputs; no parent traversal or snapshots.
Structured Rust results and separate historical Julia fingerprints are under
runtime-evidence/. Source scope is explicit, not dependency closure.

This slice accepts planning/baseline and contract only. No loader, new behavioral
test, runtime Ef or later acceptance is claimed. Next: slice-1 tests-first matrix.

produces: main.lang.runtime-evidence.baseline observed baseline and frozen boundary
