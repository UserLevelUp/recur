# Rival browser smoke — 2026-10-06

defines: demo.blackjack.rivals.browser.acceptance observed production UI
consumes: demo.blackjack.rivals.browser.seats optional seat presentation
consumes: demo.blackjack.rivals.table.settlement shared-dealer results

Production Julia server on loopback port 8794; real shuffled decks, production
assets and handlers. No private test deck endpoint or injected game state.

- Started from solo v2, selected one rival through New table. Dealt 20 chips.
  Human 7 stood; rival 15 automatically hit to 21; dealer finished at 19.
  Human bankroll 480/net -20; rival 520/net +20; all escrow zero.
- Reset to two rivals with 500 chips per seat. Dealt 20 chips. Human 9 stood;
  rivals stood at hard 20 and soft 17. Shared dealer busted at 22. Each bankroll
  520/net +20; all seats settled once. Rival IDs/panels remained separate from
  human hand 1. Hole card and dealer score were concealed before settlement.
- Reload preserved the two-rival session and results. Updated mode title and
  subtitle correctly describe two rivals sharing Ada and the deck.
- Responsive override 390x844: two panels render vertically, controls remain
  reachable; DOM clientWidth and scrollWidth were both 375 (no horizontal overflow).
  A full-page screenshot was inspected. Temporary viewport is restored afterward.

These are author-operated browser observations, not an automated UI runner or
proof of strategy quality. Randomized/deterministic Julia and HTTP tests cover
rule/error/ledger boundaries separately. Skill ratings are intentionally deferred.
