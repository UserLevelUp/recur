# Optional computer rivals v1

defines: demo.blackjack.rivals.contract optional competing seats
consumes: demo.blackjack.web.split.contract existing human split rules
consumes: main.command.warp.dispatch asynchronous bounded assignments

User choice: zero (existing solo), one or two computer rivals beside the human,
all competing against the same dealer. Rivals do not replace the human. Julia
owns all gameplay; the browser only presents public snapshots. No real money.

## Rules and interface frozen for worker slices

Zero rivals preserves the existing v2 engine and exact public schema. Rival mode
uses `BlackjackRivals.Table`, `newgame(money=500, rival_count=1)`,
`public_state(table)` and `transition(table, command; order=nothing)` in
`main.blackjack.rivals.engine.jl`. Module is standalone with `using Random` and
the copied Rules sources under `rules/`; integration will adapt their include
paths. Table exposes `revision`. Transition accepts protocol version 3, the same
actions/revision/human hand IDs as v2, and copies before mutation. Optional reset
`rival_count` is integer 0..2; server owns mode switches between v2 and v3.

Rival public schema is `blackjack-web-state-v3`, with `protocol_version=3`, all
the existing human v2 fields, `rival_count`, and `rivals`. Each rival exposes
id, name, balance, starting, escrow, net, hands, stats, result, profit and policy.
Human score/history/split fields retain existing shapes. Rival hands use the
same public hand fields. Dealer hole card and score remain hidden during play.
Private deck/cursor are never projected. Rival actions cannot be requested by
the client. Rivals are identified separately from human split hands.

Shared shuffled 52-card deck; deal two passes, human then participating rivals
in stable seat order then dealer on each pass. Deterministic `order` fixtures
validate a complete unique deck. Human rules preserve Hit, Stand, Double and one
Split. Ada stands on soft/hard 17 and peeks for natural blackjack. Dealer natural
settles all seats immediately. Human natural waits for rivals/dealer without
requiring a human action. Split aces get one card; split 21 pays 1:1.

V1 rivals are explicitly labeled **Stand on 17 benchmark**, with fixed hit below
17 / stand at 17 or above, no split/double and no hidden-information access.
Policy takes only visible hand score/public action information, never deck/hole
card. Rivals act automatically after the human finishes, bounded by the finite
deck, then the dealer plays once and every seat settles once. They wager the
human's bet when affordable, otherwise the largest affordable even amount up to
that bet; below 2 chips they sit out. Each rival starts with the selected funds.
No skill rating or claim of optimal play is introduced in this Warp.

Separate seat wallets/escrow; one shared house reserve (10000). Sum of all seat
wallets + escrow + house is conserved. Aggregate house reserves must cover worst
possible winnings before deal/split/double. Naturals pay 3:2, normal wins 1:1,
push returns stake, bust loses. Failed actions cannot mutate any seat. Each seat
has round/hand wins/losses/pushes and net profit; retain last 20 human rounds.

Browser worker produces `main.blackjack.rivals.js`: export
`window.BlackjackRivalsView = { render(container, rivals), accepts(schema),
protocolVersion(state) }`; optional CommonJS export for Node tests. Safe DOM
textContent only, accessible per-seat panels, score/cards/bankroll/net/policy,
friendly zero-rivals view, no gameplay or payout calculations. Root integrator
owns reset chooser (0/1/2), script allowlist, HTTP mode switching, snapshot
acceptance and insertion into existing UI. Worker must not edit canonical files.

## Coordination and acceptance

Engine and browser workers have disjoint scratch directories and immutable task
contexts, source placeholders, tests and explicit inputs. Actual CLI coordinator
runs configured host processes, workers run tests, observations are reviewed
before publishing completion gates. No provider output automatically accepts a
slice. Config/map stays fixed while attempts run. Lang remains optional.

Integration requires HTTP isolation, stale/replayed requests, bad rival counts,
mode switches only between rounds, privacy, assets and original solo/Split
regressions. Acceptance includes browser play with one and two rivals and a
narrow viewport. Preserve original source-bound receipts as historical; write
new rival review evidence rather than claiming old hashes still qualify.
