# Lang runtime-evidence contract v1

Status: planned. Date: 2026-09-12. Paths are repository-relative unless an
explicit evidence root is stated. This contract governs a bounded WIR1 evidence
integration, not a general execution engine. Slice-0 must freeze the concrete
association/API schema before slice-1 fixtures and implementation begin.

## Invariants

1. Follow hierarchy, artifact selection, declared lineage, source traces and
   recorded context into a selected Lang scope. Lang supplies the highest local
   specificity, not a requirement to formalize the whole repository.
2. Reuse WIR1/CIR1 query models as applicable and `src/warp_evidence.rs` for
   supported external-result checks. This Warp implements WIR1 presentation only;
   no second parser or broader validation claim is permitted.
3. Header contracts, body aliases/boundaries and footer state/evidence retain
   canonical source and scope identities. Behavioral prose needs explicit tests;
   it is not executed or proved by parsing Lang.
4. Keep declared Eventness, static validation, observed test runs, assessed input
   freshness and accepted Warp coverage separate. Desired Ef is not observed Ef.
5. Reading a report does not run tests/bindings, dispatch work, mutate receipts
   or complete slices. Opinionated Lang actions belong to `recur-lang`; external
   runners produce observations, and authorized writers record acceptance.
6. Preserve existing greeting/server/inspector behavior and legacy assertions.
   The current API promises `observed_evidence=[]`; new evidence requires an
   explicit additive opt-in or versioned contract, not an unnoticed default change.
7. FNV fingerprints detect content changes, not authorship or cryptographic trust.
   Checked results do not prove test sufficiency, dependency closure or all behavior.

## Initial and final states

E0: bounded Lang query/report works; the inspector has no runtime-evidence
loader. The native Lang receipt loader validates identity fields but only checks
that `artifact` and `test_receipt` are nonempty. Existing declared completion and
that ACK alone do not establish checked test acceptance.

Ef: one new tests-first capability has an explicit source/scope-to-slice-to-test
association, bounded read-only evidence assessment, a clear compact/expanded
presentation and reproducible current-versus-historical evidence. Its acceptance
requires current checked final gates, not just a renamed Eventness artifact.

Expand current attention for unresolved questions and expected red behavior.
Collapse only after acceptance into verification, decisions, limitations and a
rehydration pointer. Preserve historical receipts; do not overwrite a red run,
rename a failed result to passed, or reclassify stale evidence by changing hashes.

## slice-0: baseline and contract

Contract ID: `contract:main.lang.runtime-evidence.slice-0:v1`.
Gate: `baseline-and-contract`, declared planning evidence.

- Read Improvement 30, the existing query/API contracts and the dogfood gap
  assessment. Retain their distinction between implemented fragments and future
  GRID0/COORD work. Do not treat their older roadmap cursor as live state.
- Observe focused Lang query/evidence checks and the applicable Julia inspector,
  server, greeting and API suites using a recorded runtime/binary. Reassess
  `main.web-lab` stale evidence without rewriting its historical receipts. Record
  unrelated failures and the exact shared behavior required for this Warp.
- Freeze a linked, versioned association contract: Lang source identity/hash,
  qualified scope and aliases, Warp/slice/contract identity, required gate/test
  case IDs, E0/dE/Ef, attempt identity, producer/runtime and result references.
  Bind relevant specification, implementation, tests, configuration and runner
  inputs explicitly. Names, timestamps or equal field shapes cannot infer links.
- Define absent, declared, checked, stale, failed, malformed, mismatched and
  ambiguous evidence outcomes and their API status/display behavior. These are
  evidence classifications, not universal Eventness suffixes.
- Choose an additive opt-in or versioned API surface preserving the original
  packet, its `execution: not-run`, coverage, findings and unknown fields. Freeze
  error behavior, byte/file limits, history selection and canonical path handling.
  Requests choose server-owned catalog IDs/scopes, never roots, binaries or commands.

### Evidence-root decision is a prerequisite

Warp inventory evaluates evidence relative to the map directory. This new map
is initially under `warps/`; `../src` and `../demos` are not valid contained
evidence paths there. Existing checked helpers do not implement a per-reference
escape or an arbitrary cross-root override.

Before accepting this slice, record and exercise a supported layout that binds
the intended actual inputs, not just copies. If the map needs relocation to a
common project/evidence root, specify the exact move and matching commands first,
preserve its UUID and links, and keep only one live map. Do not modify global
creation defaults or weaken containment. Product evidence roots remain fixed by
the selected catalog contract, independently of this development Warp's root.
Immutable snapshots may explain historical runs but cannot silently stand in for
the live implementation when claiming current acceptance. Verify the chosen
layout with both `recur warp evidence` and the inventory/projection used to resume.

## slice-1: specify tests before implementation

Contract ID: `contract:main.lang.runtime-evidence.slice-1:v1`.
Gate: `red-first-fixtures`, declared observations, never a failed passing gate.

- Write the compact Lang specification and linked behavioral contract first.
  Map every acceptance requirement to stable case IDs and hand-authored fixtures.
  Identify expected results independently of the future loader implementation.
- Exercise absent and declared-only references; wrong source/scope/alias/slice/
  contract; malformed or ambiguous associations; changed source/test/config/result;
  invalid counts, failed/skipped/zero tests; traversal, symlink escapes and limits.
- Cover a recorded complete marker and a native Lang accepted receipt whose test
  reference is missing or invalid: neither can become checked evidence. Keep the
  legacy companion contract unchanged unless a separately specified change is
  needed; these references are assessed at the new association boundary.
- Record expected failing assertions before implementation, with source/test
  fingerprints and commands. Setup, compilation or missing-tool errors are not
  behavioral red evidence. A missing entry point alone is not the full red suite.
- Retain red observations as historical evidence tied to E0. Keep intentionally
  red suites outside the normal green runner until implementation is available.

## slice-2: bind and assess evidence

Contract ID: `contract:main.lang.runtime-evidence.slice-2:v1`.
Gate: `scope-evidence-binding`, checked test evidence; no skipped tests allowed.

Implement the frozen association/loader boundary and make slice-1 cases pass
without weakening assertions. Reuse the existing checker via a supported code
or argument-vector CLI boundary; do not duplicate its fingerprint/result rules.
Association checks add scope/contract relevance that generic file checking lacks.
Retain structured reasons, explicit checked input scope and producer-not-rerun
qualification. Keep valid history separate from current-source assessment.

Tests must compare source/config/receipt bytes before and after queries and
verify that no binding, test runner or capsule command executes. Tests for native
Lang receipt handling must demonstrate the legacy ACK is not sufficient proof.
Serialize new data outside the unchanged query-v1 packet unless a separately
reviewed versioned query contract is required by an observed gap.

## slice-3: present precise contracts and evidence

Contract ID: `contract:main.lang.runtime-evidence.slice-3:v1`.
Gate: `inspector-evidence-presentation`, checked tests; no skipped tests allowed.

Integrate the bounded loader into the frozen API surface and existing inspector.
Show exact header contracts and linked requirements; compact body composition,
canonical aliases and boundary edges; footer E0/dE/Ef with separately labeled
recorded state, static findings, observed red/green attempts, current assessment
and Warp acceptance. Unsupported and unvalidated semantics remain visible.

Run real loopback HTTP and browser checks. Include empty/loading/error states,
stale and mismatched evidence, repeated local letters and inert untrusted text.
Compact and expanded views must agree on identity, relationships and verdicts.
Check desktop/mobile layout and preserve the original no-evidence API path.

## slice-final: acceptance, drift and re-entry

Contract ID: `contract:main.lang.runtime-evidence.slice-final:v1`.
Gates: `integration-regression`, `reentry-and-staleness`; checked tests, no skips.

- Demonstrate E0 specification, expected red tests, implementation green, checked
  evidence and authorized Ef acceptance for one capability. Retain both source
  baselines and the unchanged requirements or explicitly reviewed revisions.
- In an isolated fixture, mutate each relevant input and a result artifact;
  re-query and demonstrate lost current acceptance. Reject a wrong-scope receipt
  even if it points to passing results. Do not mutate shared historical evidence.
- Restart the query/inspector and reconstruct the same report from disk without
  chat or process memory. Walk from tree/files/trace-id/source trace/context
  recovery to the exact Lang view and its source-bound observations.
- Rerun affected suites and the normal Julia runner; run Cargo tests/clippy if
  Rust is changed. Record expected-broken cases distinctly from skipped tests.
  Relevant test gates must contain executed passing cases, not a claim that zero
  unexpected failures means all required tests ran.
- Publish structured results and explicit input fingerprints in the layout
  frozen by slice-0. Recheck earlier slice evidence after shared inputs change;
  use supported retry/evolution policy rather than overwriting or accumulating
  conflicting accepted layers. No completion until the live projection agrees.
- Leave verification, remaining limitations and a concise reveal handoff. Update
  Improvement 30/expert guidance for observed delivered behavior, not promises.

## Non-goals

No new general Lang grammar, unified WIR1/CIR1 lifecycle, GRID0/live coordinator,
automatic test generation, worker execution, `recall` implementation, arbitrary
file access, release publishing, Git commit/push, installation or cleanup. Closing
unrelated Warps is not an implicit requirement. If these become necessary, stop
at the affected gate and propose an explicit scope revision.

defines: main.lang.runtime-evidence.contract.v1 bounded specification and evidence gates
consumes: recur.lang.query.v1 exact contracts and reported coverage
consumes: recur.warp.evidence.integrity.external structured result and scoped input checks
produces: main.lang.runtime-evidence.tests required tests-first acceptance cases
produces: main.lang.runtime-evidence.acceptance qualified E0 to Ef observations