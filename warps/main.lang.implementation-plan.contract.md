# Bounded companion implementation plan v1

defines: recur.lang.plan.v1 deterministic source-bound implementation and test advice
consumes: demo.blackjack.session observed repeated-state and testing needs
consumes: recur.lang.init.configuration editable planning preferences

Implement `recur-lang plan SOURCE [--scope SYMBOL] [-d ROOT] [--json]`.
Reuse the core report projector for WIR1/CIR1/SGR1; do not parse a second grammar
or invoke a sibling executable, binding, LLM, test runner or writer. SOURCE and
scope obey existing core read boundaries and errors. Planning opinions belong
only in the companion. A core library report API may expose the existing pure
projector without changing core CLI behavior.

Read the nearest project policy using init's discovery/validation conventions.
Absent keys use init defaults in memory, with no config or directory creation.
Preserve source and configuration bytes. Include effective preferences, config
path, actual config fingerprint (null when absent), and effective-policy hash.
Keep unrelated config contents out of the returned packet. Unknown preferences
remain inert. Malformed known fields use existing LINIT diagnostics and exit 2.

Return `recur-lang-implementation-plan-v1` with the original query packet,
source/hash, policy, ordered phases, work items, graph findings, test plan,
unresolved decisions and `execution: not-run`. Each selected header symbol gets
one implementation item retaining its declared contract/binding data. If static
findings exist, include a repair item and state `blocked`, exit 1. Scope selection
must retain the unfiltered CIR graph and cannot hide cycles. `prioritize_graph_findings`
controls the repair item's placement, never whether findings block readiness.
Without findings, use `needs-target` when target is unspecified, otherwise
`planned`; both exit 0 because the advisory packet was produced.

`specification_first=true` orders specification, tests, implementation; false
orders implementation, tests. `include_test_plan=false` leaves suggested tests
empty and explicitly records that opt-out. With tests enabled, include separate
behavioral tests per symbol, interface/alias tests, and CIR join/producer and
cycle-fault tests. These are suggestions, not generated assertions or observed
passes. Always flag excluded grammar and the need for runtime evidence and
human decisions about semantic invariants. Do not infer card rules or arbitrary
implementation-only dependencies from opaque type names.

JSON must be deterministic for unchanged inputs; no timestamps/random IDs.
Text must show state, target, selected work, suggested checks and limitations.
Input errors preserve core/initialization diagnostics in structured JSON and
exit 2. This increment adds no scaffolder, scheduler, general execution, advanced
grammar or acceptance/transition authority.

Acceptance: tests fail against the previous companion; then verify both IRs,
scoped cycles, preferences and opt-outs, no mutation, invalid config, source
boundaries, deterministic repeats, source/config fingerprint drift and text
output. Run existing companion init/warp and core query regressions. Record
the actual candidate; historical receipts for changed source may become stale.
