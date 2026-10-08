# Single-player blackjack website

defines: demo.blackjack.web.requirements playable browser table
consumer: demo.blackjack.cards existing score and payout semantics

Author contracts, then tests, then implementation, in four stages: table/public
projection, interactive rounds, HTTP sessions, browser rendering and controls.
The existing blackjack-lab source stays unchanged. Julia owns game decisions;
HTML/CSS/JavaScript are presentation and HTTP transport only. No Python.

- One human versus the computer dealer, with Hit, Stand, Double Down and one Split.
- Simulated chips only; default balance 500 and house 10000. New table accepts
  integer starting funds 20..10000. Even bets 2..500 and no more than available
  balance; wager is held in escrow until a single settlement.
- One freshly shuffled 52-card deck per round. Initial deal alternates player,
  dealer, player, dealer. Dealer hole card and total remain private while playing.
- Dealer peeks for natural blackjack and stands on all 17s including soft 17.
  Naturals settle immediately, paying 3:2; normal wins pay 1:1; ties return stake.
  Player bust loses even if dealer could bust. One equal-rank split into two hands, no resplit. Split aces get one card each and stand; all split 21 pays 1:1. No insurance or surrender.
- Double is available only with the initial two cards, sufficient player funds
  and house reserve; add the same stake, take one card, then advance to the next hand or resolve the dealer. Double after split is allowed for non-aces.
  Hitting to 21 automatically stands. Dealer draws once after all player hands finish, unless all bust.
- Completed hands show full cards, result, profit and balance. Keep last 20 rounds with per-hand results,
  lifetime table win/loss/push counts and net profit. Reset is only between hands.
- Session state lives in the Julia process, survives page refresh but not server
  restart or expiry after two hours idle. At most 128 active sessions. Cookie is
  HttpOnly and SameSite Strict. Actions carry protocol version 2, the last observed revision and the active hand ID for play; stale
  or duplicate requests cannot pay twice. Failed actions do not mutate state.
- Listen on loopback. Serve an exact asset allowlist, not repository files.
  Reject cross-origin writes, oversized JSON, malformed commands and invalid bets.
- Responsive desktop/mobile layout, keyboard-accessible controls, visible focus,
  reduced-motion support, connection errors and recovery without lost hand state.
- Lang checks validate declared fragments. Review all source dependencies and
  input/output correspondence separately; record hashes and unknowns in Eventness.

Acceptance: deterministic Julia fixtures for naturals, soft aces, hits, bust,
stand, double, pushes, funds/revision errors and privacy; randomized legal play
conserves money; HTTP session isolation and duplicate actions; real browser
smoke through deal/action/settlement/reset plus narrow viewport inspection.


Split acceptance and exact rules: split/main.blackjack.web.split.tests.todo.current.md. Both player and house reserves cover aggregate wagers; split and double preserve wallet + escrow + house. Stats distinguish rounds from hands. Public schema blackjack-web-state-v2; old clients receive a reload response.
