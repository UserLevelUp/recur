# Evidence refresh v1: frozen tests-first contract

Refresh current source-stale gate evidence under the same accepted contract.
Keep map, accepted layer, old manifests and results immutable. This is a
same-contract evidence operation owned by recur-warp; it is not Warp evolution,
new acceptance, a producer runner, Lang grammar, or automatic coordination.

CLI: `recur-warp refresh MAP LAYER --gate G --from evidence:OLD --to evidence:NEW
--refresh-id ID --reason TEXT [-d ROOT] [--confirm] [--json]`.
MAP and LAYER are exact root-relative files. Preview is read-only. Success exits
0; invalid inputs, rejected evidence, conflicts and IO errors exit 2. Confirm
publishes one immutable JSON receipt. Identical replay returns idempotent only
while replacement evidence remains checked. No retry selects history by time.

Receipt schema warp-evidence-refresh-v1 binds refresh ID, map path, Warp and
bubble identity, complete required-slice policy fingerprint, exact layer path
and bytes, gate, old/new reference and manifest fingerprints, and explicit reason.
IDs: 1..80 ASCII letters/digits/._-, excluding . and ... All paths use forward
slashes with no absolute/drive/UNC/backslash/NUL/dot/dot-dot/empty components;
existing symlink/reparse ancestors are refused for refresh reads and writes.

Receipts live beside the map in WARP.evidence-refresh/ID.json. At most 64 entries
(including staging) are examined there; unexpected files fail closed. JSON/map/
layer/receipt reads <=2 MiB each, other evidence inputs <=32 MiB each; <=128
canonical files and <=64 MiB unique bytes per refresh assessment. These limits
are checked while reading. Existing broad Warp discovery retains its own scope;
refresh adds no whole-root scan. Snapshots cannot supply current evidence.

Only an accepted layer with nonempty result hash and matching required contract
can be refreshed. Gate must exist in both policy and layer. Original evidence
must be external, previously passing with an unchanged result artifact, and
currently stale only through source input drift. Malformed, failing, manual,
missing-result, changed-result, skipped-disallowed or contract-stale observations
cannot be laundered. New evidence must be checked under the unchanged gate rule,
same evidence kind and project, and retain every predecessor source path; scope
may grow. No role/path removal, grammar change or weakened gate policy is inferred.
Explicit reason and producer metadata are auditable claims, not authentication.

A refresh graph is scoped to one accepted layer/gate. Its single chain begins
at an original reference. One successor per reference, no cycles, no orphan
records, no copied conflicting IDs, <=64 steps. Each edge binds immutable
predecessor/replacement manifest bytes. Queries check every historical result
and each scope inclusion; current freshness comes only from the leaf's actual
inputs. Historical source drift remains visible in reasons; historical results
are never reclassified as newly executed. Other references/layers, conflicts,
contract mismatches and unmet dependencies continue to block as before.

Queries use the same resolver in show/merge/list/ring projection and retain
effective evidence reference, original reference and refresh IDs in assessment
reasons. Existing schemas and no-refresh behavior remain compatible. Bare
`warp evidence OLD` still reports OLD's current staleness; it has no gate context.
Recorded lifecycle Markdown and Lang status schemas are not refreshed by this v1.

Before writing, revalidate the complete plan/read-set and receipt-directory
inventory. Use create-new staging, sync, no-clobber publication, then cleanup.
No existing receipt is overwritten. A complete matching staging file may be
republished on identical confirmed retry; torn/conflicting staging blocks with
its exact path. Publication interruption never emits false success. Exclusive
operator ownership is assumed for the short write; no hostile-concurrency or
universal power-loss guarantee. No install, producer execution or source copying.

## Tests specified before behavior

- Fresh baseline -> source drift -> preview -> confirmed refresh -> fresh-process
  query complete; original map/layer/manifest/result bytes identical.
- Red/zero/skipped/malformed/missing/changed results, manual evidence, wrong
  gate/layer/contract/project/kind, reduced source scope: reject without writes.
- Subsequent source drift, second refresh, deterministic replay, fork/orphan/
  cycle/conflicting receipt, altered historical bytes: fail closed as applicable.
- Other stale references and dependency/contract/result conflicts still block.
- Root traversal, junction escape, byte/file/history boundaries, dry purity,
  prepublication drift and interrupted publication/retry.
- Existing Cargo regression, actual old-Warp refresh after truthful reruns,
  current re-entry and unchanged receipt hashes; commit and fast-forward checks.

## Slices and acceptance

slice-0: complete this plan, callable tests and preserved expected red observations;
declared gate contract-and-red. A compile or missing-command error is not red.
slice-1: implement resolver/actor and run all specified new tests; checked gate
refresh-behavior. No acceptance before green observations.
slice-final: full regression, guidance, refresh stale affected earlier gates with
actual observed results, verify projections, commit and fast-forward a.0.2.8;
checked regression gate. Git publication is reported separately from test gates.

Changing prior accepted inputs is explicitly authorized by this refresh task.
Their historical receipts remain immutable; apply this tested explicit renewal
mechanism after current tests pass. Own baseline is declared; own checked gates
freeze final code inputs. Preserve line endings across Git so content hashes
refer to checkout bytes; test the staged/committed content against manifests.

defines: recur.warp.evidence.refresh.v1 explicit same-contract renewal
consumes: recur.warp.evidence.integrity.external shared external checker
