# Reconciliation verification — 2026-09-06

Baseline: `837dee48bdd5b2f4fd5e1a0692c5629fca92c2e5`. Initial local changes were
the requested red-first tests, notes and Slice 0 pointer; preserved and extended.
No production Rust/Cargo change, install, commit, push or private capsule edit.
Root `.recur-warp` is a public capability card.

Reviewed both root documents, six supporting notes, capability card, command and
evidence guides, policy/scorer/schema/evidence/companion sources and focused tests.
The ten-family matrix binds claims to source/test/doc excerpts. Current summaries
separate existing capabilities from historical temporal/epic/routing proposals.
The differential execution trajectory was preserved as `.historical.md`, not
declared wholly implemented. Private checkpoints retain historical references.

Concrete remaining gap: companion collapse uses root-local configuration and
final-token suffix parsing, unlike shared nearest-config validated policy and
compound suffix matching. No runtime fix or generalized policy engine is claimed.

## Executed checks

Initial red-first result: 10 passed, 4 failed. Final expanded reconciliation suite:
49 passed, zero failed/errors/broken. No failures converted to expected-broken.
The suite is now included in runtests.jl; local links, historical boundaries and
retired markers are checked in addition to evidence references and negative cases.

Each ran successfully with `julia julia-tests/<filename>`:

| File | Passed |
|---|---:|
| main.command.warp.test.jl | 92 |
| main.command.warp.structure.test.jl | 28 |
| main.command.warp.bubble.test.jl | 32 |
| main.command.warp.ring-topology.test.jl | 44 |
| main.command.recur-warp.test.jl | 62 |
| main.command.warp.evidence-integrity.test.jl | 108 |
| main.command.warp.discovery.test.jl | 75 |
| main.command.warp.docs-reconciliation.test.jl | 49 |

Total: 490 focused assertions. No new full Cargo/Julia regression run is claimed.
Behavioral writers used temporary fixtures. Project writer actions are limited
to the requested reconciliation acceptance layers.

Read-only repository release-safe help for both CLIs, `warp config -d docs --json`,
and documented optimum-fixture status matched the claims. Scoped trace-id
discovered reconciliation contracts. Final map/merge/reveal/inventory checks
establish closeout, not semantic proof from role classification.
`git diff --check` passed; `git diff --name-only -- src Cargo.toml Cargo.lock`
was empty. The accompanying source receipt binds changed document/test bytes
with SHA256; baseline binds unchanged implementation. Newline conversion may
change byte hashes independently of semantic content.

Runtime: Windows, Julia 1.12.0. Binary SHA256:
Recur `b86cce7237278f98adfdd51e076b2513dc9c886f713ded91a027e6bac1517a4e`;
companion `0b5d9ab91aff899092f1afd6b6b1b81990878a2bcfd5a757246f3b287f1feecd`.

Layers carry reviewed declared evidence, not independent producer execution by
Recur. Matching snippets establish reference integrity, not semantic truth.
Branch publication is not registry release evidence. The next configuration-driven
companion Warp is proposed only, not declared or implemented by this closeout.
