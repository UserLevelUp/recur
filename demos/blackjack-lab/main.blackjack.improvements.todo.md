# Blackjack-driven Lang improvements

defines: demo.blackjack.improvements concrete language and companion opportunities
consumes: demo.blackjack.session iterative experiment
consumes: demo.blackjack.coordination actual projection and join behavior

- [ ] Reusable WIR result contracts and state-plus-action bindings.
  Current WIR input aliases reduce repetition, but output aliases/nested semantic
  types still lack general reuse and checking. First define contract equality,
  optional/default fields and error behavior; add round-to-session red cases.
  Keep this distinct from merely assigning equal field shapes the same identity.
  Contract-first Warp: `main.lang.contract-reuse`.

  defines: demo.blackjack.improvements.contracts reusable state and output contracts
  consumes: recur.lang.work.remaining.contracts existing authoring-friction focus

- [ ] Dynamic player scatter/gather with stable identity and join cardinality.
  The demo uses one batch lane for all configured players. Extend the existing
  `main.lang.advanced.scatter` contract-first Warp with 1, 3 and 6 players,
  sitting-out identities, a missing result, a duplicate result and reordered
  completion. Decide cardinality and failure semantics before executable syntax.

  defines: demo.blackjack.improvements.scatter variable players and exact joins
  triggers: main.lang.advanced.scatter contract-first player fan-out examples

- [ ] Declared-versus-implemented dependency and invariant checks.
  The hidden Julia cycle is invisible to the current static model. Decide an
  explicit runtime receipt or implementation mapping boundary, then test a cycle,
  changed payout behavior, missing conservation check and incomplete bindings.
  A generic planner suggestion is not such a check.
  Contract-first Warp: `main.lang.binding-correspondence`.
  The demo now supplies a concrete 19-function reference and source-bound author
  review in `main.blackjack.dependencies.review.todo.current.md`. Default calls,
  stale references and a mapped hidden cycle are tested; general review packets
  and custom callbacks still require decisions.

  defines: demo.blackjack.improvements.correspondence runtime relationships and money invariants
  consumes: recur.lang.work.remaining.dependencies existing correspondence focus

- [ ] Policy-informed executable test/scaffold generation after planning.
  The new `recur-lang plan` identifies selected symbols, graph blockers and test
  categories. A later separate increment should turn reviewed expectations into
  target-specific files without overwriting user code, and bind observations to
  acceptance. Retry, watch, grids, imports and general execution remain in their
  existing contract-first Warps.

  defines: demo.blackjack.improvements.scaffolding reviewable generated tests and bindings
  consumes: recur.lang.work.remaining.scaffolding existing companion follow-up

The bounded `recur-lang plan` implementation is tracked separately in
`warps/main.lang.implementation-plan.contract.md`; these checkboxes stay open.
They describe measured boundaries, not promises that every language proposal
ships in this demo. Split any item into a focused Eventness todo while retaining
these trace identities; append `.checked` only after its actual acceptance.
