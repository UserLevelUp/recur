RECUR IMPROVEMENT 30
Recur Lang Coordination Contracts
=================================
Date: July 24, 2026
Status: Proposal / active incremental direction
Author: Captured from Recur Lang and multi-intelligence orchestration design

INTENT
------
Add a small, language-independent coordination formalism to Recur so humans
and multiple intelligences can divide software work into bounded lanes, join
their results, and inspect whether the coordination logic is sound.

The compact Recur Lang form is:

```text
i(a) -> f(a) -> o(b)
```

where:

- `i(a)` is the exact input contract for the block;
- `f(a)` is a compact function or work-slice symbol;
- `o(b)` is the exact output contract;
- a downstream `i(b)` may alias the same canonical contract as `o(b)`;
- expansion and contraction are lossless views over one parsed model.

This improvement is deliberately incremental. Recur Lang should grow only
when a real Recur, Eventness, software-development, or orchestration problem
needs the next capability.

PRODUCT BOUNDARY
----------------
Recur Lang is a coordination control plane. It is not a universal compiler,
linker, build system, CI service, or replacement for target-language tools.

```text
recur lang   = pure query, diagram, static check, report, and explanation
recur-lang   = coordination actor, lane state, receipt validation, ACK/NAK
workers      = humans or intelligences that create C#, Rust, HTML, CSS,
               JavaScript, TypeScript, Angular, React, MVC, CSHTML, or
               other project artifacts
toolchains   = dotnet, npm, cargo, linters, test runners, browsers, CI, etc.
```

The standalone Recur executables do not need access to every compiler or
linker. External workers and toolchains perform implementation, compilation,
testing, linting, packaging, and benchmarking. They return small structured
receipts that Recur can inspect and coordinate.

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

This reduces structural hallucination and coordination drift. It does not make
arbitrary implementation claims true. An intelligence can still produce
incorrect C# or Rust that fits a valid output shape, so independent tests and
verification lanes remain necessary.

TRACE-ID AND RECUR LANG
-----------------------
The three related surfaces have deliberately separate responsibilities:

| Surface | Primary question |
|---|---|
| `recur trace-id` | Where does this identifier appear, and which relationships were declared near it? |
| `recur lang` | Is this formal coordination program valid, and what does it mean? |
| `recur-lang` | How should a valid coordination program advance through its lanes? |

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

The companion may render the changing control-room view while coordinating:

```text
recur-lang coordinate <source> --view grid
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
Recur Lang coordinates and validates the resulting evidence; it does not need
the Rust compiler inside its executable.

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

Proposed companion forms:

```text
recur-lang coordinate <source>
recur-lang coordinate <source> --view grid
recur-lang assign <lane>
recur-lang accept-receipt <receipt>
recur-lang status <run>
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
not execute the declared slice, invoke a target-language toolchain, infer
approval from a plan, or mutate any other artifact.

The remaining companion commands above are design shapes. Their exact
contracts should be frozen only when a focused test slice is opened.

IMPLEMENTATION SLICES
---------------------

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
above are intentionally still open.

The second sub-slice freezes the first concurrent boundary as
`recur-lang-concurrent-ir-v1`: named message contracts, projected coordinator
ports, lane input/output messages and policies, the initial fork, ordered
typed awaits, downstream consumers, reachability, and stable `RCIR001` through
`RCIR010` diagnostics. It is a read-only communication graph, not a scheduler.

### Slice 2: Static graph report

- Build dependency and wait graphs from the canonical IR.
- Add cycle, unreachable-lane, missing-join, contract, and stale subsystem
  import checks.
- Generate the first source-hashed coordination report.
- Add Julia fixtures and focused tests for accepted and rejected graphs.

### Slice 3: Pure `recur lang` queries

- Add read-only show, expand, refs, lanes, diagram, grid, check, and report
  surfaces.
- Preserve core Recur purity.
- Make text and JSON projections agree.

### Slice 4: Living master work report

- Freeze a versioned grid/report shape with run, phase, lane, block, mode,
  dependency, evidence, attempt, watcher, and freshness fields.
- Render `recur lang grid` as a pure snapshot from the canonical model.
- Rebuild the same snapshot after process restart using only durable state.
- Add a live `recur-lang coordinate --view grid` projection after snapshot and
  JSON behavior are stable.
- Support drill-down from grid cell to lane, WorkOrder, receipt, and timeline.
- Collapse `coordination.current` into a durable `coordination.complete` report.

### Slice 5: Companion receipts and Eventness

- Add the smallest useful `recur-lang` coordinator surface.
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
defines: recur-lang.coordinator companion lane state receipt validation and ACK/NAK actor
defines: recur.lang.soundness-boundary orchestration soundness is internal while implementation correctness requires external evidence
defines: recur.lang.master.work.report living block grid and completed audit report projected from canonical coordination Eventness
defines: recur.lang.trace.boundary trace-id discovers open-world repository lineage while Recur Lang validates closed-world coordination semantics
defines: recur.lang.subsystem.composition accepted child models contract into versioned blocks with separate parent integration Eventness
consumes: main.lang compact input function output contracts and canonical bundle aliases
consumes: main.recur.purity.decision core recur pure query and companion actor split
consumes: workflow.pattern.docs.tests.rust.verify.complete recurring docs tests Rust verification loop
produces: recur.lang.coordination-report source-hashed cycles joins scopes evidence and Eventness status
produces: recur.lang.formal-diagram canonical AST projection for humans and intelligences
produces: main.improvement.30.live-grid active focused cursor for the living master work report
triggers: main.improvement.30.contract future versioned coordination IR and JSON schema
triggers: main.improvement.30.static-analysis future cycle reachability join and lane-scope report
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
- `docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`
- `docs/main.improvement.30.live-grid.todo.current.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.improvement.delivery-loop.recurring.md`
- `demos/main.lang/main.lang.algorithm-lab.recur`
- `demos/main.lang/main.lang.skippy-watch-coordination.recur`
- `demos/main.lang/main.lang.runtime.jl`
- `julia-tests/main.lang.test.jl`
