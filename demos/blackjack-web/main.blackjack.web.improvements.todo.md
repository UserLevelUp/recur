# Website follow-ups

defines: demo.blackjack.web.followups bounded extensions after observed acceptance
consumer: demo.blackjack.web.dependencies source-to-reference review limits

- [ ] Specify Julia binding names with `!` before extending WIR token grammar.
  This demo uses an explicit `Bindings.settlement` → `BlackjackWeb.settle!` mapping.
- [ ] Generalize source-to-reference review packets in the existing
  `main.lang.binding-correspondence` Warp; avoid mistaking hash freshness for proof.
- [ ] If requested, specify persistence and public-hosting requirements separately
  (database, authentication, transport protection, deployment and session policy).
- [x] Split implemented in main.demo.blackjack-web.split with symbolic
  hand/escrow contracts and payout/conservation tests.
- [ ] If requested, scope insurance or surrender as separate contract-first Warps.

These are open ideas, not incomplete requirements of the local single-player
demo. The tracked verification record states the completed scope and evidence.
