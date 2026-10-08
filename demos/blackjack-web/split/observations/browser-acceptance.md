# Split browser acceptance observation

defines: demo.blackjack.web.split.browser.verification desktop and mobile observations
consumer: demo.blackjack.web.split.test.plan browser.play
publish: demo.blackjack.web.split.browser.acceptance observed user-facing behavior

2026-10-02, Codex in-app Chromium browser, author-operated smoke test.
Fixture: main.blackjack.web.split.browser-fixture.jl on localhost:8792, using
production engine/server/assets. The private fixture seeds an opening pair; it
adds no production API or alternate scoring code.

1. Original 8-spades/8-hearts, 10-chip wager, wallet90: Split enabled, exact extra
   wager text displayed. Dealer 10-spades plus hidden card.
2. Click Split: hands [8,3] and [8,2], wallet80. Hand1 marked aria-current/active;
   hand2 says UP NEXT. Split disabled with one-split explanation.
3. Click Double: first hand21 with bet20 and STOOD; wallet70. Hand2 highlighted,
   bet10. Dealer hole card and score still concealed. No round/history payout.
4. Viewport390x844: both panels and all controls visible without horizontal page
   overflow (clientWidth=scrollWidth=375). Reload retains active hand2, cards,
   stakes and hidden dealer. Visual inspection confirms gold active-hand border.
5. Stand hand2: dealer19 revealed; hand1 +20, hand2 -10, wallet110, roundprofit+10.
   One history row contains both outcomes. Stats one win, one loss, two hands,
   one round. No browser errors/warnings observed.
6. Viewport override reset. Production server restarted separately on8791 with
   new v2 session schema; production uses its normal random deck.

Browser observations are manual UI evidence, not an automated UI regression
runner or independent audit. Deterministic Julia HTTP tests separately cover
stale Split/final Stand, wrong active ID, old protocol and unchanged state.

Production smoke after restart: normal shuffled opening 2-diamonds/K-spades,
20-chip bet. Keyboard S stood; Ada drew to24, wallet520 and one-round/one-hand
history. New table dialog reset to500 and empty history. Browser console had no
warnings/errors. Fixture test tab closed; production table left open. Minor
presentation polish after first screenshot: singular round/hand labels and a
stacked mobile history heading; no game-rule changes.
