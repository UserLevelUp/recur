# Improvement 30 Watch Coordination Contract v0

Status: `todo.future-plan` (Level 0/1 read-only concurrent IR implemented;
watch/runtime coordination not implemented)
Date: 2026-07-24
Source: `demos/main.lang/main.lang.skippy-watch-coordination.recur`

## Purpose

Define a precise, progressively disclosed model for one coordinator and
multiple long-lived intelligence lanes that communicate through filesystem
Eventness and `recur-watch`.

This contract separates three things that are easy to blur together:

```text
watcher process   = recur-watch remains subscribed and emits file events
lane controller   = watching / working / awaiting-guidance state machine
intelligence      = human or LLM that performs one bounded work order
```

An intelligence may move from watch mode to work mode, but its watcher process
does not need to stop. Events arriving during work may queue. Instruction IDs
and receipts make those duplicate or delayed notifications safe to process.

Normative terms `MUST`, `MUST NOT`, `SHOULD`, and `MAY` describe the intended
contract. The current Julia `main.lang` parser does not yet accept this syntax.
The Rust `recur-lang-concurrent-ir-v1` parser now accepts only named contracts,
coordinator output ports, lane message/policy declarations, and the compact
fork/await flow. It performs no scheduling, watching, receipt acceptance, or
state-machine execution.

## Level 0: Compact Process

The default view should fit in one small diagram:

```text
                                      +-> csharp-monkey --+
change -> Skippy.plan -> fork --------+-> web-monkey -----+-> await
                                      +-> test-bird ------+
                                                               |
                                                               v
              merge-ready <- Skippy.decide <- git-monkey <- review-bird
```

Equivalent compact source:

```recur
solution async :
  i(a)
  -> skippy.plan(a)
  -> fork [csharp_monkey(a), web_monkey(a), test_bird(a)]
  -> await [csharp_monkey.o(b), web_monkey.o(b), test_bird.o(b)]
  -> review_bird(a)
  -> git_monkey(a)
  -> skippy.decide(a)
  -> o(b)
```

This view answers who works, which work is concurrent, where the joins occur,
and which block owns the final decision. Everything else is drill-down.

## Level 0A: Living Master Work Report

The same graph MUST project into a master grid:

```text
Lane             Current block       State       Evidence
------------------------------------------------------------
csharp-monkey    csharp.f(a)          WORK        2 files
web-monkey       web.f(a)             PRODUCED    npm.test ACK
test-bird        test.f(a)            BLOCKED      question.002
review-bird      review.i(a)          WAIT 2/3     -
git-monkey       git.i(a)             WATCH        -
------------------------------------------------------------
Overall          implementation       ACTIVE       merge-ready: false
```

A lane cell contains the lane and host identity, compact block, controller
mode, Eventness state, round, attempt, watcher ACK, dependency readiness,
evidence, blockers, and last event time.

The grid MUST NOT store independent coordination truth. It is reconstructed
from:

```text
canonical AST
watcher status
immutable WorkOrders
worker receipts
Eventness transitions
```

The pure query renders one snapshot and exits:

```powershell
recur lang grid solution
recur lang grid solution --json
```

The companion may keep the same projection alive:

```powershell
recur-lang coordinate `
  demos/main.lang/main.lang.skippy-watch-coordination.recur `
  --view grid
```

Restarting the companion MUST reconstruct an equivalent normalized JSON grid
from durable files. Watcher events wake the renderer, but the renderer rereads
and validates those files before changing a cell.

At completion:

```text
solution.coordination.current -> solution.coordination.complete
```

The completed report preserves the final cells and their underlying event
timeline. Color MAY decorate a live view but MUST NOT be the only state signal.

## Level 1: Exact Lane Contracts

Every lane MUST declare:

- one exact canonical input reference;
- one output receipt contract;
- allowed read and write patterns;
- allowed external tool identifiers;
- required evidence receipts;
- dependencies on prior receipts;
- a familiar description of its bounded work.

`allow write` governs project artifacts. A footer `channel` separately grants
the publisher permission to create only its declared protocol prefix inside
its own lane directory. Publishing `worker.*` receipts therefore does not
silently widen a lane's project write scope.

Example:

```recur
lane web_monkey {
  i(a) := project skippy.plan.o(b).orders["web_monkey"]
  o(b) := WorkReceipt
  f : i(a) -> o(b) ~ "Implement the bounded Angular or React slice"

  allow read  ["src/Web/**", "tests/Web/**", "docs/**"]
  allow write ["src/Web/ClientApp/**"]
  allow tools ["editor", "recur", "recur-watch", "npm"]
  require receipt ["npm.lint", "npm.test", "npm.build"]
}
```

The projection in `i(a)` MUST retain the canonical identity of the selected
`WorkOrder`. Copying its fields into a separately declared record is not an
exact alias.

## Level 2: Watch Topology

Each worker receives a private lane directory:

```text
.recur/runs/<run-id>/lanes/<lane>/
```

Skippy is the only publisher of coordinator messages:

```text
coord.<lane>.<round>.instruction.current.md
coord.<lane>.session.complete.md
```

The worker is the only publisher of worker messages:

```text
worker.<lane>.<round>.ack.current.md
worker.<lane>.<round>.question.current.md
worker.<lane>.<round>.receipt.current.md
worker.<lane>.<round>.duplicate.ignored.md
```

Skippy watches the run root recursively:

```powershell
recur-watch --id skippy `
  --filter "worker.**.current.md" `
  --dir ".recur/runs/<run-id>" `
  --format json `
  --poll-framing 2
```

Each worker watches only its private lane directory:

```powershell
recur-watch --id web-monkey `
  --filter "coord.**" `
  --dir ".recur/runs/<run-id>/lanes/web_monkey" `
  --format json `
  --poll-framing 2
```

The prefix split is load-bearing:

```text
Skippy writes coord.*   and watches worker.*
workers write worker.*  and watch coord.*
```

A participant MUST NOT subscribe to the prefix it publishes. This prevents
self-trigger amplification. The static `self_subscription` check rejects that
topology.

The file extension is part of the watched hierarchical name. A filter ending
in `current` does not match a filename ending in `current.md`; the coordinator
therefore uses `worker.**.current.md`. A terminal `**`, as in `coord.**`,
matches all remaining segments including the extension.

`--poll-framing 2` selects crash-tolerant polling. Omitting it selects the
lower-latency filesystem event stream. Both modes use the same file protocol
and JSON event shape.

## Watcher Readiness Gate

With `--id`, each runner writes:

```text
.recur/watch/recur-watch.<id>.status.current.md
```

Before the first instruction is published, the coordinator MUST observe every
required watcher in:

```text
state = active
ack = accepted
```

The pure query is:

```powershell
recur watch status web-monkey --json
```

`recur watch` MUST NOT be confused with `recur-watch`:

```text
recur-watch = active blocking subscription process
recur watch = read watcher Eventness and exit
```

This gate prevents the first instruction from racing ahead of subscription
setup.

## Level 3: Worker State Machine

The normative worker states are:

```text
watching -> claimed -> working -> produced -> watching
                 \-> awaiting-guidance -> claimed
watching/produced -> stopped
```

Transition table:

| Current | Event | Guard | Next | Required effect |
|---|---|---|---|---|
| `watching` | instruction event | unseen idempotency key | `claimed` | Validate order; publish ACK |
| `watching` | duplicate event | ACK or receipt exists | `watching` | Ignore and record duplicate |
| `claimed` | contract accepted | hashes and scope valid | `working` | Project bounded context |
| `claimed` | contract rejected | invalid or stale order | `awaiting-guidance` | Publish question/NAK |
| `working` | tools complete | required receipts present | `produced` | Publish work receipt |
| `working` | blocked | unresolved decision | `awaiting-guidance` | Publish question |
| `awaiting-guidance` | new round | new round ID | `claimed` | Validate the new order |
| `produced` | receipt durable | content hash verified | `watching` | Mark idempotency key complete |
| `watching` | session complete | no active work | `stopped` | Exit cleanly |
| `produced` | session complete | receipt already durable | `stopped` | Exit cleanly |

`coord.session.complete` is a drain signal, not an emergency cancellation. If
it appears during `working`, the controller SHOULD finish or explicitly NAK
the current round before stopping. A separate future `coord.session.abort`
event would require its own cancellation contract.

## Watch Mode and Work Mode

Watch and work are lane-controller modes, not two different LLM personas:

```text
watch mode:
  consume event
  inspect instruction identity
  reject stale or duplicate work

work mode:
  load only declared context
  use only allowed tool families
  produce artifacts and evidence
  validate changed paths
  publish one receipt

return to watch mode
```

The `recur-watch` child process MAY remain armed during work. Its stdout is an
event stream, not the sole source of truth. After every event, restart, or
timeout, the controller MUST rescan the lane directory for unanswered
instructions:

```text
instruction exists AND no ACK/receipt exists for its idempotency key
```

This makes the protocol recoverable from disk rather than dependent on
session-local memory.

## Level 4: Work Order

A valid `WorkOrder` contains:

```text
run_id
round_id
lane
objective
context references
allowed read patterns
allowed write patterns
allowed tool identifiers
acceptance criteria
receipt dependencies
reply path
source hash
idempotency key
```

An instruction MUST be self-contained enough that the lane does not have to
invent its scope. References may be expanded, but an undeclared reference
cannot silently become an input.

The idempotency key SHOULD be derived from:

```text
run-id + lane-id + round-id + instruction-content-hash
```

An amended instruction MUST use a new round ID. Closed instructions and
receipts MUST NOT be overwritten.

## Level 5: Work Receipt

A valid `WorkReceipt` contains:

```text
run_id
round_id
lane
instruction reference and hash
verdict
artifact references
commit references
changed file paths
external tool receipts
questions
unresolved issues
start and completion timestamps
```

The coordinator MUST reject a receipt when:

- its instruction hash does not match;
- its run, lane, or round does not match the instruction;
- a changed file falls outside `allow_write`;
- a required tool receipt is absent;
- a declared dependency receipt is missing or rejected;
- its artifact or commit reference cannot be resolved;
- the source hash is stale and policy does not allow rebase/revalidation.

A valid shape is not proof of correctness. The receipt is evidence presented
to the integration and verification lanes.

## External Tools

Recur Lang names tool capabilities but does not implement their toolchains.

```text
csharp-monkey -> external dotnet format/test
web-monkey    -> external npm lint/test/build
test-bird     -> external focused test runners
review-bird   -> external integration tests and diff inspection
git-monkey    -> external git/recur-git integration
```

The worker environment runs those commands. It returns `ToolReceipt` records.
Neither `recur` nor `recur-lang` needs to contain `dotnet`, Node, Cargo, Git,
a browser, a compiler, or a linker.

## Coordinator Semantics

Skippy MUST:

1. validate the static graph;
2. wait for watcher readiness;
3. publish one immutable order per initially ready lane;
4. wait asynchronously for ACK, question, and receipt events;
5. answer questions with a new round rather than mutating a closed round;
6. dispatch `review_bird` only after required implementation receipts exist;
7. dispatch `git_monkey` only after review acceptance;
8. produce either a merge-ready candidate or a bounded NAK report;
9. publish a session-complete event to every lane.

Skippy MUST NOT:

- write implementation files;
- run target-language toolchains;
- invent a successful receipt;
- silently waive a missing acceptance criterion;
- merge merely because all workers returned;
- treat a watcher notification as proof that a complete file was read.

## Atomic Publication

A producer SHOULD write a complete message to a temporary non-matching name,
flush it, and atomically rename it to its final Eventness filename.

```text
temporary: .coord.web-monkey.001.instruction.pending
final:     coord.web-monkey.001.instruction.current.md
```

Consumers MUST tolerate create/modify notification bursts. They validate the
message hash and idempotency key before acting. The filesystem is the durable
event log; watcher output is only a wake-up signal.

## Joins and Feedback

An `await` joins exact receipt references, not a count of anonymous workers.
For example:

```recur
await [
  csharp_monkey.o(b),
  web_monkey.o(b),
  test_bird.o(b)
]
```

The join remains incomplete if any required lane is missing, rejected, or
stale. Optional lanes must be declared optional in the input contract.

Repair cycles are illegal unless explicit and bounded:

```recur
feedback repair {
  from review_bird.o(b) where verdict == rejected
  through skippy.plan(a)
  to affected lanes
  until review_bird.o(b).verdict == accepted
  limit ChangeRequest.attempt_limit
}
```

Static cycle reporting distinguishes this feedback declaration from accidental
dataflow or wait cycles.

## Static Checks

The fixture requests:

```recur
check contract_identity
check circular_ref
check wait_deadlock
check unreachable_lane
check conflicting_write_scope
check self_subscription
check required_receipts
```

The generated report should include at least:

```recur
report {
  orchestration_sound: true
  circular_ref: false
  circular_refs: []
  wait_deadlock: false
  unreachable_lanes: []
  conflicting_write_scopes: []
  required_evidence_complete: false
  spec_escape: false
  merge_ready: false
}
```

`required_evidence_complete` and `merge_ready` normally remain false before
runtime receipts arrive even when the static graph is sound.

## Progressive Disclosure

All views MUST derive from one canonical parsed model:

| View | Question answered |
|---|---|
| Compact flow | Who runs, in what order, and where do we wait? |
| Living grid | What block and state does every lane occupy right now? |
| Lane contract | What may this intelligence read, write, use, and produce? |
| Watch topology | Which files wake which controller? |
| State machine | How does the lane move between watching and working? |
| Work order | What exact task and evidence were assigned? |
| Receipt | What happened, what changed, and what remains unresolved? |
| Report | Is the orchestration sound and is integration justified? |
| Expansion | What detailed behavior implements this symbol? |

Hover and expansion are conveniences. The same information MUST remain
queryable in text and JSON without an IDE.

## Grammar Outline

This is a structural outline, not yet a frozen parser grammar:

```ebnf
program       = declaration, header, body, footer ;
declaration   = "recur", version, "coordination", identifier ;
header        = "header", "{", { contract | coordinator | lane }, "}" ;
contract      = "contract", identifier, "{", { field }, "}" ;
coordinator   = "coordinator", identifier, "{", { scope | policy }, "}" ;
lane          = "lane", identifier, "{", lane_input, lane_output,
                function, { policy }, "}" ;
lane_input    = "i", "(", symbol, ")", ":=", contract_reference ;
lane_output   = "o", "(", symbol, ")", ":=", contract_reference ;
function      = symbol, ":", input_reference, "->", output_reference,
                "~", string ;
body          = "body", "{", { flow | grid | feedback | machine | expansion }, "}" ;
flow          = identifier, ("sync" | "async"), ":", flow_expression ;
grid          = "grid", identifier, "by", identifier, "{", grid_projection, "}" ;
feedback      = "feedback", identifier, "{", feedback_rule, "}" ;
machine       = "machine", identifier, ["(", identifier, ")"],
                "{", { state | transition }, "}" ;
footer        = "footer", "{", { watcher | channel | check | report |
                event | warp }, "}" ;
```

The AST and JSON representation must be versioned before this outline becomes
accepted syntax.

## Failure and Recovery Cases

The first executable contract tests should cover:

1. all watchers ready before the first dispatch;
2. three independent initial lanes complete out of order;
3. duplicate create/modify notifications execute one round once;
4. a worker restarts and rediscovers an unanswered instruction;
5. a worker publishes a question and receives a new-round answer;
6. an out-of-scope changed path is NAKed;
7. a required tool receipt is absent;
8. review rejects and one bounded repair round succeeds;
9. repair exceeds its attempt limit;
10. an accidental wait cycle is rejected before dispatch;
11. session completion drains workers cleanly;
12. watcher state and coordination state remain separately queryable.
13. process restart reconstructs an equivalent normalized JSON grid;
14. the completed report preserves the live grid's transition history.

## Command Boundary

```text
recur lang   = inspect and statically check this program
recur-lang   = coordinate declared lanes and validate file receipts
recur-watch  = active blocking filesystem subscription
recur watch  = pure watcher-state query
workers      = perform implementation using external tools
```

None of these declarations authorize a commit, merge, deployment, or
target-language command unless the relevant external worker and policy
explicitly permit it.

## Discovery

```powershell
recur tree "main.improvement.30" -d docs/
recur files "main.lang.skippy-watch-coordination" -d demos/main.lang/
recur trace-id "main.improvement.30.**" --scope "**" --ext ".md" -d .
recur watch explain
```

## Trace-Id Lines

```text
defines: main.improvement.30.contract.watch-coordination-v0 formal watch work receipt and join contract for multiple intelligence lanes
defines: recur.lang.worker-state-machine watching claimed working awaiting-guidance produced and stopped transitions
defines: recur.lang.watch-topology coordinator writes coord and workers write worker namespaces without self-subscription
defines: recur.lang.master.work.report living lane grid and completed report projected from the same durable coordination history
consumes: main.command.watch active recur-watch runner and pure recur watch status query
consumes: main.improvement.30 Recur Lang coordination control-plane proposal
produces: main.lang.skippy-watch-coordination progressively disclosed design fixture
triggers: main.improvement.30.contract future versioned AST JSON and diagnostic freeze
```
