# Blackjack dependency reference and LLM conformance review

defines: demo.blackjack.dependencies.review source-bound comparison of Julia to ideal Lang contracts
consumes: demo.blackjack.dependencies.reference private-helper input/output and dependency reference
consumes: demo.blackjack.session implementation under review
triggers: demo.blackjack.dependencies.reassessment changed source requires renewed review

Status: **reviewed for the recorded default implementations; not a blanket
cycle-free certificate**. Reviewer: Codex (author self-review, not independent
audit). Exact reviewed source/reference hashes are in
`dependencies/main.blackjack.dependencies.review.json`.

The ideal reference is
`dependencies/main.blackjack.dependencies.recur`. Its adjacent registry maps
19 named functions/constructors/local helpers to files, arguments, results and
direct helper calls. This includes private scoring, validation, allocation,
receipt and draw helpers rather than stopping at public class boundaries.
Registry file paths and descriptive review-lane read policies use the parent
`demos/blackjack-lab` directory as their review context. Core Lang does not enforce
these policies or resolve them as Julia imports.

Each helper has `nameInput` and `nameResult` contracts. Its review lane's `i(a)`
collects its work order and the required helper reviews; `o(b)` returns a
`ReviewReceipt`. The body describes review prerequisites, not execution of the
game. CIR1 checks this review graph. It does not automatically bind the documented
Input/Result contracts to Julia values or inspect source implementation calls.

## Review procedure for an LLM or human

1. Read the selected source files and expand each input/output bundle. Verify
   private helpers, constructors, closures, broadcasts, calls in comprehensions,
   module includes and default callback bindings—not just public entry points.
2. Compare each real call with the registry and the declared helper dependency.
   Record a missing edge as a mismatch. Add a justified edge to the model only
   after deciding whether the implementation or intended design is wrong.
   Do not edit the specification merely to make faulty code conform.
3. Run `recur lang check` on the resulting reference. A newly recorded
   `score -> session` call creates a closed cycle through play or settlement.
   The regression test demonstrates this mapping with an altered Julia fixture.
4. Distinguish calls from state feedback and captured variables. A session's
   next-round state is data, not a recursive call from score back to session.
   Score's ace loop decreases the remaining ace count; hit loops consume a
   finite deck; the session loop has an explicit round budget. Review these
   termination conditions separately from dependency acyclicity.
5. Record mismatches, unknown targets, scope exclusions and the reviewed source
   hashes in Eventness. A source/reference change makes this observation stale
   and requires a new review; refreshing hashes alone is not review evidence.

## Current findings and open decisions

- Default named helper calls agree with the recorded reference and are acyclic
  within that model. The demo tests compare the model edges with this reviewed
  registry and reject stale source hashes. This is a check on review consistency,
  not an independent extraction of Julia's call graph.
- The loader includes stages 01–06 and the app includes the loader. None of the
  stage implementation files includes the app or loader. Runtime-loaded or
  externally modified modules are outside this observation.
- Custom `player_worker` / `house_worker` callbacks have unknown call effects
  until their implementations are also mapped. This review covers their default
  bindings only. Do not generalize its verdict to arbitrary callbacks.
- Julia/Base/Random internals, dispatch under arbitrary future types,
  metaprogramming and external native code are outside this bounded reference.
- The existing scalar-to-single-field packing convention remains explicit;
  nested semantic types and game invariants still need behavioral tests.

- [ ] Review any custom callback before widening the accepted scope.
- [ ] Obtain independent review if stronger assurance than author self-review
  is needed; retain both observations and unresolved disagreements.
- [ ] Define the general companion review-packet/receipt protocol in
  `main.lang.binding-correspondence`, using this example first. No Julia parser
  or automatic binding executor is required for that reference-first increment.

```powershell
recur lang check main.blackjack.dependencies.recur -d demos/blackjack-lab/dependencies --json
recur lang show main.blackjack.dependencies.recur --scope score -d demos/blackjack-lab/dependencies --expand
recur trace-id demo.blackjack.dependencies --scope 'main.blackjack.**' -d demos/blackjack-lab --format full
```
