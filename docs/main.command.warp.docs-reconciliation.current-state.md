# Warp: audited implementation and proposal boundary

Follow-up 2026-09-06: [companion policy foundation](main.command.warp.companion-policy.readme.md)
resolves the suffix-policy mismatch described in this baseline audit. The historical
finding below explains why the follow-up exists; it is not an unfixed current claim.
Generalized transition policies remain proposed. Original audit receipts retain
their baseline source binding.

Audit date: 2026-09-06. Source baseline: `837dee4` on `recur-lang` and
`a.0.2.8`; this is branch/source publication, not proof of a package-registry
release or the version installed on another machine. This reconciliation changes
documentation/tests only. The [claim matrix](main.command.warp.docs-reconciliation.claims.json)
binds each classification to concrete source, test and documentation excerpts.

## Current capabilities

| Family | Audited scope | Boundary |
|---|---|---|
| scoring-config | Suffix grouping, fixed residual weights, policy provenance | Arbitrary configured scoring weights/signals are not implemented |
| status-explain-next | Pure verdict, evidence and suggested-action projections | Suggestions do not execute or authorize work; not a scheduler |
| maps-rings | Flat maps and explicit recursive ring schemas, dependencies and parent acceptance | Directory nesting alone is not ring ownership; subscriptions describe/check receipts, not start watchers |
| complete-receipt | Confirmed accepted layers and policy-aware lifecycle declarations | A receipt is not independent verification; gates belong to the map |
| evolve-collapse | Exploded-bubble supersession and confirmation-gated archival | Not general autonomous goal optimization; query/actor collapse policy is not fully equivalent |
| evidence-freshness | Declared versus checked external evidence, result/input checksums | Only declared inputs checked; no producer rerun or cryptographic attestation |
| reveal | Opt-in structured state/readiness reconciliation | Does not rewrite narrative or infer unrecorded intent |
| discovery | Bare `recur warp` / explicit list; configurable roots and exclusions | Inventory uses manifest-local evidence; explicit merge retains its older scope |
| milestones-temporal | Design vocabulary and examples | No general epic/horizon forecasting runtime |
| methodology | Bounded slices, evidence, ownership, recoverable reasoning | Human coordination model, not proof that every design idea is implemented |

## Configuration: already real, but not uniform

`src/warp_policy.rs` resolves the nearest ancestor `.recur/config.toml`, validates
`[warp.suffixes]`, reports field provenance and recognizes the longest compound
suffix. Queries and the lifecycle receipt writer use this policy. `receipt` uses
the first configured completion suffix but records a **declaration**, not a proof.

Discovery separately reads `[warp.discovery]` roots/exclusions. Completion derives
dependencies, evidence mode and gate rules from the Warp map. Evolution requires
an exploded source and matching carried Slice contracts; it is not a configurable
general transition engine. Scoring weights are constants in the current scorer,
not arbitrary values loaded from the configuration examples in the old proposal.

Historical mismatch at 837dee4 (resolved by the follow-up): `collapse_suffix_policy` read only
`<invocation-root>/.recur/config.toml`, not nearest-ancestor policy. Collapse takes
the final dot-delimited filename token, rather than longest compound suffix, and
uses a separate permissive parser. Thus previewing core `collapse-plan` is not
a guarantee the companion will classify every custom/nested policy identically.
Use the companion's own dry run and inspect it before any authorized confirmation.
The reconciliation itself did not fix that behavior; the follow-up adds executable
parity and refusal tests and delegates that helper to shared WarpPolicy.

## Proposed next implementation, not a declared or completed Warp

Configuration-driven `recur-warp` policies should start with tests for that concrete
parity gap: configuration discovery, validation, compound suffixes and preview/actor
agreement. Then define permitted transitions, required evidence, bounded writes,
retries and escalation for a small opinionated action. Preserve compatibility or
explicitly agree a migration; do not silently replace defaults.

Core Recur exposes policy, hierarchy and state through queries. A companion applies
policy only within authorized scope. Configuration does not grant itself authority;
`next` advice is not permission. Shared conventions may later support other trait
companions, but no universal policy schema or arbitrary execution service is shipped
by this reconciliation. Capability discovery should expose prerequisites, effects,
limits and evidence, not merely command names.

## Ownership and historical material

Improvement 27 owns Warp methodology and its query/companion boundary. Improvement
29's proposed `recur-reveal` companion is separate from the implemented core
`recur reveal`. Improvement 30 owns Recur Lang's semantic relationships, not Warp.
Neither proposal is an automatic next task or accepted implementation requirement.

Root documents retain the original control/temporal/cross-domain rationale under
explicit design labels. Supporting future-plan notes retain their historical text
with a current split at the top. Their future-plan suffixes refer to remaining
proposals, not to absence of today's commands. The old differential execution
trajectory is archived as historical rather than retained as active unfinished work.
Historical receipts and private `.recur` capsules are not rewritten.

Current operating references: [commands](main.command.warp.readme.md),
[evidence](main.command.warp.evidence.readme.md),
[reconciliation tests](main.command.warp.docs-reconciliation.tests.md).

defines: recur.warp.docs.reconciliation.current evidence-backed current implementation boundary
defines: recur.warp.docs.reconciliation.policy-gap separate collapse policy differs from shared query policy
produces: recur.warp.docs.reconciliation.next proposed configuration-driven companion policy tests before implementation
