# Improvement 30: Concurrent IR v1

Status: `todo.current`
Parent: `README.CORE.IMPROVEMENT30.md`
Slice: `1 / first concurrent coordination sub-slice`
Date: 2026-07-24
Fixture: `demos/main.lang/main.lang.skippy-watch-coordination.recur`

## Manual Warp

```text
E0(main.improvement.30.concurrent-ir.todo.current)
  -> dE(freeze lanes, exact messages, fork, and ordered await joins)
  -> Ef(main.improvement.30.concurrent-ir.contract.complete)
```

This Eventness transition is manual. Parser output is evidence, not authority
to declare the slice complete.

## Communication Goal

Independent lanes remain useful only when their communication boundaries are
exact and rediscoverable. The first concurrent IR therefore answers:

```text
Who can run independently?
What exact message does each lane consume and produce?
Which messages satisfy each await?
Which lane or coordinator block consumes the joined result?
```

## Goldilocks dE

Freeze `recur-lang-concurrent-ir-v1` for the fixture's Level 0 and Level 1
surface:

- named message contracts and fields;
- coordinator output ports needed by lane inputs;
- lane identity, persona, input dependencies, output message, and function;
- read, write, tool, and required-receipt declarations;
- one `solution async` compact flow;
- the initial fork;
- ordered exact `await` joins and their next consumers;
- source hash and source spans;
- stable diagnostics for unknown lanes, missing joins, and mixed join contracts.

## Acceptance Criteria

- The five fixture lanes parse in source order.
- Initial implementation lanes are exactly `csharp_monkey`, `web_monkey`, and
  `test_bird`.
- Each initial lane consumes its projected `WorkOrder`.
- Each lane produces an exact `WorkReceipt` message.
- The first await contains the exact three forked receipt ports.
- `review_bird` consumes those three receipts plus its projected order.
- `git_monkey` consumes `review_bird.o(b)` plus its projected order.
- The final await feeds `skippy.decide`.
- JSON serialization is deterministic across repeated parses.
- Unknown fork lanes, omitted fork receipts, and mixed await contract types are
  rejected with stable diagnostic codes.
- No worker is scheduled and no file outside tests is mutated by parsing.

## Non-Goals

- no watcher or channel parsing;
- no state-machine execution;
- no work-order publication;
- no receipt acceptance;
- no repair feedback loop;
- no live grid;
- no deadlock or write-scope analysis beyond this exact compact flow;
- no automatic Eventness completion.

## Frozen Contract

Schema:

```text
recur-lang-concurrent-ir-v1
```

Top-level JSON shape:

```json
{
  "schema": "recur-lang-concurrent-ir-v1",
  "language_version": "0.2",
  "coordination_name": "SkippyWorkshop",
  "source": "main.lang.skippy-watch-coordination.recur",
  "source_hash": "fnv1a64:...",
  "contracts": [],
  "coordinator_ports": [],
  "lanes": [],
  "flow": {
    "name": "solution",
    "mode": "async",
    "expression": "i(a) -> ... -> o(b)",
    "fork_lanes": [],
    "awaits": [],
    "span": {}
  }
}
```

Each lane preserves:

```text
name
persona
input symbol and source expression
resolved input message references
output symbol and typed message port
function identity and familiar name
allow-read patterns
allow-write patterns
allowed tool identifiers
required receipt identifiers
source span
```

Each message reference preserves:

```text
identity             csharp_monkey.o(b)
producer             csharp_monkey
contract             WorkReceipt
projection           null
```

A projected coordinator message instead preserves the selected canonical
contract:

```text
identity             skippy.plan.o(b)
producer             skippy.plan
contract             WorkOrder
projection           orders["web_monkey"]
```

Each await contains exact typed message references, its next consumer, and its
source span. Vector order follows source order; JSON is therefore deterministic
without sorting away the authored communication sequence.

Source spans and source hashing use the conventions frozen by
`recur-lang-warp-ir-v1`.

### Static invariants in v1

- every fork member is a declared lane;
- the first await producers exactly equal the initial fork lanes;
- one await cannot mix canonical message contracts;
- an await before a lane exactly matches that lane's receipt dependencies;
- every downstream consumer is a declared lane or coordinator scope;
- every lane is reachable from the compact flow.

### Stable diagnostics

| Code | Meaning |
|---|---|
| `RCIR001` | Missing or duplicate coordination declaration |
| `RCIR002` | Invalid, duplicate, empty, or unclosed named contract |
| `RCIR003` | Invalid coordinator or coordinator output port |
| `RCIR004` | Invalid or duplicate lane declaration |
| `RCIR005` | Invalid lane port, function, policy, or message reference |
| `RCIR006` | Invalid compact async flow, fork, or await syntax |
| `RCIR007` | Unknown lane, producer port, or downstream consumer |
| `RCIR008` | Fork/await or await/consumer dependency mismatch |
| `RCIR009` | One await mixes incompatible canonical message contracts |
| `RCIR010` | Declared lane is unreachable from the compact flow |

Diagnostics serialize with `code`, human `message`, and optional `span`.
Consumers branch on the code rather than parsing message text.

## Discovery

```powershell
recur files "main.lang.skippy-watch-coordination" -d demos/main.lang/
recur tree "main.improvement.30.concurrent-ir" -d docs/
recur trace-id "recur.lang.concurrent.ir.v1" --scope "**" --ext ".md" -d .
```

## Trace-Id Lines

```text
defines: recur.lang.concurrent.ir.v1 exact lane message fork and await model for the first 0.2 coordination subset
consumes: recur.lang.warp.ir.v1 shared source hash span and diagnostic conventions
consumes: main.improvement.30.contract.watch-coordination-v0 design fixture
produces: main.improvement.30.concurrent-ir.contract stable read-only communication graph
triggers: main.improvement.30.static-graph wait cycle reachability and join analysis
```
