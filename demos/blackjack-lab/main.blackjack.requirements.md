# Hello World to a blackjack session: contract-first experiment

defines: demo.blackjack requirements for growing compact symbolic workflows
defines: demo.blackjack.round independently settled player-versus-dealer results
defines: demo.blackjack.session configurable multi-round money and final standings
triggers: demo.blackjack.tests executable behavioral and language probes

This is a deliberately bounded teaching variant with simulated chips. Choose
1–6 players, one named house dealer, starting money, an even fixed wager,
1–1000 rounds, a seed and the house's starting bank. Defaults: three players,
100 chips each, wager 10, five rounds, Dealer, house bank 1000, seed 42.
Player identifiers remain stable even after a player cannot afford the wager.
The house is reported separately and is not a tournament entrant.

Each round uses a fresh shuffled 52-card deck. For deterministic examples an
explicit complete permutation can replace shuffling. Deal one card to each
active player then the dealer, twice. Players hit until 17. The dealer stands
on all 17s including soft 17. A two-card natural beats a non-natural 21 and pays
3:2; other wins pay 1:1; a push changes neither balance. A player bust loses
even if the dealer also busts. If either opening hand is natural, that player
does not need to draw. A dealer natural ends play immediately.

No split, double, insurance, surrender, variable wagers, human decisions or
multi-deck shoe are claimed. These are explicit rules of this demo, not a claim
to implement all casino variants. Integral, even wagers avoid fractional chips.
Reject invalid config, duplicate cards and malformed balances before publishing
a result. Copy mutable inputs. Bank plus player balances is conserved, all
balances stay nonnegative, and consumed cards are unique. Reserve house funds
for all active players' possible natural payouts before starting a round.

Every round reports each player's win/loss/push/sitting-out, payout delta,
resulting balance, player winners, dealer wins against specific players, and
ties. There may be zero or multiple winning players. A session stops at its
round limit, when no player can fund a wager, or when house reserves are too
small. Final leaders are all players tied for the greatest ending balance;
net profit and the reason for stopping are reported separately.

## Iteration order

1. Hello World: optional argument bundle, binding, flow, event and Warp.
2. Table object and calling function: configuration bundle, alias and share.
3. Cards and scoring: explicit contracts, ace/natural edge cases.
4. Play and settlement: large bundles, conservation and rejection without mutation.
5. Session and CLI: configurable player count, repeated rounds and tied leaders.
6. Coordination: named contracts, projected orders, independent score producers,
   join, ordered awaits, receipt policies and audit; Julia implements scheduling.
7. Query and adversarial probes: list/show/report/check, expand, event filters,
   dependency/wait cycles, missing joins, scope boundaries and excluded grammar.
8. Companion evidence: init preview/install/repeat; transition preview, observed
   behavioral evidence, confirmed acceptance, replay/recovery and input drift.

Write the numbered Lang contracts and behavioral tests before implementation.
Each stage's observations must distinguish a missing implementation from an
assertion failure, and static fragment validity from runtime correctness.
Do not claim all language proposals work. Record every tested feature and every
excluded/deferred capability in the feature matrix and follow-up todo.
