# Split v2 source-to-Lang review

defines: demo.blackjack.web.split.dependencies reviewed implementation boundary
consumer: demo.blackjack.web.split.contract symbolic inputs and outputs
consumer: demo.blackjack.web.split.runtime deterministic and randomized evidence
publish: demo.blackjack.web.split.review source-bound author assessment
register: demo.blackjack.web.split.reassessment changed inputs require renewed review

Reviewer: Codex author self-review, 2026-10-02. This is a manual local review,
not independent verification or proof about all Julia dispatch/library internals.
The original v1 review and acceptance remain historical. Pre-change root files
are retained as non-discoverable `.snapshot` files under observations/v1-source.
The v2 JSON fingerprints are relative to the blackjack-web root.

## Contract correspondence

The split WIR has six boundaries. `PrivateRound` is the mutable Game, returned
as a copy by transition; helpers mutate only that unpublished copy. Hand stores
stable id, cards, stake, status, split origin and individual result/profit.
`RevisionedHandCommand` contains version=2, revision, action, hand_id for play
commands. Deal/reset instead carry bet/money. Unknown fields are rejected.
`PublicHands` contains id/cards/stake/status/split_hand/score/result/profit.
The exact 20-field public snapshot is compared to both view declarations by tests.
`SplitPolicy` is currently fixed by source, not configurable per session.
The settlement reference's `(round,result)` describes Game and its stored results;
Julia returns Game alone. Types and packing are descriptive, not Julia signatures.

| Symbol / alias | Actual helper | Dependencies / effects |
| --- | --- | --- |
| eligibility.f | BlackjackWeb.split_eligibility | Reads original hand rank/size, wallet and aggregate reserve; no mutations |
| SplitV2.split | BlackjackWeb.split! | eligibility, active, rank, Hand, draw!, handscore, advance!; replacement cards in ID order |
| SplitV2.advance | BlackjackWeb.advance! | Activates next waiting hand or calls finish!; transition/end_hand! supply completed status |
| SplitV2.dealer | BlackjackWeb.finish! | Requires no playing/waiting hands, handscore, Rules.score, draw!, settle! |
| SplitV2.settle / Bindings.settlement | BlackjackWeb.settle! | outcome, public_hand, ledger/history/stats update once per round |
| view.f | BlackjackWeb.public_state | public_hand, allowed, split_eligibility, Rules.score; copies mutable collections |
| handscore/outcome | Same-named Julia helpers | Rules.score; removes natural eligibility after split before comparing totals |
| transition | BlackjackWeb.transition | version/revision/identity validation, allowed, copied state; dispatches deal/split/hit/stand/double |
| route.f | BlackjackWebServer.handler | v2 protocol check before revision, locked store update; 426 old protocol, 409 stale revision |
| Browser.render | JS render | paintHands/paintCards/historyRows; server supplies score/status/results |
| Browser.command | JS command | Version, revision, active ID, busy guard; no automatic write retries |
| paintHands/acceptSnapshot | JS functions | Two labeled panels with one current hand; v2 snapshot validation |

Aliases are used for Julia `!` names because WIR binding tokens do not support
that suffix. Lang does not call these functions. The CIR is review prerequisites,
not an extracted runtime call graph: rules → hands → turns → dealer → settlement
→ view. Runtime helper calls flow transition → split!/end_hand! → advance! →
finish! → settle!. No helper calls transition or the HTTP handler from below.
`settle!` can also run immediately for original naturals. `public_state` is a
separate read path, not called by settle!. Repeated user turns are HTTP events,
not recursive calls. The intentional CIR back-edge remains rejected with SGR001
including under scoped queries. Hidden or dynamic Julia cycles remain outside
that static claim.

## Reviewed semantics and limits

One split, equal rank (not equal ten-value), max two hands. Player funds and house
reserve cover all stakes before split/double. Split aces receive one card each;
other split 21s stand automatically. Only original naturals pay 3:2. All hands
finish before a single dealer/aggregate settlement. Hole-card privacy applies
through the second hand. Stats count rounds and hands separately; history caps
at 20 rounds. Successful transition increments revision once; failed actions
leave the original copy intact. Restart explicitly resets old in-memory sessions.

A serialization test initially assumed JSON3 preserved `2.0` as Float64. It
normalizes an integral JSON number to Int; the HTTP test now rejects `2.5`, strings
and booleans while direct Julia tests reject non-Integer arguments. Protocol v2
is the numeric integer value, not a requirement about JSON spelling.

No new public debug/deck endpoint, persistent sessions, resplits, insurance,
surrender, multiplayer, public hosting or automatic Julia cycle scanner.
Browser smoke uses a separate local fixture server seeded through the private
Store, then real controls. Production always shuffles a fresh deck.
