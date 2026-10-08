# Blackjack Lang experiment: observed results

defines: demo.blackjack.observations tested contracts, implementation and review boundaries
consumes: demo.blackjack.dependencies.review source-bound author review
consumes: recur.lang.plan.v1 companion advice

Observed 2026-09-30 on branch `recur-lang`, based on commit
`2ecae66df03c6e7a4dd783ba3e7bd389e53d245a` plus the uncommitted experiment.
Raw logs and scoped SHA-256 fingerprints are under `warps/lang-blackjack`.

## Results

- Complete Julia suite: **19,332 passed, 73 expected-broken**, exit 0.
- Standalone blackjack suite: **13,833 passed**, exit 0.
- Native Cargo suite: **257 passed, 7 ignored**, exit 0. After the final
  dependency-review advice text changed, all 14 companion native tests passed
  again and the complete Julia suite exercised the final installed binaries.
- Companion plan has 41 assertions in that final Julia run. Its initial tests
  failed twice against the previous companion, which lacked the command.
- Each of six numbered contracts and tests preceded its implementation;
  the retained stage baselines fail because those implementation files were
  absent. These are scaffold baselines, not six demonstrated algorithm bugs.
- All seven final binaries were built and installed locally. The command is
  `recur-lang plan SOURCE`; it returns advice and never executes Julia.

The seeded eight-round example with four players, 100 starting chips, wager 10,
dealer Ada, house bank 1000 and seed 7 finishes with balances
`[120, 80, 120, 75]`, joint leaders 1 and 3, and house bank 1005.
The complete runtime test includes 180 seeded sessions and all 1,326 distinct
two-card score combinations. These observations do not establish coverage of
every possible game state or every Lang feature.

## Faults that improved the reference

Five structural result comparisons caught three real contract mismatches:
the table's money field name and missing configuration fields in round and
session results. The contracts were corrected against the intended API, and
those regressions remain. Scalar/vector single-field packing remains a stated
convention rather than an automatic general binding validator.

An implementation-only cycle is invisible to an unchanged Lang file. Stage 09
therefore adds ideal input/result contracts and a reviewed call registry for 19
functions, constructors and local helpers. Injecting a call from Julia `score`
to `tournament` makes the recorded review stale. Explicitly mapping that call
into the reference then produces SGR001 with the closed path
`play -> session -> score -> play`. Scope filtering cannot hide that finding.

This is a reference-first LLM/human workflow. The registry was authored after
reading the implementation; the tests compare the model to that registry and
check recorded hashes. They do not independently extract Julia calls. Custom
callbacks, arbitrary dispatch, metaprogramming and external internals remain
unknown/out of scope. The Eventness review is deliberately `overall_complete:
false`, marked author self-review, and retains its unresolved work.

The source-bound checked Warp demo uses two actually observed session cases,
preview/confirmation, accepted replay recovery, and stale-input rejection in
temporary roots. It is not evidence of full language or application correctness.

## Toolchain and failed attempts

Juliaup 1.12.7 ran with `--startup-file=no -O0 -C generic` and the
`demos/web-evidence-lab` JSON3 environment. The demo assertion module uses
minimal compilation with inference disabled after reproduced inference/LLVM
faults in large test closures; the game module retains ordinary compilation.
World-age invocation and an incorrect legacy scope in early tests were fixed.
Failed attempts remain in separate logs, not counted as successful runs.

Rust 1.85.0 used `release-safe`, one build job, incremental compilation disabled,
64 MiB Rust stack, package codegen-units 1 and final package opt-level 0. Two
earlier opt-level 1 installation attempts failed with compiler heap corruption.
Final compilation, installation and regressions succeeded with the package
override. No Python was used. Historical receipts for changed source are
preserved and may now be stale; their hashes were not refreshed to imply review.

## Remaining work

The bounded planner improvement is implemented and tested. Reusable contracts,
the general source-to-reference review packet, and dynamic scatter/gather remain
separate contract-first Warps. See `main.blackjack.improvements.todo.md` and
`main.blackjack.dependencies.review.todo.current.md` for the current scope.
