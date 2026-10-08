# Playable website: bounded acceptance

defines: demo.blackjack.web.verification observed local website acceptance
consumer: demo.blackjack.web.requirements gameplay and public boundary
publish: demo.blackjack.web.verification.result tests and browser observations
register: demo.blackjack.web.reassessment changed source requires fresh evidence

Observed 2026-10-02 on `recur-lang`, with local uncommitted changes. Scope: this
second website, its reused rules and the explicitly recorded source review.
The first blackjack demo was preserved. No commit, push, public deployment or
new Rust installation was performed for this website task.

## Automated results

The standalone Julia suite exits 0 with **8,702 passing assertions**:

| Group | Passed | What it checks |
| --- | ---: | --- |
| Game engine | 8,601 | Naturals, push, soft aces, bust, Hit/Stand/Double, funds, revision, escrow and immutable failure; 40 sessions of up to 30 random legal hands |
| HTTP | 41 | Cookie isolation, replay conflicts, malformed requests, privacy, expiry/capacity, origin checks and actual loopback requests |
| Assets | 14 | Served allowlist, content types/policy and essential accessibility hooks |
| Lang/reference | 46 | Five valid contracts, exact public fields and handler binding, trace roles, review graph and injected cycle, source-hash freshness |

The same 8,702 assertions also passed inside the minimal-compilation full run.
Existing rules are reused without modification. No Python or new Rust code was
needed. The game/server use ordinary Julia compilation; the assertion module
uses the earlier demo's minimal-compilation workaround on this Windows host.

**A clean full-repository result was not obtained.** The ordinary full run
aborted with a compiler access violation in the pre-existing Hold'em tests.
The `--compile=min` retry reported 27,974 passed, 73 expected-broken and one
`ReadOnlyMemoryError` in pre-existing Sudoku code, exit 1. The unchanged Sudoku
suite then passed all 139 assertions in isolation, exit 0. These are separate
observations, not a combined full-suite pass. See the regression strange file.
No failing full-run result was relabeled as accepted test evidence.

## Browser observations

Using the actual loopback server and in-app browser:

- Initial table loaded with 500 chips, usable controls and private dealer card.
- Dealt 20, refreshed mid-hand, and retained the exact player/dealer visibility.
- Hit from 16 to 21: dealer finished on 19, player won 20, balance became 520.
- At a 390×844 viewport, an odd bet of 3 disabled Deal; the 50 preset enabled it.
- Doubled a 50 wager: exactly one player card was added, player busted on 22,
  settled stake 100, balance became 420, history recorded both hands.
- Opened/closed rules, reset to 1,000 chips, and verified zeroed history/stats.
- Restored the normal viewport and used keyboard S to stand on 18; dealer reached
  21, loss 20 and balance 980 were displayed correctly.
- No browser warning/error entries were observed. Narrow layout was visually
  inspected with no horizontal overflow (390 viewport / 375 document width).
- Left the normal viewport and a fresh 500-chip table ready to play.

These are observed browser checks, not a saved automated browser test runner.
Unknown browser/platform combinations and external library internals remain
outside this bounded acceptance. All artwork is native HTML/CSS; Envato assets
were unnecessary. No external assets, trackers or remote font dependencies.

## Reference and lineage findings

The output-shape test caught seven omitted public fields; the binding test caught
the wrong handler namespace. Both references were corrected, retaining the tests.
WIR1 does not accept Julia's `!` suffix in a binding token; the settlement alias
is explicitly resolved in the review document and left as a follow-up grammar
decision. The CIR review graph detects the deliberately mapped rules/engine
cycle under both full and scoped queries. It does not inspect hidden Julia calls.

Trace comments use the current project vocabulary (`publish`, `consumer`,
`register`) and regression checks verify contract/producer/test-consumer roles.
The manual source review is explicitly author self-review with hashed inputs;
library internals and arbitrary future code remain unproven.

Raw failures and successes are retained in `observations`. The Warp
`main.demo.blackjack-web` uses declared references to the baseline and this
bounded acceptance, not independently checked producer receipts. Global suite
qualification remains visible and is not implied by this demo's completed state.
