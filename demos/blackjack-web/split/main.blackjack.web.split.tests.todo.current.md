# Split acceptance matrix

defines: demo.blackjack.web.split.tests proposed runtime acceptance
consumer: demo.blackjack.web.split.contract symbolic requirements
consumer: demo.blackjack.web.split.implementation.plan implementation slices
publish: demo.blackjack.web.split.test.plan bounded behavior checks

Status: **implemented and verified**. The original matrix below maps to runtime.test.jl (engine), ../main.blackjack.web.http.test.jl (protocol/replay), model.test.jl (declared cycle and exact public fields), and observations/browser-acceptance.md (browser.play). See main.blackjack.web.split.verification.complete.md for counts and limits. The historical --runtime-gap log stays red; the current probe now passes.

| ID | Fixture / action | Required observation |
| --- | --- | --- |
| eligibility.pair | 8♠ + 8♥, opening bet 10, wallet 90 | Split offered; exactly 10 additional chips required |
| eligibility.rank | 10♠ + K♥ or three-card hand | Split unavailable, no state mutation on request |
| eligibility.funds | Pair but wallet below stake / house below combined reserve | Rejected atomically |
| split.cards | Rigged opening [8,9,21,7], then [2,3] | Hands [8,2] and [21,3], shared dealer [9,7], cursor advanced twice |
| split.ledger | Same fixture from 100 chips, bet 10 | Wallet 80, escrow 20, house 10000; stable IDs 1/2 |
| split.once | Attempt to split either resulting pair | Rejected; max two hands |
| turn.advance | Stand or bust hand 1 | Hand 2 becomes active, dealer stays hidden and does not draw |
| turn.identity | Correct revision but wrong active hand ID | Rejected without mutation |
| dealer.once | Finish hand 2 | Exactly one dealer draw phase; both results use that same dealer hand |
| dealer.all-bust | Both player hands bust | No dealer draw, both wagers lost once |
| split.aces | A♠ + A♥, then K♠ + 9♣ | One card each, no Hit/Double; 21 is non-natural |
| split.twentyone | Any split two-card 21 versus dealer 20 | Normal 1:1 profit, not 3:2 |
| double.independent | Double hand 1, stand hand 2 | Stake 20 vs 10; total escrow 30; reserve includes both |
| settlement.mixed | Hand 1 +20, hand 2 −10 | Net +10; wallet 110, escrow 0, house 9990 |
| settlement.tie | One win, one push | Push returns only its own stake; no double credit |
| replay.revision | Send Split or final Stand twice at same revision | One accepted write, one conflict; no extra card or payout |
| projection.privacy | Hand 1 done while hand 2 active | Hole card/score and deck absent from response |
| stats.history | Complete one split round | rounds +1, hands +2; one round record with two results |
| unsplit.regression | Existing no-split natural/Hit/Stand/Double cases | Original outcomes, private deck and monetary invariants preserved |
| protocol.reload | Cached v1 browser sends v1 write | Reject with clear reload message, do not guess a hand |
| browser.play | Pair → Split → play each hand → dealer → result | Correct active highlight, hand-specific controls, one dealer reveal |
| property.conservation | Seeded legal split/double sequences | No negative funds, unique cards, bounded hand count, conserved total |

The model fault test introduces a forbidden settlement prerequisite into the
hand-construction lane. SGR001 must expose that circular design even when the
query is scoped to the unrelated view lane. This detects the mapped model fault;
it is not a parser for hidden Julia calls.
