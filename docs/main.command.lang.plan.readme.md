# recur-lang plan

defines: recur.lang.plan source-bound implementation and test advice

```powershell
recur-lang plan main.blackjack.05.session.recur -d demos/blackjack-lab --json
recur-lang plan main.blackjack.06.coordination.recur --scope settle -d demos/blackjack-lab
```

This companion command prepares an opinionated checklist from existing WIR1,
CIR1 and SGR1 reports. It does not execute a binding, invoke an LLM, scaffold
code, run tests or accept a Warp. The embedded query retains coverage exclusions
and the complete CIR graph even when the visible work is scoped to one lane.

The nearest `.recur/config.toml` supplies the preferences initialized by
`recur-lang init`; absent preferences use the same defaults in memory. Planning
does not create that config. Set `[recur-lang] target = "julia"` (or your chosen
target) explicitly. The packet retains source/config fingerprints and effective
preferences without exposing unrelated config contents.

- `specification_first` chooses specification/tests/implementation phases; an
  explicit false chooses implementation/tests.
- `prioritize_graph_findings` places a repair item first or last. Findings still
  block the plan regardless of their position.
- `include_test_plan` controls suggested behavioral, contract, join and cycle
  checks. An opt-out does not establish that validation is unnecessary.

Work items follow declaration order, not a computed execution schedule. Test
suggestions reference declared behavior; humans or agents must supply precise
expected outcomes and semantic invariants. Opaque types do not establish rules
such as blackjack payouts, money conservation or a maximum player count.

The dependency-conformance suggestion explicitly asks a reviewer to compare
private helpers, closures and callback targets against a Lang reference, map
missing call edges, check for cycles and retain source-bound findings in
Eventness. The command does not infer those calls itself. The blackjack demo
provides a 19-function reference and a deliberately incomplete review record
that distinguishes default bindings from unknown custom callbacks.

JSON uses `recur-lang-implementation-plan-v1`. `planned` and `needs-target`
exit 0; `blocked` static findings exit 1. Input/config errors exit 2 with their
core LANG or initialization LINIT diagnostics. Unchanged inputs produce the
same packet; no timestamps or random IDs are added. Fingerprints are observed
input identities, not an atomic filesystem snapshot or future freshness guarantee.

See the [bounded contract](../warps/main.lang.implementation-plan.contract.md)
and [blackjack experiment](../demos/blackjack-lab/main.blackjack.readme.md).
