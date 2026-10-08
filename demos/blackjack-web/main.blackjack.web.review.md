# Website source-to-Lang review

defines: demo.blackjack.web.dependencies author review of the website boundary
consumer: demo.blackjack.web.table private state and public projection
consumer: demo.blackjack.web.play validated transitions and settlement
consumer: demo.blackjack.web.http isolated revisioned requests
consumer: demo.blackjack.web.browser presentation of authoritative snapshots
register: demo.blackjack.web.reassessment changed reviewed input needs a new review

Reviewer: Codex, author self-review. Scope: the explicitly hashed local sources
and contracts in `main.blackjack.web.review.json`. This is not whole-program
cycle detection or an independent audit. Unknown library/browser internals remain
out of scope. A matching hash establishes freshness, not correctness.
Trace comments use this repository's configured `publish`, `consumer`, and
`register` vocabulary. A regression checks that the play contract, producer and
test consumer appear in their intended role groups without changing global policy.

## How the symbolic reference corresponds to the code

The four WIR files progressively define table, interaction, transport and browser
boundaries. `PrivateGame` includes phase/revision, purse, escrow, house ledger,
deck/cursor, hands and history. `PublicState` explicitly lists all 18 fields;
the regression compares these names with the real Julia result. Semantic types
and rules are verified by behavior tests, not by opaque type names alone.
Single-value returns such as Game or PublicState use named single-field packing
in the reference. The browser has top-level functions rather than a Browser
module; `Browser.render` and `Browser.command` are documented logical bindings.
`Bindings.settlement` maps explicitly to Julia `BlackjackWeb.settle!`. WIR1's
current binding token cannot contain `!`; adding Julia identifier support is a
future grammar decision, not an invented implemented feature.

The CIR file is **review order**: shared rules → engine → HTTP → view → controls.
It is not an execution scheduler or extracted call graph. View review depends on
HTTP's public contract, although render does not itself call the server. Controls
consume view helpers and transport results. The intentional request/response
cycle across turns does not create reverse Julia imports or recursive calls.

## Reviewed helper calls, inputs and effects

| Helper/binding | Input → output | Direct local dependencies / effects |
| --- | --- | --- |
| Rules module | cards → score/outcome | Includes unchanged blackjack-lab 01–04; no website imports |
| integer | value, bounds, label → Int/error | Validates booleans, ranges and whole-number inputs |
| newgame | starting chips → Game | integer; creates fresh ledger, no HTTP calls |
| allowed | Game → action names | Reads phase, hand size, player/house reserves |
| public_state | Game → PublicState | allowed, Rules.score; copies collections; hides hole card and deck |
| draw! | Game, hand → mutation | Consumes next deck card; bounded by finite deck |
| settle! | playing Game → settled Game | Rules.compare_hands; applies one escrow payout and history/stat update |
| finish! | playing Game → settled Game | Rules.score, draw!, settle!; dealer hits below 17, never after player bust |
| transition | Game, command, optional test deck → copied Game/error | integer, allowed, newgame, Rules.deck/score, draw!, finish!, settle!; validates before publishing |
| response/failure | status, body → HTTP.Response | JSON3.write; headers; no game mutation |
| session_key | request → cookie key/empty | Cookie parsing, no game mutation |
| handler | request, Store → response | session_key, newgame, transition, public_state, response/failure; one lock publishes successful actions |
| start/main | port → loopback HTTP lifecycle | handler, HTTP.serve!, no browser invocation |
| cardNode/paintCards | public card IDs → DOM | Rank/suit presentation only; no scoring or deck generation |
| scoreText/historyRows | public results → text/DOM | Server totals and outcomes, no recalculation of rules |
| chosenBet/updateBet | input + public limits → UI validity | Even-number and available-funds feedback; server still validates independently |
| render | snapshot → table DOM | paintCards, scoreText, historyRows, updateBet; never calls command/load |
| load | GET state → render | fetch, error, render; visibility/reconnect events, no retries of writes |
| command | action + revision → POST then render | fetch, error, render; busy guard; stale state restored or explicit reconnect |
| event listeners | user input → command/dialog/render helpers | Calls are event-triggered; dialogs block gameplay shortcuts |

Handler calls engine; engine imports only reusable rules and Random; rules never
include the website. Browser calls the HTTP protocol and consumes responses.
None of those lower layers imports its controller. The test deliberately adds an
engine dependency to the rules review lane and requires SGR001, including under
scope filtering. It checks the **declared** cycle, not hidden Julia syntax.

Loops are bounded separately: each draw consumes a finite card; score consumes
the remaining high aces; the session store expires idle entries and caps active
sessions; history caps at 20. An application may keep accepting later hands—that
is its intended lifecycle, not recursive dependency growth.

## Faults and remaining limits

The initial correspondence test caught seven omitted public-state fields and
an incorrect handler namespace. The references were corrected and the tests
retained. Failed traces remain in the observations directory. A pending review
file initially prevented a successful full run, as intended.

- Review external dispatch, HTTP/JSON3 internals and browser implementation
  separately if assurance beyond this local source boundary is required.
- Any new source call or result field must first be compared with these contracts.
  Do not update a hash merely to silence a stale-review failure.
- No public hosting, accounts, real money, multiplayer, splits or persistent
  database is implemented. This is a complete local single-player website.
- The source review itself is manual. Future companion correspondence support
  stays in the existing `main.lang.binding-correspondence` Warp.
