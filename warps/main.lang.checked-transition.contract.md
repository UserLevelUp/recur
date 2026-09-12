# Checked Lang transition acceptance contract v1

Date: 2026-09-12. Planning contract. Slice-0 freezes exact wire/CLI semantics
before new behavior tests and implementation; the schema names/options for that
new behavior are intentionally not invented here.

## Ownership and invariants

1. Reuse WIR1 source identities, scoped contracts and aliases; no second parser,
   new language grammar or WIR1/CIR1 lifecycle unification.
2. A shared pure Rust assessment module reuses `src/warp_evidence.rs` for external
   result/fingerprint rules. It adds explicit scope, requirement, attempt and
   transition relevance. `recur lang` may expose those facts without choosing or
   executing work. Opinionated policy and confirmed writes belong to `recur-lang`.
3. Preserve query-v1 packets and the existing companion receipt contract through
   additive opt-in or versioning. Legacy accepted receipts remain identifiable
   as legacy declarations, never silently promoted to checked evidence.
4. External producers own execution. Neither querying evidence nor confirming
   a transition may execute bindings, capsule instructions or test commands.
5. Rejection for absent/invalid/failed/stale evidence cannot advance E0 or replace
   existing Ef/status artifacts. A partial write is never reported as success.
6. Keep observations immutable. New attempts get new identities; preserve old
   source/result observations and interpret applicability separately. Historical
   completion is not erased when its evidence becomes stale, but it cannot imply
   current acceptance. FNV detects changes, not authenticity or dependency closure.
7. Maintain repository-relative live evidence paths beneath the canonical map
   root. No escaping roots, copied-source substitutes or relaxed containment.
8. This bubble has independent acceptance. It neither closes nor waives the
   Julia, HTTP, browser or other gates of main.lang.runtime-evidence.

## slice-0: contract and baseline

Contract ID: `contract:main.lang.checked-transition.slice-0:v1`.
Gate: `contract-and-baseline`, declared planning observations.

Read Improvement 30, current Lang query and companion contracts, existing Rust
parsers/checker/transition writer and their tests. Inspect the runtime-evidence
contract, red observations, unaccepted loader and blocker as prior art. Treat
crash causes as unresolved; five failed configurations are not a root-cause proof.
Inspect actual binaries/help; record executable paths, hashes and source inputs.
Observe focused Rust query/checker/companion tests. Do not rerun a Julia crash
matrix as a prerequisite for freezing this independent Rust capability.

Freeze a linked behavior/wire contract containing:

- Source path/hash, canonical root, exact qualified function and aliases;
  explicit associations to behavioral requirement IDs and required case IDs.
- Attempt, producer/runtime provenance, actual specification/implementation/test/
  configuration/runner inputs, result artifacts, and evidence hashes. Freeze
  trust assumptions for producer-supplied case labels versus checked aggregates.
- Exact E0, dE and Ef, the selected checked receipt/schema or explicit opt-in,
  result/status schema, stable diagnostics and CLI exit codes. Preserve the
  legacy default behavior. Pure and confirmed forms share the same assessment.
- Absent, declared, checked, failed, stale, malformed, mismatched and ambiguous
  outcomes with precedence, retained reasons, and current/history semantics.
- Numeric byte/file/history limits, bounded reads and any subprocess output/time
  limits enforced during collection, not after unbounded buffering. Prefer
  direct Rust calls. Bound transitive references and discovery, including
  acceptance data; a bounded JSON response alone is not a bounded read.
- Mutation boundary: revalidation timing, duplicate-attempt/conflict handling,
  destination collisions, durable status identity, failure stages and a bounded
  interruption-recovery procedure. State filesystem/concurrency guarantees
  honestly; do not promise an atomic snapshot of arbitrary external files.
- A supported evidence refresh/evolution strategy for earlier slices if later
  changes affect their inputs. Existing immutable layers with stale gates can
  block projection; adding another passing layer need not repair that. Resolve
  this before accepting checked slices, without rewriting historical receipts.

Create an explicit requirement-to-test matrix before implementation. Reproduce
behavioral red results at the actual new boundary; the legacy empty-evidence API
alone cannot prove the new evaluator's negative cases. Setup/compilation/runtime
errors are not behavioral red. Exercise actual-input evidence containment using
`recur warp evidence` and the inventory/projection used to resume this bubble.

Acceptance: baseline observations, exercised root, frozen contract, matrix and
resolved implementation/verification boundaries recorded. Creating the bubble
or repeating earlier passing counts is insufficient.

## slice-1: pure Rust assessment and query

Contract ID: `contract:main.lang.checked-transition.slice-1:v1`.
Gate: `assessment-and-query`, checked test evidence, no skips.
Dependency: slice-0.

Write hand-authored positive and adversarial fixtures first; preserve their
expected red observations, then implement the shared assessment and the frozen
additive/versioned pure query. Query output must retain exact header contracts,
body aliases/boundaries and separate footer facts. No policy recommendations or
mutations in core. The inspector need not be present for this to work.

Required tests include exact source/scope/alias/transition/attempt associations;
duplicate IDs and ambiguous local letters; repeated letters across qualified
scopes; unsupported/malformed schemas; missing case IDs and forged labels whose
aggregate result fails; manual/native ACK with invalid or missing test reference;
failed, skipped, zero or inconsistent counts; missing inputs and changed spec,
implementation, tests, config, runner, result and association. Cover Unicode
paths/identities and inert command-like text. Exercise each frozen limit at and
beyond its boundary, including aggregate bytes/files and transitive references.
Reject traversal, absolute/drive/UNC paths and canonical symlink/junction escapes.
Prove query purity against before/after artifacts and a producer-execution trap.

Acceptance: all required cases execute and pass, stable diagnostics and original
packet compatibility verified, existing checker reused, checked manifests bind
actual relevant inputs. No accepted transition is inferred from a checked test.

## slice-2: confirmed checked transition

Contract ID: `contract:main.lang.checked-transition.slice-2:v1`.
Gate: `confirmed-transition`, checked test evidence, no skips.
Dependency: slice-1.

Tests first for one explicit checked-mode `recur-lang` transition. The dry run
shows the same source/scope-bound assessment and intended mutation without writes.
Confirmation revalidates the required inputs at the frozen mutation boundary.
Missing, failed, stale, wrong-scope or malformed evidence preserves E0 and emits
the specified rejection. A current passing assessment plus explicit confirmation
may move only the exact permitted E0 artifact and record a durable checked status.

Test legacy default compatibility, stale/unsupported receipt, source drift
between planning and confirmation, changed evidence before mutation, wrong E0,
out-of-root paths, existing destination/status, duplicate attempts, conflicting
attempts and command-like bindings. Assert exact bytes of unrelated artifacts.
Inject failures at the declared mutation stages; no false ACK and no silent
overwrite. Implement the minimum frozen recovery mechanism, not a live worker
coordinator or arbitrary executor. Keep existing companion tests unchanged.

Acceptance: all required positive and negative process-level tests pass; only
the new explicit checked mode enforces its new schema. Externally observed
tests, current assessment and companion transition ACK remain distinguishable.

## slice-final: drift, interruption and re-entry

Contract ID: `contract:main.lang.checked-transition.slice-final:v1`.
Gates: `drift-and-recovery`, `regression-and-reentry`, checked tests, no skips.
Dependency: slice-2.

Demonstrate one capability end-to-end: exact spec -> observed red -> observed
green -> pure checked assessment -> authorized E0-to-Ef transition. Use isolated
fixtures to change each relevant input and result independently, proving loss
of current acceptance without rewriting the historical successful transition.
Reject passing evidence for another scope. Test recovery after interruption at
every frozen failure stage, deterministic replay and conflicting retries.

Restart processes and reconstruct the same assessment and durable transition
from disk, without chat/process memory. Walk tree/files/trace-id/source trace,
recorded recovery and the selected Lang view. Preserve unsupported coverage and
show header/body/footer identity agreement in compact and expanded output.

Run required focused suites, `cargo test --locked` and scoped/repository clippy
as appropriate; record exact commands, warnings, runtimes and inputs. Relevant
Rust cases must execute, not be ignored or replaced by mock-only checks. Julia
may supply independent optional observations; its unresolved runtime/browser
gates belong to the existing bubble and are not claimed by this acceptance.
If delivered changes invalidate legacy Julia-facing behavior, investigate and
record that regression instead of using the independent scope to dismiss it.

Produce structured actual results and explicit fingerprints under
`warps/checked-transition/`. Reassess earlier gates after shared inputs change;
use the strategy frozen in slice-0 or record a blocker. Final acceptance requires
the live projection to agree, without stale/conflicting accepted layers. Update
delivered query/companion contracts, expert guidance and reveal handoff to explain
observed behavior, limitations and how the inspector can consume it later.

## Non-goals and execution scope

No new grammar, general code execution, automatic test generation, live
coordination, watcher/grid implementation, Julia runtime repair, web inspector
integration, or closing unrelated Warps. No commits, pushes, publishing,
installation or unrelated cleanup. This creation turn writes planning artifacts
only; later implementation follows one dependency-ready slice at a time.

defines: main.lang.checked-transition.contract.v1 acceptance and ownership
consumes: recur.lang.query.v1 existing exact query contract
consumes: recur.warp.evidence.integrity.external existing result and input checks
produces: main.lang.checked-transition.tests required behavioral evidence
produces: main.lang.checked-transition.acceptance qualified transition acceptance
