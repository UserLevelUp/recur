# Splitting: contract-first design under review

defines: demo.blackjack.web.split.design two-hand extension of the playable website
consumer: demo.blackjack.web.split.contract proposed symbolic input/output bundles
consumer: demo.blackjack.web.play current single-hand transition
publish: demo.blackjack.web.split.implementation.plan requirements and acceptance cases
register: demo.blackjack.web.split.reassessment new rules require new contract review

Status: **implemented; original design retained for lineage**. The policy below was adopted for v2. See main.blackjack.web.split.verification.complete.md for observed acceptance. Previous v1 reviews and receipts remain historical.

## Proposed initial policy

- Split only the original active two-card hand with equal ranks, including aces.
  A ten and king are not equal ranks under this proposal. Suits do not matter.
- One split per round, at most two hands; no re-splitting initially.
- Reserve one additional wager of the same size from the available wallet.
  The house must cover the sum of both possible normal-win profits.
- Move the two original cards into stable hand IDs 1 and 2. Deal one replacement
  card to hand 1, then one to hand 2, from the same private deck. Play left to right.
- Allow Double Down on a non-ace split hand's first two cards, subject to its
  additional wager and the house's aggregate outstanding liability.
- Split aces receive one extra card each and automatically stand. No doubling
  or hitting them. Other split hands reaching 21 also automatically stand.
- A two-card 21 after a split is ordinary 21 and pays 1:1, never natural 3:2.
  Original unsplit naturals retain their current 3:2 rule and early settlement.
- Finish every player hand before the dealer reveals/draws. Dealer plays once,
  stands on all 17s; if all hands bust, reveal without drawing.
- Settle each hand's result independently, then publish one aggregate ledger
  update and one round-history record. Do not settle a finished first hand early.

## Symbol bundles

`PrivateRound R` owns `revision, round_id, phase, wallet, house, escrow,
hands[], active_hand_id, split_used, deck, cursor, dealer, policy`.

Each `Hand H` owns `id, cards, stake, status, from_split, split_aces,
natural_eligible, result, profit`. Status is active, waiting, stood or busted
before round settlement, then settled. A hand may not be both active and waiting.

`Command C` carries `action, revision, hand_id` for split/hit/stand/double.
The hand ID must match the active hand even if the revision is otherwise current.
Deal/reset remain round-level commands without a hand ID. All illegal requests
return an error without changing any state. A successful atomic transition
increments revision once, even when it advances a hand and completes settlement.

`RoundResult Q` contains `round_id, hands[{id,cards,stake,result,profit}],
dealer_cards, total_profit, ending_balance`. A split can produce win+loss,
win+push, two losses or other combinations; one scalar outcome cannot represent it.

The `.recur` source uses compact R-like single bundles in each scope. Its nominal
types are a reference for the reviewer; the detailed field/semantic rules here
must be tested in Julia. `SplitV2.*` names are proposed logical bindings, not
functions currently implemented by the app.

## Changes exposed by comparing the contract with today's code

| Current assumption | Required change | Why it matters |
| --- | --- | --- |
| `Game.player`, `stake`, `result`, `profit` are single values | Hands collection with per-hand stake/status/results | Two simultaneous wagers can have different outcomes |
| `finish!` immediately runs dealer and settles | Mark the active hand finished; advance or resolve dealer once | Otherwise the first hand exposes information or ends the second hand |
| `score(cards).natural` means any two-card 21 | Carry natural eligibility from hand origin | Split 21 must not receive an unintended 3:2 payout |
| Double reserve checks only `2 * current stake` | Check total outstanding normal-win liability after the action | The other hand's possible payout still exists |
| Public state has one player array and one score | `hands[]` plus active ID, per-hand actions and split eligibility | UI must show both hands and make the active choice unambiguous |
| Stats count one hand per settlement | Separate rounds from resolved hands; W/L/push totals are per hand | Two hands must not create two dealer rounds |
| History stores one flat hand/result | One record per round with a list of hand results | Preserve aggregate balance and individual results |

## Conservation and ordering

At every transition: **wallet + escrow + house = starting total**.
Before settlement, escrow equals the sum of all hand wagers. Do not clear the
first wager when moving to hand 2. House reserve uses a conservative sum of all
outstanding stakes for a split round; Double adds one current hand's original
stake to that reserve requirement. After settlement, escrow is zero.

Example with 100 chips and a 10-chip opening wager:

| Step | Wallet | Escrow | House |
| --- | ---: | ---: | ---: |
| Before deal | 100 | 0 | 10,000 |
| Deal | 90 | 10 | 10,000 |
| Split | 80 | 20 | 10,000 |
| Double hand 1 | 70 | 30 | 10,000 |
| Hand 1 wins +20; hand 2 loses −10 | 110 | 0 | 9,990 |

Neither hand should run concurrently over the shared deck. A human decision
creates another request with updated state; it is not a recursive dependency
from settlement back to turn handling. The CIR file models review prerequisites,
not parallel game execution or an automatically extracted Julia call graph.

## API and browser migration

Introduce explicit `schema: blackjack-web-state-v2` and command protocol version
2. Reject obsolete write envelopes with a reload instruction. Ship backend and
browser together; an in-memory server restart starts fresh sessions rather than
silently converting an active legacy hand. Keep the existing HTTP revision lock.

UI: add Split with a concise ineligibility reason; display two labeled hands,
their wagers and scores; highlight the active hand. Disable controls for inactive
hands. Keep the dealer hole card hidden while either hand is unfinished. Show
per-hand results plus round profit, and distinguish rounds from hands in history.

## Proposed implementation slices

1. Freeze policy and v2 bundles; add engine tests for the cases in the test matrix.
2. Implement Hand/Round state, atomic split and sequential advancement. Preserve
   the old unsplit behavior through the new single-element hand collection.
3. Implement origin-aware scoring, one dealer resolution, per-hand outcomes and
   aggregate settlement. Prove conservation and replay safety under split/double.
4. Update HTTP projections, schema/version checks and privacy tests.
5. Implement the two-hand browser layout and Split interaction; browser-test a
   deterministic pair fixture without adding a deck-setting production endpoint.
6. Renew source-bound review, run regressions, and record new evidence. Preserve
   the existing website's historical acceptance rather than rewriting it.

- [x] Inspect current implementation and identify single-hand assumptions.
- [x] Author proposed WIR input/output contracts and CIR review dependencies.
- [ ] Implement runtime cases and enable Split in the website.
- [ ] Confirm browser/HTTP acceptance and publish new runtime evidence.
