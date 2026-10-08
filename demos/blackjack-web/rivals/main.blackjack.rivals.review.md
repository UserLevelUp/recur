# Rival integration source review

defines: demo.blackjack.rivals.review source-bound author review
consumes: demo.blackjack.rivals.contract optional competing seats
consumes: demo.blackjack.rivals.coordination reviewed CLI worker outputs

Reviewed 2026-10-06 by Codex author self-review. Original Split source review and
receipts are retained unchanged as historical observations. This review selects
the current integrated source, not a claim that predecessor hashes stayed fresh.

Julia module `BlackjackRivals` owns optional rival mode. Its Rules include paths
were adapted from worker copies to the existing blackjack-lab files; no rule
source or original v2 engine was edited. Table is a separate type. Transition
validates protocol, revision, allowed human action and field set before modifying
a deep copy. Human hand IDs cannot select rival seats. Deterministic deck fixtures
and random play exercise unique shared dealing, natural/peek, Split, Double,
aggregate reserves, low-wallet sit-out, settlement once and conserved ledgers.

Policy receives a score tuple for its own visible cards. It never takes Table,
deck, cursor or dealer hole card. Public projections detach arrays/statistics and
hide the dealer hole/score until settlement; snapshot-isolation tests include
changing private dealer/deck inputs while comparing public eligibility. Scoring
and settlement call the existing Rules and local helpers; automatic turns draw
only from the finite deck, and late failures remain confined to the copied Table.

HTTP Store now accepts old Game or rival Table. Zero rivals selects unchanged v2;
one/two select v3. Handler checks current protocol and revision under its existing
session lock before transitions. Reset validates count and exact fields, and is
available only between rounds. Failed actions cannot replace the stored state.
Server asset allowlist adds only the rival presentation script. Existing origin,
host, payload, session and cookie policies remain enforced.

The browser reads public snapshots and selects protocol by schema. It renders
inert text through DOM methods; no blackjack score, decision or payout computation
was moved into JavaScript. Controls still address the human active hand. The
0/1/2 chooser resets session money/history and has explicit confirmation through
the existing New table dialog. Rival panels sit outside the fixed-height felt to
avoid overlapping human/dealer controls. Responsive grid and wrapped text bound
the new panels. Displayed net chips are comparisons, not a validated skill rating.

Worker verification and integration are distinct. The coordinator's independent
engine test command passed even though the sandboxed provider could not execute
the WindowsApps alias; its supplemental direct-runtime run is separately noted.
The integrated Boolean-deck test uses explicit vector construction to avoid a
Windows Julia inference crash while retaining the malformed-input assertion.

Lang remains optional. Existing WIR/CIR models describe original solo/Split
fragments; they have not been extended into a proof of rival mode. Current source
fingerprints establish the inputs reviewed, not exhaustive call-graph closure or
independent verification. Browser smoke and runtime logs supply bounded behavior
observations, and acceptance uses declared evidence gates.
