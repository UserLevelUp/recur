# Hello World to Hold'em: specification-first experiment

Each numbered stage is specified and statically checked before its Julia
implementation is written. Tests are then observed failing before implementation
and passing afterwards. WIR1 describes function boundaries; CIR1 separately
describes the coordination graph. Neither executes the app or proves prose rules.

| Stage | Requirement | Acceptance |
| --- | --- | --- |
| 01 | Hello World | exact greeting, default and explicit name |
| 02 | Class and caller | Julia immutable struct plus methods; caller carries one input bundle, default prefix, blank-name rejection |
| 03 | Deal | 52 unique cards, two private cards each, burn/flop/burn/turn/burn/river, reproducible injected deck |
| 04 | Evaluate | best five of seven; nine hand categories, wheel, kickers, ties, board-only winner |
| 05 | Play | two-player hand, blinds 1/2, four betting streets, check/call/raise-to/fold, no side pots; chip conservation and replay |
| 06 | Coordinate | independent hand evaluations join at settlement; introduced self/dependency/wait cycles and missing joins are detected |

Scope: a deterministic local teaching app, with a scripted CLI example. Two
players start with equal positive stacks of at least 2. Raises must be full legal
raises and fit both players' effective stacks. Equal-stack all-ins run out the
board once matched. Unequal starting stacks, short all-in raises, multiplayer
side pots, networking, persistence, tournament rules and poker AI are excluded.
The dealer is player 1 (small blind), acts first preflop and last postflop.
No real money. Julia structs/methods implement the requested class-style boundary.

Rule reference: https://www.pokerstars.com/poker/games/texas-holdem/
Heads-up position reference: https://www.pokerstars.com/help/articles/poker-rules-master/217459/

defines: demo.holdem.hello exact greeting and default parameter
defines: demo.holdem.call caller-to-class boundary
defines: demo.holdem.deal unique deck and ordered streets
defines: demo.holdem.rank best-five comparison with all tie breakers
defines: demo.holdem.play legal actions, information boundaries and conserved chips
defines: demo.holdem.settle joined rankings and deterministic payout
defines: demo.holdem.graph declared dependency and wait-cycle checks

Runtime tests are observations, not accepted Warp receipts. No complete marker
is used to stand in for test evidence. Tests will explicitly demonstrate that
unmodeled implementation dependencies remain invisible to Lang.
