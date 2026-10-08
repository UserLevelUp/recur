# Exact binary evidence and immutable renewal

artifact.type = contract
defines: main.command.watch.eventness.evidence.contract
consumer: main.command.watch.eventness.evidence.binary.identity
version = 1
branch = fix/watch-evidence-binary-hashes

Fix the verification workflow so an executable rebuilt by Cargo cannot silently
replace the executable whose behavior was observed. Retain exact tested artifacts,
record reproducible provenance, and renew the original Watch Warp's evidence with
its full predecessor source scope. Whole-file executable hashes remain integrity
identities, even when a harmless Windows linker metadata change produces a new one.

This work is a separate repair bubble linked to `main.command.watch.eventness`;
its creation does not accept or modify that bubble's historical layers.

The desired residue is predicted in
`warps/watch-evidence/main.command.watch.eventness.evidence.final.predicted.md`.
That forecast describes this single Warp's attainable outcome. Revise it as the
work reveals what is possible, until predicted final Eventness matches attainable
reality. Each Warp has its own prediction. Acceptance uses the observed criteria
below; a changed prediction is not proof of completion. Reassess any corresponding
contract changes explicitly while preserving truthful observations.

A Warp may fail. Understanding why it failed is a useful outcome even when its
predicted product is not attained. Initial and final Eventness describe the
predicted states for this Warp's transition; its slices attempt to produce that
final product. Revise the prediction as findings reveal attainable reality.

When the Warp fails, retain the failed slice or integration point, expected and
observed behavior, supporting evidence, the cause where established, and remaining
uncertainty. Distinguish observations from suspected causes. These findings supply
clues for predicting the next Warp's initial and final Eventness. Record failure
honestly; a useful explanation does not turn failed acceptance checks into passes.

## Invariants

- Eventness retains findings, commands and results. Optional Watch wake-ups carry
  trace IDs only. Watch, trace IDs, Warp metadata and Lang remain optional.
- Core queries are read-only; companions and test runners own publication policy.
- Never normalize PE metadata to make an old executable hash match a new artifact.
  Identical source bytes or matching machine code do not establish artifact identity.
- Historical bundles, accepted layers and refresh receipts remain immutable.
- Failed, skipped, missing or incomplete checks cannot be accepted as passing.
- Demo suites run only when selected; this repair does not select a demo.
- Configuration remains optional. Preserve default limits and existing behavior;
  any larger assessment scope requires an explicit, validated, bounded selection.
- Produced evidence does not automatically accept either Warp or authorize dispatch.

## initial: diagnosis and scope

Gate `diagnosis-contract` is a declared review of this contract and the recorded
rebuild experiment. The original binary is unavailable, so no claim about its
exact changed bytes is permitted. The reproduced two-build metadata difference
and Cargo build/test dependency difference remain distinct findings.

## tests: regression contract

Gate `negative-controls` requires executed tests proving these failures are caught:

1. A selected executable changes after verification starts, including a change
   limited to PE metadata. A successful child exit must not produce passing evidence.
2. Existing evidence bundle names, partial bundles and conflicting contents are
   refused or recovered only through an explicitly documented bounded protocol.
3. Changing Cargo output after snapshotting cannot change the retained artifact;
   mutating the retained artifact fails its exact integrity check.
4. Missing binaries, invalid bundle paths, containment escapes, invalid budgets,
   missing input files, source drift and failed results fail closed.
5. Build-producing Rust checks and native behavior checks have distinct provenance;
   the latter execute the exact selected core, companion and fault-hook artifacts.
6. A refresh retaining at least all 265 predecessor input paths works under an
   explicitly selected sufficient limit, while the legacy 128-file default and
   current per-file, aggregate-byte, receipt-count and publication checks hold.

Expected-red baseline observations must be saved before implementation. Planned
tests are not observed tests. A narrow scaffold check is not sufficient evidence
for these behaviors.

## implementation: exact binary bundle

Gate `binary-bundle-tests` requires regression coverage for immutable creation,
artifact copying, exact SHA-256 integrity, mutation detection and execution binding.

Run build-producing checks first. Retain the selected executables outside mutable
Cargo output, and explicitly use those paths for native and Julia behavior checks.
Capture binary hashes before and after the checks. Record source inputs including
`build.rs` and any applicable local build configuration, compiler/Cargo identities,
target, profile, feature selection, commands, outcomes and timestamps. Keep normal
and isolated fault-hook artifacts distinct. Bundle layout is an implementation
choice; local ignored binaries must have an explicit retention/export limitation.

A fresh prefix alone does not fix artifact retention or run-time binding. Do not
rewrite v6 manifests to compensate for an absent original executable.

## integration: bounded full-scope refresh

Gate `full-scope-refresh-tests` requires end-to-end refresh coverage under the
selected scope and unchanged-contract acceptance rules.

The old Watch manifests contain 265 source inputs. A replacement retains every
predecessor path; narrowing the scope is forbidden. Design an optional bounded
scope policy or another equivalent bounded solution. Preserve the 128-file default,
2 MiB JSON / 32 MiB other-file limits, 64 MiB aggregate limit and 64 receipt limit.
Publication revalidation must use the same selected scope as planning. Read-only
show/list/merge must agree with confirmed writer results. Invalid selection and
out-of-root scope fail closed. Preserve original accepted layer bytes and receipts.

Do not claim the proposed larger scope is implemented merely because the map or
this contract mentions it. A changed contract is not eligible for same-contract
refresh; use the repository's separately supported assessment path when needed.

## final: repair acceptance and promotion readiness

Gate `renewed-watch-acceptance` requires fresh source-bound observations from Rust,
selected Julia compatibility checks, native/integration Node tests and negative
controls, with no failed or skipped checks in the selected acceptance suites.

Review and explicitly accept each child slice. Renew all original affected gate
references truthfully, preserving their history; re-query original and repair
Warp show/list/merge at the project root. Both must be ready before promotion.
Record test counts, limitations and pending work. Save a Recur Git snapshot before
publishing branch changes. Commit and normal push/fast-forward remain subject to
the user's publication instruction and successful live gates. No force push.
