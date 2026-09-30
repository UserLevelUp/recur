# Recur Lang: baseline and evidence verification

```text
defines: recur.lang.work.remaining.verification bounded baseline and evidence verification
consumes: recur.lang.work.remaining parent remaining-work index
consumes: recur.lang Lang language and implementation context
consumes: recur.eventness.todo evidence-based completion convention
```

Status: open. Reviewed locally: 2026-09-29 (America/Los_Angeles).
Parent: [remaining work](README.CORE.IMPROVEMENT30.recur-lang.todo.md).
Tests-first work: [demo catalog](demos/lang-verification/main.lang.verification.readme.md),
[todo-to-test ledger](demos/lang-verification/main.lang.verification.ledger.json),
and [implementation Warp](warps/main.lang.verification.contract.md).
Stable V01-V32 markers connect each checkbox to its demos, existing coverage,
remaining tests and owning slice. A mapping does not close the checkbox.
This focus can be split into any number of narrower `.todo.md` files; retain
the trace identities and link each extracted checklist instead of duplicating it.

## Review basis and limits

Source: Marc's supplied **Recur Lang branch implementation and coverage
analysis**, prepared by dot, dated 30 September 2026. Its reviewed revision is
`4073e8ac1f64303458065da9ab120bcbbb959da0`; attachment SHA-256:
`c65e850e03a36423f5a74509876e4aef9368cd20bf6ba5e36b21b1d3409ed420`.
This follow-up inspected `recur-lang` at `db1078b` and the locally installed
Recur 0.2.8. The report's branch alignment and acceptance observations are
historical. It ran no builds or tests and measured no numerical coverage.

WIR1/CIR1, SGR1, source-bound queries, projections, legacy transitions,
checked transitions and recovery already have implementations and tests.
Preserve that work. A missing fresh gate is not a reason to reimplement it.
The [init focus](README.CORE.IMPROVEMENT30.recur-lang.init.todo.md) remains a
separate selected implementation increment.

Local observations below distinguish reproduced CLI discrepancies, inspected
source defects/gaps, and unexecuted hypotheses. None constitutes current Warp
acceptance. All implementation and acceptance checkboxes remain open.

## P0: reproducible baseline

defines: recur.lang.work.remaining.verification.baseline reproducible bounded baseline

<!-- verification: V01 -->
- [ ] Correct `every_path_is_contiguous` in
  `julia-tests/main.lang.pathing.test.jl`. Source inspection confirms
  `(length(tiles) >= 2 || any(isnothing, tiles)) && return false` rejects valid
  multi-vertex paths. Add direct valid/invalid adjacency and malformed-coordinate
  cases; explicitly choose zero/one-vertex policy. Do not wait for a future
  parser milestone to repair the test oracle or broadly promote broken tests.

  defines: recur.lang.work.remaining.verification.baseline.path.oracle path-contiguity guard and direct tests

<!-- verification: V02 -->
- [ ] Reproduce in a committed regression, then resolve full configured
  completion suffix handling. Locally, `hello.recur` copied from Hold'em stage 1
  plus `demo.holdem.hello.complete.md` returns one header for the default
  `--eventness complete`; after `[status] complete_suffix = '.complete.md'`,
  `--eventness complete.md` returns zero. The source trims leading dots but
  compares against `file_stem()`. Cover default/full suffixes, extensions,
  simultaneous states and parent/sibling isolation across listing and filtering.

  defines: recur.lang.work.remaining.verification.baseline.eventness.suffix configured suffix classification and selection

<!-- verification: V03 -->
- [ ] Reproduce in a committed regression, then resolve the CIR1 list contract
  discrepancy. Installed `recur lang list -d demos/holdem-lab --json` reports
  `recorded_eventness` as four scope objects with empty `records` for stage 6;
  `docs/main.command.lang.query.readme.md` promises `[]` for CIR1. Either conform
  or explicitly revise the contract and all consumers; do not silently weaken
  the assertion to accommodate the current result.

  defines: recur.lang.work.remaining.verification.baseline.cir.eventness CIR list empty-eventness contract

<!-- verification: V04 -->
- [ ] Establish an ordinary PR gate for supported Lang Julia, inspector,
  dogfood and proto-map cases. Current CI runs Rust, trace and the Lang command
  baseline, but not that broader group; `runtests.jl` omits the proto-map suite.
  Commit explicit environments and resolve the CI Julia 1.10 versus web-lab
  Julia 1.12 compatibility requirement and dependencies. Preserve process exit
  status; fail on zero eligible cases; report pass/fail/error/broken/skip
  separately and promote unsupported cases individually when implemented.

  defines: recur.lang.work.remaining.verification.baseline.ci ordinary PR coverage and explicit Julia environments

<!-- verification: V05 -->
- [ ] Recover reproducible focused and full Julia runs after the compiler and
  interpreter faults recorded in the [Hold'em observations](demos/holdem-lab/main.holdem.observations.md).
  Capture runtime versions and logs. A prior 527-assertion success does not
  establish current full-suite or exhaustive poker acceptance.
<!-- verification: V06 -->
- [ ] Run exact-revision gates and refresh evidence immutably. Bind source,
  tests, configuration, runner, policy and results; mutation of an assessed
  input must stale the result. Keep failed and accepted historical attempts.
  Re-query live gates before claiming current acceptance.
<!-- verification: V07 -->
- [ ] Exercise pure commands with in/out-root symlinks or Windows junctions,
  linked roots and executable-binding sentinels: no producer execution, no
  mutation, no outside-root reads; verify text/JSON errors and LANG002 exit 2.

## P1: parser, graph and query contracts

defines: recur.lang.work.remaining.verification.contracts grammar graph and query invariants

<!-- verification: V08 -->
- [ ] Build a requirements-to-case matrix for WIR cardinality, missing/duplicate
  versions/scopes, unclosed blocks, function/flow/event/Warp declarations,
  incorrect ports/dE, E0 equal to Ef, bundles and aliases. Assert exact codes
  and spans, including RLIR001/008/009/011, rather than accepting any error.
<!-- verification: V09 -->
- [ ] Cover CIR missing/duplicate coordination, coordinator/contracts/ports,
  lanes/personas, function input/output/policy, unknown producers, mismatched
  ports, unclosed joins, zero/two forks, empty awaits, unknown consumers and
  orphan producers. Include RCIR001/003/004/006/010 and the remaining contract
  cases. Test duplicate coordinator identities before deciding rejection policy.
<!-- verification: V10 -->
- [ ] Probe fake declarations in comments/quoted strings, escaped quotes,
  braces, LF/CRLF, multiline joins/flows and malformed tokens. These are parser
  hypotheses, not established defects. Define whether repeated/reordered await
  vectors preserve authored order while graph projections use sets.
<!-- verification: V11 -->
- [ ] Add an independent minimal graph oracle for nodes, typed edges,
  projections, waits, consumers, entry points, reachability, schema/hash/spans.
  Cover self, disjoint and overlapping cycles; validate every reported edge
  and shortest cycle per participating node. Distinguish AND readiness from
  reachability; scope filtering must retain global blocking findings.
<!-- verification: V12 -->
- [ ] Complete query cases: ambiguous/missing scopes and multiple flows
  (LANG003/004), missing/non-UTF8 inputs, file roots, configuration and excluded
  paths, schema/source/hash mismatch (LANG009), compact/expanded equivalence,
  parseable JSON, and simultaneous recorded states. Check list/show/report
  success 0, check findings 1 and invalid input 2.
<!-- verification: V13 -->
- [ ] Reject invalid companion argument combinations without mutation:
  confirmation without receipt, recovery without checked mode, checked ID
  misuse and receipt/dry-run combinations.

## P1: checked records and evidence integration

defines: recur.lang.work.remaining.verification.evidence identity corruption recovery and runtime evidence

<!-- verification: V14 -->
- [ ] Write red tests for conflicting published records: accepted record and
  Ef preserved while prepared record has a different scope/attempt; inverse
  corruption; missing prepared record. Decide the contract for each, then
  require confirmation/recovery to reject incompatible evidence without a
  false acknowledgement or mutation. Current selection prefers accepted when
  present; corruption outcomes are source-derived hypotheses, not reproduced.
<!-- verification: V15 -->
- [ ] Test queried status identity against altered `source_hash`, substituted
  absent `before` while true E0 is recreated, identical Ef moved with changed
  `after`, and a reduced `checked_inputs` set. Source inspection shows supplied
  hashes are rechecked, but does not establish exact expected identity and
  assessed-input completeness. Require pure qualification to reject mismatches
  and actor rejection without mutation; prove each candidate with a fixture.

  defines: recur.lang.work.remaining.verification.evidence.status.identity exact source before after and assessed-input identity

<!-- verification: V16 -->
- [ ] Exercise legacy lane escape through linked `.recur/lang`, occupied status
  paths and write failure after rename. Establish E0 restoration, no outside
  write/no false acknowledgement, and explicit ID replay/replacement policy.
  Keep legacy behavior distinct from checked-record immutability.
<!-- verification: V17 -->
- [ ] Test parent qualification across current/stale/failed/missing/declared
  evidence, contract/dependency domains, child complete/incomplete/blocked/
  exploded states and exact public-contract hashes. Include nested/replayed
  same-shape but different-identity children; parent acceptance is independent.
<!-- verification: V18 -->
- [ ] Complete the standalone runtime-evidence loader's pinned-environment gate
  with all case IDs and no unexpected skips/errors. Cover portable `RECUR_BIN`,
  byte/file/association/attempt/case limits at boundaries, canonical deduplication
  and file growth during reads. Assert combined precedence
  malformed > ambiguous > mismatched > failed > stale > declared > absent >
  checked and all contributing reasons; preserve unknown fields, execute no
  producer and never select an attempt merely by timestamp.
<!-- verification: V19 -->
- [ ] Resume the [runtime-evidence Warp](warps/main.lang.runtime-evidence.readme.md)
  in its contracted order: binding before UI, then integration/re-entry with
  fresh gates after shared-input changes. Test opt-in API associations, catalog
  allowlists, client path/command/root rejection, and `/api/lang` empty-evidence
  compatibility. Cover loading/error/stale/mismatch, compact/expanded views,
  desktop/mobile, stop/start/timeouts and restart consistency from disk.
<!-- verification: V20 -->
- [ ] Demonstrate one specialist producer moving mock-to-real while preserving
  Header identity/contracts, Body dependencies and Footer E0/dE/Ef meaning.
  Require independent parent acceptance bound to the child public-contract
  hash. A bounded typed Reveal/Rust flow need not wait for a general coordinator.

## P2: fault boundaries and independent oracles

defines: recur.lang.work.remaining.verification.robustness faults properties and cross-runtime conformance

<!-- verification: V21 -->
- [ ] Extend existing fresh-process transition recovery with syscall-level
  partial-write/sync/hard-link/unlink/cleanup failures. Preserve at least one
  valid state after publication and occupied completed records; no false ACK.
  Do not describe simulated interruption as proof of power-loss durability.
<!-- verification: V22 -->
- [ ] Add graph/BubbleRing properties for layer permutations, equivalent
  duplicate attempts, transitive dependencies, and conflicts never increasing
  coverage. Refresh tests must create correctly named competing successors,
  check attribution, narrowed predecessors, identical-manifest replacements,
  reused-ID reasons and corrupt predecessors; a bad filename alone proves no fork.
<!-- verification: V23 -->
- [ ] Exercise attempt IDs at 80/81 characters, dots/empty/Unicode, cases at
  128/129, duplicate JSON keys, zero/future/overflow timestamps and combined
  diagnostic precedence. Extend the independent Rust/Julia FNV oracle with
  empty/ASCII/Unicode/LF/CRLF/binary vectors.
<!-- verification: V24 -->
- [ ] Verify UTF-8 byte spans against actual source slices and independent
  semantic goldens. Ensure every negative mutation changes its intended
  construct and isolates one fault. Cover Julia field/type/alias/intrinsic/
  binding errors, immutable boundaries and JSON control characters via JSON3.
<!-- verification: V25 -->
- [ ] Add malformed nested inspector data, recursive inventories, timeouts,
  large output, Unicode paths and interrupted navigation where supported.
  Maintain a versioned Rust/Julia supported-feature corpus; retain the case
  where a statically sound graph produces incorrect worker output.
<!-- verification: V26 -->
- [ ] Only after reproducible gates, measure coverage and target useful
  mutations: containment, hashes before analysis, fan-in, unknown ports,
  suppressed findings, exit codes, retry N+1 and resource bounds. Keep seeded
  fuzzing, independent SCC oracles and stress cases in a bounded slow lane;
  minimize failures instead of inventing a coverage percentage target.

## Delivery, guidance and separate milestones

defines: recur.lang.work.remaining.verification.delivery platform gates documentation and milestone boundaries

<!-- verification: V27 -->
- [ ] Run native Linux and Windows tests and extracted-package smoke checks;
  Windows build-only CI is insufficient. Recur validation uses **Cargo and
  Julia, not Python**. Port or replace the existing Python package checker
  before adopting its checks: seven binaries, help/version, archive contents
  and hashes, query semantics, CIR graph and preview purity. Preserve coverage.
  Package verification, local installation and publishing remain separate steps.
<!-- verification: V28 -->
- [ ] Reconcile stale capability/umbrella tables with delivered SGR/query
  behavior and runtime-evidence readiness. Resolve the Julia Recur playbook's
  instruction to automatically delete `.current` beside `.complete`: current
  Eventness guidance requires examining scope/evidence first. Do not delete
  project markers as part of this documentation review.
<!-- verification: V29 -->
- [ ] Audit existing todo trace examples against the configured identifier
  grammar and role keywords. Local discovery truncates hyphenated segments
  and classifies `consumes:` rows as define sites with this project's custom
  keywords. This new focus uses dot-separated words for full identities;
  identifier discovery alone does not verify role classification.
<!-- verification: V30 -->
- [ ] Create separate contracts when selecting advanced work: unified WIR/CIR
  identity and imports; deterministic pure GRID0 snapshots versus watcher health;
  watch readiness before dispatch and bounded recovery; retry/failure routing
  with exhaustion; scatter/gather with associative/commutative/idempotent merges,
  empty identity and cumulative conflict handling; subsystem expansion/collapse.
  Preserve out-of-order/deduplicated events, scope boundaries and drained
  history. These proposals are not all promised for this baseline or release.

## Completion evidence and discovery

defines: recur.lang.work.remaining.verification.acceptance exact-input case ledger and checked residue

<!-- verification: V31 -->
- [ ] Maintain a ledger linking requirement -> contract -> case ID -> runner ->
  platform -> accepted revision and input fingerprints. Detect orphaned cases,
  zero-case gates and completed features still represented only by expected
  failures. Fingerprints establish byte correspondence, not authenticity or a
  complete dependency closure. Record the checked actor's exclusive-operator
  assumption; do not claim hostile-concurrency locking.
<!-- verification: V32 -->
- [ ] Record current command outputs, versions, exit statuses and receipt/gate
  identities for the selected scope. After verified completion, rename only
  that scope's todo to `.todo.checked.md`, update parent links and retain these
  trace IDs. A filename change is not formal Warp acceptance.

```powershell
recur trace-id 'recur.lang.work.remaining.verification.**' --scope 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --format full
recur files 'README.CORE.IMPROVEMENT30.recur-lang.**' -d . --sep .
recur warp show main.lang.runtime-evidence -d .
recur warp show main.lang.checked-transition -d .
```
