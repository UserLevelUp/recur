# Split v2: implementation verified

defines: demo.blackjack.web.split.verification bounded completed implementation
consumer: demo.blackjack.web.split.contract six symbolic boundaries
consumer: demo.blackjack.web.split.test.plan 22 acceptance cases
consumer: demo.blackjack.web.split.runtime Julia observations
consumer: demo.blackjack.web.split.browser.acceptance observed controls and settlement
publish: demo.blackjack.web.split.acceptance evidence for the final Warp gate
register: demo.blackjack.web.split.reassessment new rules or source changes require new review

Observed 2026-10-02 on recur-lang. Implemented the one-split policy in the Julia
engine, protocol v2 HTTP API and browser. Two independent wagers share one dealer;
only the active hand can act, and all hands settle together. Split aces receive
one card each; split 21 pays1:1. Double after split is allowed for non-aces.

## Observed acceptance

Full focused website command exited0 under Juliaup1.12.7:

```powershell
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/main.blackjack.web.test.jl
```

| Suite | Assertions passed |
| --- | ---: |
| Original unsplit engine regressions, migrated to v2 bundles | 8,601 |
| Deterministic Split cases and seeded legal-play invariants | 6,094 |
| Split HTTP protocol, active identity and concurrent replay | 18 |
| Original HTTP/session/loopback regressions | 41 |
| Served assets and accessibility markers | 14 |
| Lang references, exact output correspondence, cycle faults and reviewed hashes | 68 |
| Total | 14,836 |

Raw output: observations/acceptance-tests.log. The runtime module is now included
in the normal website suite and its existing julia-tests wrapper. It does not
use Python. Both root and Split view contracts exactly match the public fields.
The deliberate CIR back-edge produces SGR001, including a scoped view query.
Lang execution remains not-run: these are references, not an executable game DSL.

The 22-row matrix is covered by runtime.test.jl, the Split HTTP testset, original
unsplit regression assertions, and observations/browser-acceptance.md. The browser
case is author-operated smoke evidence, not an automated UI runner. It used real
production assets and handlers in a separately seeded private Store fixture:
Split → Double first hand → mobile reload → Stand second hand → +20/-10 results,
wallet110, one round/two hands. No horizontal page overflow or console errors.
The production random-deck server separately passed deal, keyboard Stand, result,
and New table reset; it remains on127.0.0.1:8791 with a fresh500-chip session.

## Lineage and limits

- Baseline was recorded before the implementation: new suite failed the missing
  hands-collection assertion. Older analysis separately recorded two missing
  runtime features. Logs and original v1 review/receipts remain historical.
- main.blackjack.web.split.review.md maps symbols to real functions, including
  logical aliases for Julia names ending in !. Its JSON fingerprints bind the
  explicitly reviewed source/contracts/tests, relative to blackjack-web root.
- Trace IDs link reference, implementation producers, tests and this observation.
  Current trace: observations/implementation-trace.json.
- Warp main.demo.blackjack-web.split uses declared evidence gates. Completion
  receipts bind these observations; they do not assert machine-checked global
  dependency closure, independent review or proof about hidden Julia calls.
- Full-repository tests were not rerun for this bounded increment. Earlier
  unrelated Windows Julia compiler/Sudoku runtime faults remain qualified in
  ../main.blackjack.web.regression.strange.md; no fresh global-pass claim.
- Server upgrade intentionally reset old in-memory sessions. Cached v1 writes
  receive426/reload; stale v2 writes409/current state. No production deck setter.
- No resplits, insurance, surrender, persistence or public hosting were added.
  The existing main.lang.binding-correspondence Warp already covers generalized
  source/reference tooling; no duplicate follow-up Warp is needed for this slice.

Updated user report: http://127.0.0.1:8793/ . Changes remain local and uncommitted.
