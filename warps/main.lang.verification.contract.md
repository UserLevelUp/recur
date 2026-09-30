# Lang verification: tests before implementation

defines: recur.lang.verification.v1 baseline demonstrations and bounded repairs
consumes: recur.lang.work.remaining.verification the complete verification backlog
consumes: recur.lang.verification.tests executable fault demonstrations
produces: recur.lang.verification.observations actual results without implied acceptance

Status: tests-first plan. This Warp owns verification backlog V01-V32; it does
not implement `recur-lang init`, replace existing runtime-evidence/refresh
acceptance, or introduce advanced language execution.

## Inputs and demonstrations

The [ledger](../demos/lang-verification/main.lang.verification.ledger.json) maps
every checkbox in the [verification todo](../README.CORE.IMPROVEMENT30.recur-lang.verification.todo.md)
to new executable cases, existing source/test artifacts, remaining test design
and an owning slice. It is intentionally explicit about incomplete coverage.
The executable suite is `julia-tests/main.command.lang.verification.test.jl`.
The [demo launcher](../demos/lang-verification/main.lang.verification.jl) lists
cases, explains a todo, runs a prefix or dispatches an existing Julia suite.

Each case uses temporary fixtures, asserts positive controls and exact behavior,
and distinguishes assertion failures from exceptions. Mutations must change the
intended construct. Red cases are ordinary failing assertions, not skipped or
`@test_broken` successes. The runner exits nonzero for failed/error/broken/empty
cases and refuses an empty selection or an existing observation output path.
Do not include this standalone suite in the normal regression runner until green.

The suite operates on explicit `RECUR_BIN` and `RECUR_LANG_BIN`. Record their
hashes, actual Julia executable/version, platform and test input hashes. The
default `julia` command may differ from Juliaup on this machine. A process crash
without a complete observation is a failed validation attempt, not an expected
product failure. Synthetic passing receipts are confined to temporary fixtures
for testing the evidence reader/actor; they never certify this repository.

## Frozen repair expectations

1. **Queries:** full configured suffixes with leading dots and `.md` extensions
   classify the same exact authored Eventness identity and can be selected using
   the normalized configured vocabulary. WIR simultaneous current/complete files
   remain observable without mutation. CIR `recorded_eventness` is exactly `[]`
   as documented. Preserve LANG002 containment, error exits and query purity.
2. **Path oracle:** for this nontrivial route helper, every path has at least two
   valid coordinate vertices and every adjacent pair is an undirected declared
   corridor. Reject missing/empty/singleton paths. This is an explicit choice for
   the helper; it does not claim all graph APIs must reject zero-length routes.
   Tests execute the current helper definitions without running unrelated future
   path-language tests. Extract to a shared helper when fixing it and update the
   tests to import it; never duplicate a corrected oracle only inside tests.
3. **Parser/graph:** comments cannot create scopes/functions or alter semantic
   discovery. Preserve original UTF-8 byte spans and LF/CRLF behavior. Retain the
   documented LANG004 ambiguity versus LANG006 parse-error boundary. Keep exact
   graph edges, node kinds, AND waits, reachability and global findings under
   scope filtering. Every reported dependency cycle consists of actual edges.
   The remaining WIR/CIR diagnostic and lexical matrix is required before closing
   V08-V12, not implied by the initial targeted cases.
4. **Status identity:** current accepted status binds the exact assessed source
   hash, requested before/after identities and full assessed-input map. Substituted
   absent `before`, moved identical Ef with rewritten `after`, or omitted inputs
   must not qualify. Pure queries and rejected actor calls preserve all bytes.
5. **Published records:** when both prepared and accepted records exist they
   describe the same transaction, allowing only the state distinction. A corrupt
   or mismatched prepared record cannot be hidden by an accepted record. Reject
   confirmation/recovery with no acknowledgement and no mutation. Add tests for
   missing prepared history before deciding whether compatible accepted-only
   replay remains valid; this plan does not silently forbid existing recovery.
6. **Evidence and delivery:** retain role-by-role drift checks and independent
   parent acceptance. Run existing loader tests, then add missing boundary and
   integration cases through the existing runtime-evidence Warp. Keep Cargo/Julia
   validation, exact package contents and native Windows/Linux behavior; port
   existing Python checks before executing equivalents for Recur.

## Implementation slices

| Slice | Tests/demo first | Implementation and completion boundary |
| --- | --- | --- |
| slice-0 | `catalog.*`, full observed red baseline, inspect ledger gaps | Review fixtures/expectations, record immutable baseline; no product acceptance |
| slice-query | `query.*`, `actor.arguments.*`, existing `tests/lang_query.rs` | Repair suffix/list behavior and complete input/purity matrix in core query/companion argument handling |
| slice-path | `path.oracle.*` | Fix shared path oracle, promote only applicable assertions |
| slice-parser | `parser.*`, `graph.*`, existing Rust IR/graph/query tests | Extend missing exact-code and lexical cases before parser changes, then preserve byte spans and graph semantics |
| slice-status | `checked.status.*`, `checked.drift.*`, positive accepted control | Tighten pure status qualification and exact assessed-input identity |
| slice-records | `checked.conflict.*`, existing re-entry/recovery tests, new legacy fault cases | Check all published transaction records; preserve valid recovery and legacy rollback |
| slice-evidence-tests | Existing loader/API/parent tests plus V17-V20/V25 matrix | Freeze additional red cases for limits, precedence, parent hash and mock/real integration before extending implementation |
| slice-evidence | Preceding matrix, loader, API, inspector/dogfood, specialist demo | Reuse existing runtime-evidence binding -> UI -> integration order; parent integration acceptance is separate |
| slice-robustness-tests | V21-V23/V26 deterministic faults, properties and boundaries | Add injection hooks only as test seams; establish their failure or control baseline before behavioral fixes |
| slice-robustness | Preceding cases and existing Cargo transition/refresh bounds | Repair proven faults, retain history; bounded slow-lane fuzzing/coverage has no invented percentage threshold |
| slice-delivery-tests | V04/V05/V27-V29 runner failure/zero-case fixtures, package fixtures, guidance/trace cases | Freeze runner/package/CI expectations; actual native CI remains required |
| slice-delivery | `delivery.*`, preceding tests, native and package suites | Add explicit Julia environment/PR coverage and Cargo/Julia package checks, reconcile guidance/trace examples |
| slice-final | All selected cases and affected legacy suites; live full-root gates | Record exact candidate, immutable fresh evidence and separate integration acceptance |

Only slice-0 is initially current. Query/path/parser/status slices may be chosen
after its review; slice-records depends on status identity. Test-design slices
are explicit prerequisites for their larger implementation slices. Do not
publish all test-design gates as accepted merely because a ledger exists.

Before editing product behavior in any slice: run its existing cases, add the
missing cases listed in the ledger, review expected outcomes against contracts,
then retain an observed baseline. Run those cases after the change and the
affected legacy suites. Never weaken assertions to manufacture green results.
If a contract changes, evolve it explicitly; do not refresh evidence under an
unchanged contract label. Keep this new Warp's acceptance distinct from older
checked-transition, runtime-evidence and refresh bubbles.

## Advanced work and companion init

`main.lang.init` retains its own contract and red suite. Use `suite init` to
demonstrate it. Planner/scaffolding/hidden-implementation-dependency proposals
remain explicitly outside this bounded baseline.

Advanced feature Warps live under `main.lang.advanced.*`: identity, grid, watch,
retry, scatter and imports. Each starts with its own contract and tests-first
gates. Their initial plans define questions and required demonstrations, not
unreviewed executable grammar or guaranteed release scope.

## Evidence qualification

Checked implementation gates require real nonempty test results with no failures,
errors or skipped required cases and fingerprints for the assessed inputs. The
initial observation JSON is not a `warp-external-evidence-v1` manifest and cannot
be submitted directly as one. Wrap actual green results only after recording the
complete selected input closure, platform and runner. Preserve all failed runs.
No `.todo.checked.md` rename or acceptance layer is authorized by a green catalog
or this planning work alone.
