RECUR IMPROVEMENT 30
Recur Lang Coordination Contracts
=================================
Date: July 24, 2026
Status: Active incremental implementation / IR foundation in progress
Author: Captured from Recur Lang and multi-intelligence orchestration design
Definition: IR means Intermediate Representation.

INTENT
------
Add a small, language-independent formalism to Recur so one human or
intelligence can understand bounded work clearly, and multiple entities can
optionally divide work into lanes, join results, and inspect whether the
coordination logic is sound.

OWNERSHIP BOUNDARY
------------------
Improvement 27 owns the generic Eventness Warp methodology and proposed
`recur warp` project-control surface. Improvement 30 owns Recur Lang: the
condensed, checkable mapping of exact inputs, functions or methods, outputs,
and their relationships.

Recur Lang may consume a Warp, bounded Slice, or receipt as a declared
contract. Its Warp IR records the exact subset needed by the language and its
receipt boundary; it does not transfer ownership of Warp methodology into
Improvement 30. See `README.CORE.IMPROVEMENT27.Appendum.md` for the clarified
coordination mental model.

The compact Recur Lang form is:

```text
i(a) -> f(a) -> o(b)
```

Read it literally from left to right as one small block function:

```text
exact input  ->  named function/work slice  ->  exact output
   i(a)      ->            f(a)             ->    o(b)
```

The input comes first because it is what the block is allowed to receive. The
function sits in the middle because it is the bounded transformation or work.
The output comes last because it is the fact the next block may consume. A
reader should be able to follow the characters with their eyes and understand
the logical direction without expanding the block: **input, then function,
then output**.

where:

- `i(a)` is the exact input contract for the block;
- `f(a)` is a compact function or work-slice symbol;
- `o(b)` is the exact output contract;
- a downstream `i(b)` may alias the same canonical contract as `o(b)`;
- expansion and contraction are lossless views over one parsed model.

### One input or a compact multi-input bundle

A function may have one simple input, or one exact input bundle with several
named sub-inputs. The visual flow still stays left-to-right: all required facts
enter through an `i(...)` port before the function runs, and one `o(...)` port
leaves after it completes.

```text
# One input
i(a) := LevelRequest
i(a) -> f(a) -> o(b)

# Several inputs from different upstream blocks, collected into one input port
i(c) := (
  request: request_reader.o(b),
  policy: policy_reader.o(d),
  receipt: verifier.o(f)
)
i(c) -> f(c) -> o(g)
```

The three upstream outputs in the second example do not have to originate from
one prior input or one prior function. They are explicit sub-inputs of one
declared input representation, `i(c)`, so the function has one precise bundle
to receive and the wiring diagram has one clearly labeled input port. A join or
wait gate must make the required producers and readiness rule explicit before
that bundle is released.

Before declaring such a bundle valid, the analyzer expands every sub-input
into a producer-to-sub-input edge and checks the expanded graph. If the
function's output can reach any sub-input of its own input bundle, directly or
through other blocks, that is a **sub-input cycle**. It is a blocking static
finding: the report must name the receiving sub-input and show the shortest
feedback path. A later bounded-feedback feature may define an explicit,
terminating exception, but an ordinary input bundle never silently permits a
cycle.

Letters are compact local port names: `a` through `z`, then `aa` through `zz`,
then `aaa` through `zzz`, and so on if a bounded scope genuinely needs them.
Scopes should normally stay small enough that short names remain readable. The
letter never carries meaning by itself: its full canonical identity includes
the scope and role, such as `pathing.compare.i(c)` or
`pathing.compare.o(g)`, preventing collisions when compact blocks are wired
together or expanded.

This improvement is deliberately incremental. Recur Lang should grow only
when a real Recur, Eventness, software-development, or orchestration problem
needs the next capability.

QUICK ORIENTATION PURPOSE
-------------------------
Recur Lang is intentionally a quick, compact language for an AI, human, or
other intelligence to sketch and inspect a semantic functional model before
implementation detail buries the logical flow. Its abbreviations are a
navigation aid, not cleverness for its own sake: a reader should be able to
see the exact inputs, function, outputs, branches, joins, and evidence gates
quickly, then expand only the part that needs adjustment.

The practical loop is:

```text
compact semantic flow
  -> validate contracts, dependencies, joins, and cycles
  -> adjust the smallest affected flow or contract
  -> re-check one non-circular model
```

The language must stay concise enough for rapid iteration while remaining
precise enough that a change cannot silently introduce a circular dependency,
an ambiguous join, or a mismatched contract. It helps coordination and
understanding; it does not replace target-language implementations, tests, or
external verification evidence.

CURRENT IMPLEMENTATION CHECKPOINT
---------------------------------

Improvement 30 now has two manually completed, versioned IR boundaries:

1. `recur-lang-warp-ir-v1` freezes one exact Recur Lang 0.1 Warp, including
   canonical contracts, flow, Eventness, source hash, spans, and stable
   `RLIR001` through `RLIR011` diagnostics.
2. `recur-lang-concurrent-ir-v1` freezes the read-only Level 0/1 communication
   graph from the 0.2 Skippy coordination fixture: contracts, coordinator
   ports, lanes, policies, typed messages, fork, ordered awaits, downstream
   consumers, reachability, and `RCIR001` through `RCIR010` diagnostics.

Their accepted Eventness artifacts are:

```text
main.improvement.30.warp-ir.contract.complete
main.improvement.30.concurrent-ir.contract.complete
```

The first contract is consumed by the receipt-backed `recur-lang warp`
companion. The second is deliberately read-only. It proves that independently
useful lanes have exact communication boundaries, but it does not schedule a
worker, publish a WorkOrder, watch a channel, accept a lane receipt, or execute
a state machine.

The current bounded implementation slice is the static graph report. It should
derive dependency and wait graphs from the frozen concurrent IR, then report
cycles, unreachable lanes, missing joins, and source-hash binding. This comes
before broader queries, the grid snapshot, or a live coordinator because those
surfaces must project one validated graph rather than reimplement graph
semantics independently.

`main.improvement.30.static-graph.todo.current` is now the single active
implementation cursor. `main.improvement.30.live-grid.todo.tracking` preserves
the control-room view we are working toward and why it is valuable without
activating its implementation prematurely.

The imagined final state is still on track, with one convergence gate now made
explicit. `WIR1` and `CIR1` are sibling frozen contracts rather than one unified
coordination AST. `SGR1` must consume `CIR1` directly and must not create a
third parser. Before live coordination, a later focused contract must define
how receipt-backed Warp Eventness and concurrent lane state compose under one
source identity. Keeping that risk visible prevents the grid or coordinator
from becoming a competing source of truth.

### Rediscovered Eventness hierarchy

This is the current docs-side hierarchy, not a projection of runtime state:

```text
main.improvement.30
├── warp-ir.contract.complete
│   └── recur.lang.warp.ir.v1
├── concurrent-ir.contract.complete
│   └── recur.lang.concurrent.ir.v1
├── static-graph.todo.current
│   └── recur.lang.static.graph.report.v1         active contract freeze
├── live-grid.todo.tracking
│   └── recur.lang.grid.report.v0                 planned contract
├── contract.watch-coordination-v0.todo.future-plan
│   └── main.improvement.30.contract.watch-coordination-v0
└── todo.future-plan
    └── recur.lang.control-plane                   umbrella direction
```

The implementation dependency hierarchy is:

```text
WIR1 complete
  -> CIR1 complete
    -> SGR1 current
      -> LANGQ pure queries
        -> GRID0 pure snapshot
          -> COORD live receipt-backed coordination
            -> DOGFOOD one real Recur Rust change
```

Static knowledge flows left to right. A later actor or view may consume an
earlier contract, but it must not silently broaden or redefine that contract.

### Symbol and identity rules

Short symbols make diagrams readable; canonical identities remain durable:

| Short symbol | Canonical semantic identity | Durable schema or Eventness identity | State |
|---|---|---|---|
| `I30` | `main.improvement.30` | `README.CORE.IMPROVEMENT30.md` | active umbrella |
| `WIR1` | `recur.lang.warp.ir.v1` | `recur-lang-warp-ir-v1` / `main.improvement.30.warp-ir.contract.complete` | complete |
| `CIR1` | `recur.lang.concurrent.ir.v1` | `recur-lang-concurrent-ir-v1` / `main.improvement.30.concurrent-ir.contract.complete` | complete |
| `SGR1` | `recur.lang.static.graph.report.v1` | `main.improvement.30.static-graph.todo.current` | current; schema not frozen |
| `LANGQ` | `recur.lang.query.surface` | `recur lang ...` | planned pure projection |
| `GRID0` | `recur.lang.grid.report.v0` | `main.improvement.30.live-grid.todo.tracking` | tracked product destination |
| `COORD` | `recur.coordinator.llm` | coordinator LLM plus `recur-watch` | planned external actor |
| `DOGFOOD` | `main.improvement.30.dogfooding` | one bounded Recur Rust coordination run | planned validation |

The short symbols above are local notation, not serialized identifiers. Future
rows are provisional until their focused slice freezes a schema. New artifacts
follow these naming layers:

```text
Eventness file identity   main.improvement.30.<slice>.<state>
semantic trace identity   recur.lang.<capability>[.<version>]
wire schema identity      recur-lang-<capability>-v<N>
pure command              recur lang <query>
stateful companion        recur-lang <action>
```

An Eventness filename answers where a slice is in its lifecycle. A semantic
trace identity answers what concept crosses files. A wire schema answers which
serialized contract a consumer received. They may point to the same work, but
they are not interchangeable names.

PRODUCT BOUNDARY
----------------
Recur Lang is a coordination control plane. It is not a universal compiler,
linker, build system, CI service, or replacement for target-language tools.

```text
recur lang   = pure query, diagram, static check, report, and explanation
recur-lang   = confirmed stateful executor that performs declared language work
               and records ACK/NAK evidence
recur-watch  = filesystem subscription and watcher-state inspection
coordinator LLM = external async decision-maker that interprets watch events,
                  validates next-step eligibility, and authorizes work
workers      = humans or intelligences that create C#, Rust, HTML, CSS,
               JavaScript, TypeScript, Angular, React, MVC, CSHTML, or
               other project artifacts
toolchains   = dotnet, npm, cargo, linters, test runners, browsers, CI, etc.
```

The standalone Recur executables do not need access to every compiler or
linker. External workers and toolchains perform implementation, compilation,
testing, linting, packaging, and benchmarking. They return small structured
receipts that `recur lang` can inspect. When asynchronous coordination is
needed, a coordinator LLM consumes those facts through `recur-watch`, decides
the next declared action, and asks `recur-lang` to execute that confirmed
action.

Earlier Julia examples that interpret algorithm bodies, and exploratory ideas
about emitting Rust, remain useful language-design experiments. They do not
oblige the production `recur-lang` executable to bundle an interpreter,
compiler backend, Rust toolchain, or linker. Optional code generation can be
performed by a declared external worker and returned as another artifact.

The boundary follows the repository-wide companion rule:

```text
recur <topic>        = pure query / inspection / explanation
recur-<topic>        = opinionated coordinator / writer / async actor
```

WHY THIS IS USEFUL
------------------
The underlying primitives are familiar: typed inputs and outputs, dependency
graphs, parallel lanes, joins, state transitions, and reports. Improvement 30
does not claim those primitives are new.

The useful Recur-specific combination is:

- compact symbolic blocks with progressive disclosure;
- exact canonical input/output identities at lane boundaries;
- one queryable model for source, diagrams, reports, Eventness, and receipts;
- bounded context for humans and intelligences;
- explicit allowed write scopes and expected artifacts;
- static cycle, deadlock, reachability, and join analysis;
- generated evidence reports that other lanes may consume;
- language-independent coordination across a mixed software repository;
- integration with Recur hierarchy, trace-id, Eventness, Warp, and reveal.

The single-operator case is first-class. One person or one intelligence can use
the same compact model to answer: what is this block for, what exact input does
it consume, what does it produce, what references it, what evidence supports it,
and what Eventness state is it in? No lane fork, watcher, coordinator, or second
worker is required for that explanation.

This reduces structural hallucination and coordination drift. It does not make
arbitrary implementation claims true. An intelligence can still produce
incorrect C# or Rust that fits a valid output shape, so independent tests and
verification lanes remain necessary.

### Requirements, specialization, and coordination cost

Recur Lang should be easy for a human or intelligence to propose from a clear
requirement, but exact enough for the resulting proposal to be checked before
workers act. For example:

```text
"annotate the map, compare current and researched routes, score the result"
  -> MapDraft -> annotate -> MapContract
  -> MapContract -> [route_now, progression]
  -> [RoutePlan, ProgressionPlan] -> compare -> score -> ScoreReceipt
```

The proposal is not authority. The canonical model must still establish exact
contracts, producers, consumers, fork membership, await gates, write scopes,
and evidence requirements. `SGR1` and later pure queries then expose missing
producers, invalid joins, unreachable specialists, and accidental cycles before
coordination begins.

This makes specialization explicit and reviewable:

```text
annotator        -> RegionLabelMap
path planner     -> RoutePlan
technology model -> ProgressionPlan
runner simulator -> VerifiedRun
scorer           -> ScoreReceipt
```

The same boundary can represent a person, an intelligence, a target-language
worker, or a deterministic algorithm. A specialist receives the smallest useful
projected context plus stable references it may expand; it returns one declared
output and evidence rather than repeating broad prose coordination.

Coordination must remain proportional to the task. A small one-file repair may
cost less with one worker and ordinary tests. Lanes, receipts, and durable
Eventness earn their overhead when work is parallel, long-running, high-context,
cross-language, review-heavy, or likely to require handoff and integration.

TRACE-ID AND RECUR LANG
-----------------------
The three related surfaces have deliberately separate responsibilities:

| Surface | Primary question |
|---|---|
| `recur trace-id` | Where does this identifier appear, and which relationships were declared near it? |
| `recur lang` | Is this formal coordination program valid, and what does it mean? |
| `recur-lang` | Execute one confirmed declared action and record its outcome. |
| coordinator LLM + `recur-watch` | Which declared action is eligible after a durable event? |

`recur trace-id` is an open-world, language-independent scanner. It can follow
one stable identifier through specifications, Rust, C#, Julia, web files,
tests, receipts, and Eventness. It classifies textual relationships but does
not know whether two `i(...)` and `o(...)` contracts are identical, whether a
join is satisfiable, or whether a wait graph can deadlock. Absence from a scan
is not automatically an error.

Recur Lang is a closed-world formal model. `recur lang` parses and explains
the contracts, blocks, lanes, joins, waits, feedback limits, write scopes, and
required evidence declared by a coordination program. Missing or incompatible
parts inside that boundary can invalidate the program. `recur-lang` is the
stateful actor that advances a valid program and records ACK/NAK state. Its
first shipped action is one bounded, receipt-backed Warp transition; broader
lane coordination remains incremental. The actor does not broaden the static
model implicitly.

The integration point is durable identity. Recur Lang contracts, blocks,
lanes, work orders, and receipts publish stable trace IDs. `trace-id` then
discovers where those identities travel in artifacts outside the formal
parser. For example, `recur lang show game.path-monkey.f` explains the block
and its exact boundaries, while `recur trace-id "game.pathing.route"` finds
its specification, implementation, fixtures, tests, reviews, and receipts.

CORE MODEL
----------

### Contracts and symbols

Input and output roles remain explicit in the header:

```text
scope gcd {
  i(a) := (left: Int, right: Int)
  o(b) := (value: Int)
  f : i(a) -> o(b) ~ "Euclid greatest common divisor"
}
```

A shared boundary uses one canonical contract:

```text
scope verify {
  i(b) := gcd.o(b)
  o(c) := (accepted: Bool, evidence: List<ReceiptRef>)
  f : i(b) -> o(c) ~ "Verify the result"
}
```

`gcd.o(b)` and `verify.i(b)` are not merely similar records. They resolve to
the same canonical contract. Intentional conversions use an explicit adapter
block instead of an implicit structural cast.

### Lanes

A lane is a bounded unit of coordinated work. A future lane contract may
declare:

```text
lane web.ui {
  i(a) := (
    specification: SpecRef,
    api_contract: ContractRef
  )

  target angular
  allow read  ["ClientApp/src/**", "docs/**"]
  allow write ["ClientApp/src/**"]

  require {
    receipt npm.test
    receipt npm.lint
  }

  o(b) := (
    patch: PatchRef,
    files_changed: List<Path>,
    evidence: List<ReceiptRef>,
    unresolved: List<Issue>
  )
}
```

The syntax above is a design target, not a claim that the current Julia spike
already parses it.

The declared write scope is useful in two ways:

1. an external sandbox or agent host may enforce it as a capability;
2. Recur Lang may compare a returned file list or patch receipt with it and
   reject an out-of-scope result.

Recur itself must not claim enforcement when it only performed after-the-fact
validation.

### Worker ownership and coordinator routing

A worker owns one declared WorkOrder, not the surrounding coordination problem.
Its role is to read the projected input, perform the bounded work, publish its
declared output and evidence, and surface unresolved questions. It MUST NOT
silently expand its assignment, self-approve its result, invent downstream
dependencies, or redefine its completion policy.

The coordinator LLM owns assignment and verification routing. After a durable
worker receipt arrives, it releases the declared next step: an independent test
or review lane when separation matters, or a later bounded verification command
for the same worker when the declared policy permits it. In both cases,
verification is a distinct Eventness fact and must cite its tests or other
evidence.

`recur-watch` is an asynchronous wake-up mechanism, not an intelligence and not
proof of completion. A watcher event causes the coordinator LLM to reread the
immutable WorkOrder and durable receipt, validate the declared state, and decide
whether a next command is eligible. The worker's job remains the declared lane
slice; the coordinator decides what follows. `recur lang` performs none of this
waiting, scheduling, or routing.

### Fan-out, joins, and composition

One input may feed one function or several independent lanes:

```text
i(change) -> [runtime, tests, docs]
```

The integration block makes the join visible:

```text
integration await [
  runtime.o(b),
  tests.o(b),
  docs.o(b)
] -> f(a) -> o(candidate)
```

A contracted group may itself become a reusable symbol, but expanding it must
show the exact lanes, contracts, waits, source locations, and evidence rules.

### Semantic flow and merge laws

The compact notation needs explicit laws wherever independent results are
combined. These are future language and runtime contracts, not behavior that
the current bounded `CIR1` parser already promises. The pathing fixture is the
red-first proving ground, especially its `assemble` block that merges paths
and deduplicates pellets.

1. **Type closure.** Every edge and join member resolves to one exact declared
   contract. A result of one composition may feed the next block only through
   a compatible named port or explicit adapter.
2. **Causality and non-circular flow.** A consumer cannot be released before
   its declared producer and wait gate are satisfied. Dependency cycles and
   wait cycles are static findings, not conditions to discover by hanging a
   coordinator.
3. **Associativity, when declared.** A normalized collection merge may define
   `merge(merge(A, B), C) == merge(A, merge(B, C))`. The language must not
   assume this for arbitrary functions or joins; the relevant operator must
   explicitly declare or inherit the law.
4. **Commutativity, when declared.** Independent asynchronous completions may
   be readiness-order independent, while authored port order remains preserved
   for explanation. A merge is commutative only when its normalized semantic
   result is the same for `merge(A, B)` and `merge(B, A)`.
5. **Idempotence and retries.** Repeating the same qualified result must not
   change an idempotent aggregate: `merge(A, A) == merge(A)`. A duplicate key
   with a different value is a stable conflict/NAK, never an implicit choice.
6. **Identity and empty work.** Each collection operator must define its empty
   identity and the meaning of a zero-item scatter or join. The implementation
   may not improvise this behavior at runtime.
7. **Determinism and cumulative growth.** Equal source, normalized input, and
   declared seed produce byte-equivalent normalized results. Adding one valid,
   non-conflicting qualified branch may extend a cumulative aggregate; it may
   not erase or reorder unrelated facts.
8. **Scatter/gather cardinality.** A dynamic scatter over `N` qualified work
   orders must produce exactly the declared qualified outputs, and its gather
   must account for each required result exactly once.

Initial red-first tests should therefore prove associative grouping,
completion-order-independent normalized output, idempotent retries, identity
behavior for empty work, deterministic output, conflicting-key rejection, and
exact scatter/gather cardinality. These laws belong in the canonical model and
its tests before a coordinator relies on them.

### IR-backed mocks and smaller tests

The same exact input/output contract can bind to a real worker or a
deterministic mock. A test changes the binding, not the surrounding graph:

```text
i(request) -> real.f(request) -> o(receipt)
i(request) -> mock.f(request) -> o(receipt)
```

Both bindings must satisfy the same declared input and output identities. This
lets a test isolate one lane, supply known mock outputs for its dependencies,
and still validate the real join, consumer, Eventness, and evidence shape
around it. The IR should reject a mock that omits a required field, returns an
incompatible contract, or leaves a required join unsatisfied.

This is deliberately smaller than starting the whole system: parse one model,
select declared mock bindings, run or inspect the bounded flow, and compare the
resulting normalized outputs and receipts. A mock proves only the tested
coordination and contract behavior; target-language correctness still requires
the appropriate external tests and evidence.

### Validation, refinements, and handled exceptions

`validate` is a first-class functional block when raw input must become data
that later blocks may trust. It makes the refinement visible instead of hiding
it inside a method body:

```text
i(a) := RawFormInput
i(a) -> validate.f(a) -> o(b) := Result<ValidatedFormInput, ValidationErrors>

ValidatedFormInput -> refine.f -> FinalFormModel
ValidationErrors   -> form_feedback.f -> FormFeedback
```

The success and failure outcomes are separate declared ports or variants. A
successful validation may feed `refine`; validation errors feed a named handler
that produces user-visible feedback. Likewise, a persistence operation may
return `Result<SavedForm, PersistenceError>` and route the error to a declared
retry, operator-review, or failure-report block.

The IR must make these exceptional paths visible and verify that each declared
failure outcome has a compatible handler. It must not treat an exception as an
implicit side exit. Retry edges are ordinary graph edges and therefore remain
subject to the same non-circular rules unless a later bounded-feedback contract
explicitly declares a terminating retry policy.

### Bounded retries and asynchronous awaits

An `await` names the exact outcomes that must exist before a consumer is
released; it is not an implicit pause or a guess about what might arrive:

```text
await all [validation.success, policy.success] -> persist
```

Every awaited outcome needs one declared producer and one compatible consumer.
An asynchronous completion order may vary, but the normalized result must not.

A retry is the exceptional back-edge case. It must declare a maximum attempt
count, retain the input snapshot or qualified work key it is retrying, and
route exhaustion to a terminal handler:

```text
persist.failure -> retry
retry.retry -> persist     bounded max_attempts 3
retry.exhausted -> failure_report
```

The IR must reject an unbounded retry, an await with a missing producer, a
retry with no exhaustion outcome, and any retry/await cycle that can deadlock.
Red-first fixtures should prove those rejected cases as well as a valid bounded
retry whose completion order preserves the same normalized result.

### Subsystem contraction and parent integration

A system or subsystem may be designed, coordinated, and accepted within its
own bounded model before a parent system integrates it. Its public input and
output form the reusable boundary:

```text
subsystem game.pathing version 1 {
  i(request) := (
    map: MapRef,
    origin: Tile,
    destination: Tile,
    actor: ActorRef
  )

  o(route) := (
    path: List<Tile>,
    cost: Int,
    reachable: Bool,
    evidence: ReceiptRef
  )

  public pathing : i(request) -> f(pathing) -> o(route)
}
```

Internally, `f(pathing)` may expand into map validation, route calculation,
movement-rule checks, joins, and verification lanes. A parent may contract all
of that accepted detail into one block:

```text
system game.gameplay {
  use game.pathing@1

  gameplay : i(frame)
    -> f(game.pathing@1)
    -> f(enemy.movement)
    -> f(sprite.animation)
    -> o(frame-result)
}
```

The `system`, `subsystem`, `public`, and `use` forms are design targets. The
current Julia 0.1 parser does not yet accept them.

Composition follows these rules:

1. The child owns and validates its internal graph.
2. The child publishes a versioned or content-hashed public contract plus its
   required acceptance receipt.
3. The parent imports that exact boundary rather than copying a similar shape.
4. The compact parent view shows one subsystem block; expansion drills into the
   child's exact source, lanes, evidence, and Eventness.
5. Compatible internal changes may preserve the public boundary. A breaking
   boundary change creates a new identity and cannot inherit prior acceptance.
6. Child implementation completion, child verification, integration readiness,
   parent acceptance, and parent completion are separate facts.

This prevents a large solution from becoming one enormous graph. Each
intelligence can receive the smallest useful subsystem or lane projection,
while the parent retains exact identities for impact analysis and integration.

### Eventness and Warp

Lane progress should be externally visible:

```text
todo.current
  -> assigned
  -> produced
  -> contract-checked
  -> integration-tested
  -> accepted
  -> merge-ready
```

Subsystem composition adds a scoped handoff:

```text
child.implementation.complete
  -> child.verification.accepted
  -> child.integration.ready
  -> parent.child.integration.accepted
```

The final child state does not imply that the parent is complete. Readiness and
parent acceptance records must carry the same public contract version or
content hash. Eventness therefore reports local truth without allowing status
to leak upward through the system hierarchy.

Warp continues to express the current state, a bounded useful slice, and the
desired state:

```text
E0(change.todo.current) -> dE(runtime.f) -> Ef(change.merge-ready)
```

The transition is evidence, not wishful prose. ACK/NAK records should preserve
the lane, attempt, source hash, artifact reference, test receipt, and reason.

CANONICAL DOCUMENT STRUCTURE: HEADER, BODY, AND FOOTER
------------------------------------------------------
To balance pure human intuition, formal rigor, and state convergence, Recur Lang
standardizes on a tripartite document anatomy:

```text
+-----------------------------------------------------------------------------------+
|  HEADER: Functional Contracts & Flow                                              |
|  f(a) -> f(b) with explicit input / output typing, bundle signatures, and ports   |
+-----------------------------------------------------------------------------------+
|  BODY: Hierarchical State Tree                                                    |
|  Multi-line nesting with arrows cutting across depth levels; instant cycle check   |
+-----------------------------------------------------------------------------------+
|  FOOTER: Refinement & Convergence State                                           |
|  Current eventness residue, bounded Slices, verification receipts, and next gate  |
+-----------------------------------------------------------------------------------+
```

### 1. Header — Functional Contracts & Flow
- Pure directional flow declared explicitly: `f(a) -> f(b)`.
- Input and output types, aliases, and bundle signatures are declared clearly in
  the header so readers and parsers never have to guess data contracts.
- Directionality remains front and center: `i(a) -> f(a) -> o(b)`.

### 2. Body — Multi-Line Hierarchical State Tree
- Visual, structural hierarchy rendered using familiar Recur namespace/depth semantics.
- Directed arrows (`-->`) cut across or plunge into varying levels of depth to
  explicitly chart dataflow, fan-outs, and handoffs.
- Enables instant topological analysis:
  - **Circular reference detection**: immediate identification of illegal backward loops.
  - **Logic gaps & starved nodes**: detection of unsatisfied inputs, unreachable branches,
    and orphan outputs.
  - **Boundary encapsulation**: catches illegal arrows penetrating private domain membranes.

### 3. Footer — Refinement & Convergence State
- Anchors the theoretical flow to real-world Eventness and Warp progress.
- Tracks residual pressure, bounded Slices (`slice_id`, `contract_hash`, `attempt_id`),
  and acceptance receipts.
- Declares static analysis requests (`check circular_ref`, `check lane_scope`) and
  specifies the next deterministic gate to advance toward optimum completion.

STATIC ORCHESTRATION CHECKS
---------------------------
`recur lang check` should eventually validate the graph without executing
target-language work.

Initial checks should include:

- unknown or duplicate symbols;
- input/output role mistakes;
- non-identical shared contracts;
- missing producers or consumers;
- invalid fan-out and fan-in;
- async lanes without a visible join or await;
- unreachable lanes and outputs;
- imported subsystem boundaries with unknown, missing, or stale versions or
  content hashes;
- parent acceptance that references a different child boundary;
- accidental dependency cycles;
- wait/deadlock cycles;
- Eventness transitions with no declared evidence;
- conflicting or overlapping lane write scopes;
- required receipts missing from an integration gate;
- stale reports whose source hash no longer matches.

Cycle analysis should distinguish:

- contract-alias cycles;
- dataflow cycles;
- lane wait cycles;
- Warp cycles;
- ordinary function recursion;
- explicit bounded feedback.

An intentional loop must be declared and bounded. It must not be inferred from
an accidental graph cycle.

SOURCE REQUESTS AND GENERATED REPORTS
-------------------------------------
The source footer requests checks:

```text
footer {
  check circular_ref
  check lane_scope
  report coordination
}
```

The result is generated rather than manually asserted:

```text
report {
  source_hash: "..."
  orchestration_sound: true
  circular_ref: false
  circular_refs: []
  unreachable_lanes: []
  conflicting_write_scopes: []
  required_evidence_complete: true
  implementation_correctness: externally-verified
}
```

When a cycle exists:

```text
report {
  orchestration_sound: false
  circular_ref: true
  circular_refs: [
    {
      kind: dataflow
      path: a.o(b) -> b.i(b) -> b.o(c) -> a.i(a)
      intentional: false
    }
  ]
}
```

The report may be rendered as text, JSON, or a formal diagram from the same
canonical AST. The diagram is not separately maintained documentation.

LIVING MASTER WORK REPORT
-------------------------
The coordination graph should also project into a block-level grid that changes
as Eventness arrives:

```text
Solution: customer-search                         Attempt 1/3

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

This is the living master work report for one solution. At the block level it
shows:

- which lanes are watching, ready, working, blocked, produced, or verified;
- which compact `f(a)` block each intelligence currently owns;
- which joins are still missing exact receipts;
- attempts, questions, NAKs, stale progress, and dependency blockers;
- evidence already produced and whether integration is justified.

The grid is a projection, not another source of truth. Its cells must be
reconstructed from the canonical AST, watcher state, immutable WorkOrders,
receipts, and Eventness. Restarting the renderer or coordinator must rebuild
the same grid from disk.

Progressive drill-down should follow:

```text
grid
  -> lane
    -> contract
      -> current WorkOrder
        -> changed files
          -> tool receipts
            -> exact Eventness timeline
```

The pure surface renders a snapshot and exits:

```text
recur lang grid <solution-or-run>
recur lang grid <solution-or-run> --json
```

The coordinator LLM may keep the changing control-room view current while
coordinating from `recur-watch` events:

```text
coordinator LLM -> recur-watch events + recur lang grid <source>
```

`recur-watch` supplies active filesystem notifications. Core `recur watch`
continues to inspect watcher state only. A future watcher art mode may show
watcher activity, but the master work report belongs to the Recur Lang
coordination model because it understands contracts, blocks, joins, attempts,
and receipts.

When work finishes, the live report becomes its durable audit record:

```text
solution.coordination.current -> solution.coordination.complete
```

Completion preserves the final grid, source hash, lane transitions, questions,
receipts, commits, verification, and merge decision. The live control room and
the final work report are two views of the same history.

SOUNDNESS BOUNDARY
------------------
Recur Lang can make precise claims about its own coordination model:

```text
orchestration_sound
contract_match
circular_ref
lane_scope_valid
required_evidence_complete
event_transition_valid
```

It cannot prove arbitrary target-language behavior without external evidence.
Reports should therefore use honest states such as:

```text
implementation_correctness = unverified-external
implementation_correctness = externally-verified
implementation_correctness = rejected
```

Passing a receipt schema does not prove that the receipt is trustworthy.
Trusted environments may add artifact hashes, command records, signatures,
CI run identifiers, or independent verification receipts.

MULTI-INTELLIGENCE CHANGE FLOW
------------------------------
A coordinator such as Skippy may divide one requested change into bounded
lanes:

```text
                         +-> runtime lane --+
specification -> assign -+-> tests lane ----+-> integration -> verify -> merge
                         +-> docs lane -----+
```

Each worker receives only the relevant projected context plus stable references
it may expand. Each result contains a patch or commit reference, changed-file
list, evidence receipts, and unresolved issues.

Parallel workers should normally use isolated worktrees or lane branches.
An integration lane combines their commits onto one task branch, checks the
declared contracts, consumes external test receipts, and produces either an
accepted candidate or a NAK reason. Improvement 30 does not replace Git and
does not authorize automatic merging by itself.

DOGFOODING RECUR'S RUST CODE
----------------------------
Improvement 30 should prove itself on Recur's own code before claiming broad
orchestration value.

A useful first dogfooding case is improving or validating a Rust algorithm:

```text
generated cases
  +-> Julia reference result --+
  +-> Rust candidate result ---+-> differential verification -> report
```

Possible lanes:

- preserve and characterize the current Rust behavior;
- propose a clearer or faster Rust implementation;
- generate boundary and adversarial cases;
- define properties independent of either implementation;
- run the existing Julia reference model;
- consume external `cargo test` and benchmark receipts;
- independently verify the candidate before integration.

The Julia implementation may act as a reference oracle or contract harness.
The Rust worker or CI environment owns `cargo`, `rustc`, tests, and benchmarks.
`recur lang` statically validates the declared model and inspects resulting
evidence; the coordinator LLM handles asynchronous routing; `recur-lang`
performs the confirmed declared action and records its outcome. It does not
need to bundle the Rust compiler: target-language tool execution remains a
separately declared and explicitly confirmed executor capability.

COMMAND DIRECTION
-----------------
Proposed pure query forms:

```text
recur lang show <symbol>
recur lang expand <symbol>
recur lang contract <scope>
recur lang lanes <scope>
recur lang diagram <scope>
recur lang grid <scope-or-run>
recur lang check <source>
recur lang report <source-or-run>
recur lang refs <symbol>
```

When needed, a coordinator LLM uses `recur-watch` wake-up events and the pure
forms above to route and authorize work. `recur-lang` then executes the exact
confirmed action and records the state transition; `recur lang` never performs
that mutation.

The first focused synchronous receipt-confirmation command is:

```text
recur-lang warp <source> <scope> --eventness <exact-E0-file> \
  --receipt <external-receipt> --confirm
```

The first focused companion command is now frozen:

```text
recur-lang warp <source> <scope>
recur-lang warp <source> <scope> \
  --eventness <exact-E0-file> \
  --receipt <external-receipt> \
  --confirm
```

Without `--confirm`, `recur-lang warp` is a pure dry run that reports the
declared `E0`, `dE`, `Ef`, source hash, and required receipt schema. A confirmed
transition is deliberately narrow: it validates one root-contained E0
artifact and one `recur-lang-warp-receipt-v1`, renames that artifact to the
declared Ef identity, and writes one durable ACK or NAK status record. It does
not yet execute the declared slice or invoke a target-language toolchain. A
future work-execution action must remain explicit, source-bound, scoped, and
confirmation-gated; it may not infer approval from a plan or mutate unrelated
artifacts.

The remaining companion commands above are design shapes. Their exact
contracts should be frozen only when a focused test slice is opened.

IMPLEMENTATION SLICES
---------------------

The slice number describes dependency order, not permission to activate every
later behavior. Each slice opens one bounded Eventness artifact, freezes its
contract, verifies it, and only then moves that artifact to a completed state.

| Slice | Symbol | Roadmap state | Reason for position |
|---|---|---|---|
| 0 | `I30` seed | complete | Preserve the proposal, fixtures, and product boundary before implementation. |
| 1a | `WIR1` | complete | Give Warp one canonical, receipt-bound model before adding more syntax. |
| 1b | `CIR1` | complete | Make lane communication exact before analyzing or scheduling it. |
| 2 | `SGR1` | current | Derive soundness facts once from `CIR1` for every later consumer. |
| 3 | `LANGQ` | planned | Expose pure views only after their model and analysis are stable. |
| 4 | `GRID0` | `todo.tracking`; implementation gated by 2–3 | Keep the visible coordination goal explicit without creating a second source of truth. |
| 5 | `COORD` | Warp increment complete; multi-lane actor planned | Mutate state only through frozen schemas, external evidence, and ACK/NAK. |
| 6 | `DOGFOOD` | planned | Validate usefulness on one real repository change after the boundaries exist. |
| 7–8 | integration and IDE | future | Scale and presentation come after semantics and evidence. |

### Slice 0: Preserve the design and seed

- Add Improvement 30 and its docs-side future-plan bridge.
- Keep the current `main.lang` Julia parser, algorithms, and tests as a spike.
- Keep the Skippy watch-coordination source and formal v0 contract as a
  design fixture for compact-to-exact progressive disclosure.
- Mark unsupported syntax and runtime behavior honestly.
- Keep unrelated slices parked while the live-grid cursor pulls one bounded
  contract and snapshot slice.

### Slice 1: Freeze the coordination IR

- Define versioned AST and JSON shapes for systems, subsystems, public
  boundaries, imports, scopes, contracts, lanes, joins, write scopes, evidence
  requirements, events, Warps, and source spans.
- Decide canonical identity and explicit adapter behavior.
- Define stable diagnostics before expanding execution behavior.

The first Goldilocks sub-slice freezes only the existing 0.1 Warp boundary as
`recur-lang-warp-ir-v1`: exact local/canonical contracts, one function and
flow, Eventness edges, `E0 -> dE -> Ef`, source hash, source spans, and stable
`RLIR001` through `RLIR011` diagnostics. The remaining coordination IR forms
above are intentionally still open. Its manually completed contract is
`docs/main.improvement.30.warp-ir.contract.complete.md`.

The second sub-slice freezes the first concurrent boundary as
`recur-lang-concurrent-ir-v1`: named message contracts, projected coordinator
ports, lane input/output messages and policies, the initial fork, ordered
typed awaits, downstream consumers, reachability, and stable `RCIR001` through
`RCIR010` diagnostics. Its manually completed contract is
`docs/main.improvement.30.concurrent-ir.contract.complete.md`. It is a
read-only communication graph, not a scheduler.

Slice 1 is therefore foundationally useful but not globally complete. Systems,
subsystems, imports, adapters, feedback, watcher topology, and the remaining
0.2 syntax must still arrive through later bounded contracts.

### Slice 2: Static graph report — current

- Build dependency and wait graphs from the canonical IR.
- Add dependency-cycle, wait-cycle, unreachable-lane, and unsatisfied-join
  findings for the CIR1 boundary.
- Generate the first source-hashed coordination report.
- Add Julia fixtures and focused tests for accepted and rejected graphs.

`SGR1` is current because `CIR1` now provides stable typed nodes, edges, fork
members, awaits, consumers, spans, and source hashes. The graph report should
remain a deterministic read-only projection. It must not schedule lanes or
advance Eventness.

Stale subsystem imports remain a later graph-report extension because `CIR1`
does not yet model systems, imports, or versioned public boundaries.

`demos/pathing/main.lang.pathing.recur` is a red-first language-conformance
fixture for the broader destination: one-input broadcast, distinct parallel
functions, dynamic per-power-node scatter, qualified outputs, local graph
views, and deterministic joins. Its source-shape tests are active while its
parser, graph, and execution contracts remain `@test_broken`. Those future
contracts guide later slices without broadening the current CIR1-only `SGR1`.

The fixture also now contains an executable Julia prototype with manifest-backed
ASCII topology, terrain, current-route, and optimum-route layers aligned to a
portable bitmap. It records capability negotiation at each selected terrain
stage and validates glyph definitions, cell-to-image alignment, and deterministic
route costs. This is downstream evidence for future contract and query design;
it does not expand SGR1 beyond `ConcurrentIr` or make the `0.3` parser/runtime
shipped behavior.

### Slice 3: Pure `recur lang` queries

- Add read-only show, expand, refs, lanes, diagram, grid, check, and report
  surfaces.
- Preserve core Recur purity.
- Make text and JSON projections agree.
- Make `report` the fast orientation projection: explain header contracts and
  descriptions, body bindings and flow, footer checks and Eventness, plus the
  exact upstream/downstream references for a source or selected symbol.
- Preserve one source-hashed IR interpretation so a human or intelligence can
  resolve what is what without manually reconciling header, body, and footer.

### Slice 4: Living master work report

- Freeze a versioned grid/report shape with run, phase, lane, block, mode,
  dependency, evidence, attempt, watcher, and freshness fields.
- Render `recur lang grid` as a pure snapshot from the canonical model.
- Rebuild the same snapshot after process restart using only durable state.
- Let a coordinator LLM refresh a live grid only after snapshot and JSON
  behavior are stable; the rendered data remains derived from durable state.
- Support drill-down from grid cell to lane, WorkOrder, receipt, and timeline.
- Collapse `coordination.current` into a durable `coordination.complete` report.

`GRID0` is the tracked product destination. Its first implementation pull
remains a pure, deterministic snapshot after it is promoted back to
`todo.current`. Live refresh waits until `SGR1` and the shared pure query
projection exist, so the display cannot become an independent state store or
soundness engine.

### Slice 5: Receipt confirmation and Eventness

- Add the smallest useful synchronous receipt-confirmation surface.
- Accept versioned external receipts through files or standard input.
- Validate required evidence and actual changed files against lane declarations.
- Write ACK/NAK Eventness that core Recur can inspect.
- Do not invoke arbitrary target-language toolchains.

The first part of this slice is implemented as
`recur-lang-warp-plan-v1`, `recur-lang-warp-receipt-v1`, and
`recur-lang-warp-status-v1`. File receipts are accepted; standard input,
lane write-scope validation, multi-lane coordination, and arbitrary 0.2
coordination syntax remain future slices.

### Slice 6: Recur Rust dogfooding

- Choose one real Rust algorithm or validation problem.
- Use isolated implementation, test, and verification lanes.
- Use Julia where useful as an independent reference.
- Consume external Cargo receipts.
- Measure whether the formalism catches drift or reduces integration effort.

### Slice 7: Multi-worktree integration

- Represent lane branches, commits, patches, conflicts, and integration order.
- Detect overlapping declared write scopes before parallel work starts.
- Keep commit, merge, approval, and destructive actions behind explicit policy.

### Slice 8: IDE and formal diagrams

- Add hover, go-to-definition, expansion, and contraction from source spans.
- Render block, lane, wait, and Eventness diagrams from the canonical AST.
- Surface the smallest useful cycle or contract path in diagnostics.

NON-GOALS
---------
Improvement 30 does not initially:

- compile C#, Rust, JavaScript, TypeScript, Angular, React, MVC, or CSHTML;
- bundle target compilers, package managers, browsers, or linkers;
- replace Git, worktrees, CI, build systems, or test frameworks;
- prove arbitrary source-code correctness;
- trust a worker merely because it returned a well-shaped receipt;
- automatically merge, approve, deploy, or publish work;
- create an unbounded autonomous-agent loop;
- require every project to use Julia or Rust;
- make expansion bodies a second source of truth.

SUCCESS CRITERIA
----------------

- A compact lane graph expands into exact contracts, dependencies, and evidence.
- An accepted subsystem contracts into one versioned block that a parent may
  import without losing drill-down, lineage, or acceptance evidence.
- Child completion and parent integration remain distinct Eventness facts.
- Accidental cycles and invalid joins are reported before integration.
- Workers in different implementation languages can use the same lane model.
- External test and build results can be represented without Recur owning those
  toolchains.
- A returned patch can be checked against its declared write scope.
- Eventness shows assigned, produced, verified, accepted, and rejected states.
- A master grid reconstructs every lane's current block and evidence from
  durable state, both before and after coordinator restart.
- The live grid and completed work report derive from the same history.
- An independent verification lane can NAK an apparently complete result.
- One real Recur Rust change is coordinated and validated through the model.
- The model saves navigation or integration effort compared with ordinary
  prose-only coordination.

TRACE-ID LINES
--------------

```text
defines: main.improvement.30 recur lang coordination contracts for bounded multi-intelligence lanes
defines: recur.lang.control-plane language-independent static orchestration and progressive disclosure model
defines: recur.coordinator.llm external async actor using recur-watch wake-ups and recur lang projections
defines: recur.lang.soundness-boundary orchestration soundness is internal while implementation correctness requires external evidence
defines: recur.lang.master.work.report living block grid and completed audit report projected from canonical coordination Eventness
defines: recur.lang.trace.boundary trace-id discovers open-world repository lineage while Recur Lang validates closed-world coordination semantics
defines: recur.lang.subsystem.composition accepted child models contract into versioned blocks with separate parent integration Eventness
defines: recur.lang.warp.ir.v1 versioned canonical Warp contract with source hash spans Eventness and stable diagnostics
defines: recur.lang.concurrent.ir.v1 read-only typed lane message fork await and reachability contract
defines: recur.lang.static.graph.report.v1 provisional next read-only dependency wait and soundness projection
defines: recur.lang.requirements.translation bounded requirement-to-contract-and-graph proposal discipline
defines: recur.lang.specialization.boundary projected context declared output and evidence for one specialist
defines: recur.lang.coordination.proportionality coordination overhead must remain smaller than the work it coordinates
consumes: main.lang compact input function output contracts and canonical bundle aliases
consumes: main.recur.purity.decision core recur pure query and companion actor split
consumes: workflow.pattern.docs.tests.rust.verify.complete recurring docs tests Rust verification loop
consumes: main.improvement.30.warp-ir.contract.complete accepted WIR1 foundation
consumes: main.improvement.30.concurrent-ir.contract.complete accepted CIR1 foundation
produces: recur.lang.coordination-report source-hashed cycles joins scopes evidence and Eventness status
produces: recur.lang.formal-diagram canonical AST projection for humans and intelligences
produces: main.improvement.30.static-graph active SGR1 contract-freeze cursor
produces: main.improvement.30.live-grid tracked destination for the living master work report
triggers: main.improvement.30.contract future versioned coordination IR and JSON schema
triggers: main.improvement.30.static-graph.todo.current cycle reachability join and wait report over CIR1
triggers: main.improvement.30.dogfooding future Recur Rust algorithm validation lane
```

DISCOVERY
---------

```powershell
recur files "README.CORE.IMPROVEMENT30" -d ./
recur tree "main.improvement.30" -d docs/
recur files "main.lang.**" -d . --sep . --sep _
recur files "main.command.lang.**" -d docs/
recur trace-id "main.improvement.30" --scope "**" --ext .md -d .
```

RELATED
-------

- `recur_language_start.md`
- `docs/main.lang.readme.md`
- `docs/main.command.lang.readme.md`
- `docs/main.command.trace-id.readme.md`
- `docs/main.improvement.30.warp-ir.contract.complete.md`
- `docs/main.improvement.30.concurrent-ir.contract.complete.md`
- `docs/main.improvement.30.static-graph.todo.current.md`
- `docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`
- `docs/main.improvement.30.live-grid.todo.tracking.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `demos/main.lang/main.lang.algorithm-lab.recur`
- `demos/main.lang/main.lang.skippy-watch-coordination.recur`
- `demos/pathing/main.lang.pathing.recur`
- `julia-tests/main.lang.pathing.test.jl`
- `demos/main.lang/main.lang.runtime.jl`
- `julia-tests/main.lang.test.jl`
