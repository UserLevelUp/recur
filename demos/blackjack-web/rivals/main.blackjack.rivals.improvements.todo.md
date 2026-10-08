# Subsequent skill-comparison work

defines: demo.blackjack.rivals.skill.followups measured comparisons after baseline
consumes: demo.blackjack.rivals.policy.decision fixed visible benchmark
consumes: demo.blackjack.rivals.table.settlement observed per-seat results

- Define additional public-information policies, comparable action/bet rights,
  strategy versions and deterministic replay fixtures before adding difficulty.
- Specify skill metrics and their limits: bets, bankroll constraints, sample size,
  natural outcomes, seat/deck order and randomness affect raw net-chip comparisons.
- Design persistent session/export statistics separately from this in-memory
  local table, with explicit inputs and source-bound acceptance evidence.
- If desired, prepare optional Lang contracts for more complex policies before
  coding their bindings; retain runtime and browser acceptance as separate gates.

These are future Warp candidates. This increment supplies optional one/two rival
seats and a labeled benchmark; it does not infer a rating from a short session.
