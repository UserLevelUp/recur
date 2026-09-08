# Recur Lang static graph Warp

Status: draft implementation plan; no slices accepted.
Release target: a.0.2.8, subject to the final Recur Lang release scope.

This map makes the existing
`main.improvement.30.static-graph.todo.current.md` contract discoverable through
`recur warp`. That document remains the acceptance authority for SGR1. WIR1
and CIR1 remain historical completed contracts; this map does not reclassify
their receipts or claim that they were tested again.

## Starting evidence

- `src/recur_lang_ir.rs` defines the bounded Warp IR.
- `src/recur_lang_concurrent_ir.rs` defines `ConcurrentIr` and its parser.
- `src/recur_lang_main.rs` implements receipt-backed `recur-lang warp`.
- `main.improvement.30.concurrent-ir.contract.complete.md` records CIR1 acceptance.
- `main.improvement.30.static-graph.todo.current.md` specifies the unimplemented
  shared graph report and its existing exit evidence.
- `../demos/main.lang/main.lang.skippy-watch-coordination.recur` supplies the
  accepted concurrent source fixture.

No static graph module or pure `recur lang` command was found in the source
inspection on 2026-09-08. This is an inspection result, not a fresh test baseline.

## Proposed slices

| Slice | Result | Acceptance gate |
| --- | --- | --- |
| slice-0 | Reassess CIR1 and freeze SGR1's exact report fields, graph direction, entry/reachability rules, and finding identities. | Run existing Rust IR/companion and focused Julia fixture tests; record actual results. Add focused failing graph cases for missing behavior without weakening existing tests. |
| slice-1 | Deterministic source-bound topology report over an existing `ConcurrentIr`. | Exact five fixture lanes in authored order; every input dependency and all three ordered awaits preserved; JSON includes schema, IR schema, source hash, nodes, edges, waits and findings; repeated normalized output is byte-equivalent. |
| slice-2 | Report dependency cycles, wait cycles, unreachable lanes and unsatisfied joins. | Independent synthetic positive and negative graphs exercise each finding, exact identities and available source spans; the valid fixture has no blocking finding; `orchestration_sound` agrees with findings. |
| slice-final | Accept the complete SGR1 contract against current artifacts. | Full Rust regression, focused Julia fixture regression, deterministic JSON and read-only checks, formatting/diff hygiene, and no new Clippy diagnostics in touched modules. Follow the existing contract's implementation-commit and manual Eventness transition requirements. |

Dependencies are sequential. `slice-0` is the first ready slice. Test descriptions
above are proposed acceptance work; no passing runs or receipts are invented.
The map uses the repository's declared evidence gates. Review actual logs and
their source applicability before recording acceptance; a gate label alone does
not establish checked external evidence.

## Scope and downstream Warps

The product purpose is compact, scoped understanding with less cognitive load.
SGR1 supplies the exact graph facts that later queries can explain using local
function letters and input/output bundle references, expanding details only as
needed. The analyzer reports structural findings; repair recommendations and
other opinionated language behavior belong in `recur-lang`.

SGR1 consumes CIR1 directly. It adds no parser, new source syntax, scheduler,
watcher, worker execution, CLI query family, or grid renderer. Preserve the
existing contract and use a revised contract identity if its requirements change.

After SGR1, build separate bounded Warps from the existing design:

1. Pure language queries: `main.lang.baseline` now plans the bounded baseline
   in `warps/main.lang.baseline.contract.md`. Select the first useful commands from
   `main.command.lang.readme.md` and project the shared model/report. Freeze
   scope plus Eventness filtering, header/body/footer explanation, compact
   function letters, lossless bundle expansion and visible boundary/cycle paths
   as acceptance criteria. Keep opinionated guidance in `recur-lang`.
2. Grid snapshot: use `main.improvement.30.live-grid.todo.tracking.md` after the
   graph and pure query prerequisites are accepted.
3. WIR1/CIR1 convergence: freeze how receipt-backed Eventness and concurrent
   lane facts share source and contract identity before live coordination.
4. Live coordination and one real Rust dogfood change: use the existing watch
   coordination proposal, with worker, receipt, parent-integration and recovery
   gates defined before implementation.

These are downstream candidates, not an assertion that all must ship in 0.2.8.
The final release cutoff remains a product decision.

## Resume

```powershell
recur warp show main.improvement.30.static-graph -d docs
recur warp slices main.improvement.30.static-graph -d docs
recur files "main.improvement.30.**" -d docs
```

defines: main.improvement.30.static-graph.warp bounded implementation plan for the existing SGR1 contract
consumes: recur.lang.concurrent.ir.v1 frozen communication model
consumes: recur.lang.static.graph.report.v1 recorded SGR1 acceptance requirements
triggers: recur.lang.query.surface downstream pure projections after SGR1 acceptance
