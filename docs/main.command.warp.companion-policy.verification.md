# Companion policy foundation verification — 2026-09-06

Baseline: reconciliation commit `17626c8b94f326a949b46c22b515d0863c1af7ac`,
pushed atomically to recur-lang and a.0.2.8. Implementation below is local,
not committed/pushed or installed globally by this work.

## Red to green

`julia julia-tests/main.command.warp.companion-policy.test.jl` against the old
release-safe companion: 44 passed, 19 failed, zero errors/broken. Failures expose
inherited policy, compound suffix, normalization and invalid-policy refusal gaps.
After replacing the separate parser with WarpPolicy::load and using state/group:
64 passed. Expanded safety cases then passed **81 assertions**, zero failures.
No assertions were removed or weakened to obtain green results.

Tests run real core/actor subprocesses on temporary roots. They compare bytes
before/after dry runs and refusal, test all four state groups, inherited/nearest
configuration, normalization/longest suffix, malformed/scalar/non-string/duplicate/
overlapping/invalid-component config, policy changes before confirmation, blockers,
unknown files, archive destination collisions, invalid UTF-8, lane isolation,
exact archived content, preserved interesting content and acknowledgment creation.

Implementation preserves existing output schemas, defaults, confirmation, archive
paths and conservative refusal. It additionally rejects unreadable content instead
of silently treating it as empty. Migration differences are documented in the
readme: inherited config now applies; malformed legacy actor configs fail as core
queries do; compound suffixes match the shared longest-suffix rule.

## Regression runs

- `cargo build --profile release-safe --locked --bin recur --bin recur-warp`: passed.
- `cargo test --locked`: 179 passed, 7 ignored doctests, zero failures; existing
  unused Element warning unchanged.
- `julia julia-tests/runtests.jl`: **2853 passed, 73 expected broken**, zero
  failures, 1m15.5s. Includes the 81 new assertions, existing query/receipt/evolution/
  collapse tests and all Sudoku suites. No existing expectations changed.
- Final `julia julia-tests/main.command.warp.docs-reconciliation.test.jl`:
  51 passed after two follow-up documentation links were added; the full run had
  executed the earlier 49-link/assertion version. This is not reported as a new
  2855-assertion full run.
- `rustfmt --edition 2021 src/recur_warp_main.rs` and `git diff --check`: passed.

Windows, Julia 1.12.0; repository release-safe binaries. SHA256 of tested files:

| Artifact | SHA256 |
|---|---|
| src/recur_warp_main.rs | 4f38e95cc19ac283aeba1a3e9227bef2bc1fb92f2c593f8f387b93e864a0858a |
| julia-tests/main.command.warp.companion-policy.test.jl | f8aec1d2fbff78aee2ccea10c9fb26517648e46b5f070bf98d834ffe5f404f45 |
| target/release-safe/recur-warp.exe | 801289e01754b2d249a31c993fdae957f772ee641405e1f58ac0f680ceca5c5f |

Hashes bind local bytes; Git newline conversion may change byte fingerprints.
Unchanged shared policy/query implementation is bound by the baseline commit.
Reconciliation historical receipts retain their original source binding; current
guides link this resolved follow-up instead of implying the old mismatch persists.

## Bounded completion

This completes shared suffix-policy foundation, not arbitrary opinionated goal
execution. No permission expansion, transition DSL, external command scheduler or
new scoring engine. Unknown files may remain companion ambiguities even when core
queries omit them. Confirmation re-evaluates current state, but no immutable
preview token or transactional concurrency guarantee is claimed. More general
transition/evidence/retry policies require a separate bounded design and tests.
Acceptance layers carry reviewed declared evidence; Recur does not rerun producers.
