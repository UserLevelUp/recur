# Implementation-plan verification

defines: recur.lang.plan.verification observed bounded companion acceptance
consumes: recur.lang.plan.v1 deterministic plan contract
consumes: demo.blackjack.observations integration evidence

The previous companion failed the two initial plan probes because it had no
`plan` subcommand. See `lang-blackjack/blackjack-plan-red.log`.

The final companion exposes deterministic implementation/test advice using the
core report projector and nearest init policy. Tests cover WIR/CIR, scoped graph
findings, preference opt-outs, errors, no writes, source/config fingerprints,
text output and dependency-conformance review guidance. It does not execute
bindings, inspect Julia automatically, generate code, or confer acceptance.

Observed acceptance on 2026-09-30:

- Full Julia suite, final installed binaries: 19,332 passed, 73 expected-broken,
  exit 0, including 41 plan assertions and 13,833 blackjack assertions.
- Full Cargo suite: 257 passed, 7 ignored, exit 0; final companion native rerun
  after the last advice text change: 14 passed, exit 0.
- All seven final binaries installed successfully. Failed compiler attempts are
  preserved and excluded from successful evidence.

Raw evidence and the explicitly scoped input/binary fingerprints are in
`lang-blackjack`. The demo observations describe compiler workarounds, actual
contract mismatches, the mapped hidden cycle and limits of author self-review.

These Warp gates use **declared evidence**, not independently checked test
receipts. The producer observed the runs above and binds this report as its
declaration. Other advanced and binding-correspondence Warps remain open.
