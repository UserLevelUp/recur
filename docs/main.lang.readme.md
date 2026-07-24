# main.lang.readme

Status: `brainstorming prototype`

`main.lang` is the canonical hierarchy for the experimental language that
describes Recur-native orchestration.

The language is designed around:

- exact input/output bundle contracts;
- compact symbolic functions with lossless expansion;
- composable fan-out and fan-in;
- synchronous and asynchronous lanes;
- Goldilocks-sized `dE` slices;
- Warp transitions from `E0` to `Ef`;
- Eventness identifiers that existing Recur commands can trace.

The originating design notebook remains at `recur_language_start.md`.

## Artifact hierarchy

```text
docs/main.lang.readme.md
docs/main.command.lang.readme.md
demos/main.lang/main.lang.algorithm-lab.recur
demos/main.lang/main.lang.skippy-watch-coordination.recur
demos/main.lang/main.lang.runtime.jl
demos/main.lang/main.lang.cli.jl
julia-tests/main.lang.test.jl
```

The `main.lang` prefix deliberately marks language implementation, examples,
tests, and documentation wherever they occur in the repository.

## Command and companion split

The target command surfaces follow Recur's existing purity convention:

```text
recur lang   = pure language query and explanation
recur-lang   = coordination actor, lane state, receipt validation, ACK/NAK
```

`recur lang` renders compact/expanded views, checks contracts, and inspects
runtime Eventness without executing code or writing state. The current Julia
CLI interprets the algorithm fixture as a language-design spike. Production
`recur-lang` does not need to bundle target-language compilers, linkers, or
build systems.

See `docs/main.command.lang.readme.md` for the command contract.

## Compact and expanded views

The ordinary reading surface stays compact:

```recur
i(a) := (values: List<Int>)
o(b) := (values: List<Int>)

bubble sync : i(a) -> f(a) -> o(b)
merge  sync : i(b) -> f(b) -> o(c)

share bubble.o(b) -> merge.i(b)
```

`o(b)` and `i(b)` resolve to the same canonical signature. The Julia runtime
rejects a separately declared bundle even if its fields happen to look the
same.

Inspecting `gcd.f` reveals the familiar name and contract. Adding `--expand`
reveals its body. Both views come from one parsed model.

## Hierarchical subsystem composition

A validated subsystem may contract into one reusable block for a larger system.
The parent imports the child's exact public input/output identity, contract
version or content hash, and acceptance receipt. It does not copy a merely
similar record shape.

The compact parent view can therefore show `f(game.pathing@1)` while expansion
reveals pathing's internal lanes, joins, evidence, source spans, and Eventness.
Compatible internal changes can remain hidden behind the same public contract;
a breaking boundary receives a new identity.

Completion remains scoped. Child implementation completion and verification
may produce `child.integration.ready`, but only the parent can produce
`parent.child.integration.accepted`. Neither state implies that the entire
parent system is complete.

The `system`, `subsystem`, `public`, and `use` declarations remain proposed
syntax and are not accepted by the current Julia 0.1 parser.

## Recur Lang and `trace-id`

Recur Lang supplies closed-world semantics: it knows whether the declared
contracts, dependencies, joins, waits, feedback edges, scopes, and receipts
form a valid coordination model. `recur trace-id` supplies open-world lineage:
it finds stable identifiers across documentation, target-language code, tests,
Eventness, and receipts without parsing all of those artifacts as Recur Lang.

In short, `recur lang show gcd.f` explains a formal symbol; `recur trace-id
"demo.algorithm.gcd"` finds where that symbol's durable identity travels
through the wider project. Recur Lang should publish trace IDs, but neither
surface replaces the other. See `docs/main.command.trace-id.readme.md` for the
implemented scanner boundary and `docs/main.command.lang.readme.md` for the
proposed language commands.

## Run the Julia prototype

From the repository root:

```powershell
julia --startup-file=no demos/main.lang/main.lang.cli.jl list
julia --startup-file=no demos/main.lang/main.lang.cli.jl show gcd.f
julia --startup-file=no demos/main.lang/main.lang.cli.jl show gcd.f --expand
julia --startup-file=no demos/main.lang/main.lang.cli.jl show "merge.i(b)"
julia --startup-file=no demos/main.lang/main.lang.cli.jl run gcd left=1071 right=462
julia --startup-file=no demos/main.lang/main.lang.cli.jl run merge values=9,3,7,1,4
julia --startup-file=no demos/main.lang/main.lang.cli.jl run primes limit=30
julia --startup-file=no demos/main.lang/main.lang.cli.jl run pyramid rows=5 glyph=+
julia --startup-file=no demos/main.lang/main.lang.cli.jl run all
```

Inspection and execution commands also accept `--json`.

## Run the bounded `recur-lang` Warp companion

Inspect the declared transition without changing any files:

```powershell
cargo run --locked --bin recur-lang -- `
  warp demos/main.lang/main.lang.algorithm-lab.recur gcd --json
```

A confirmed transition additionally requires the exact current Eventness file
and a `recur-lang-warp-receipt-v1` bound to the source hash from the dry run:

```powershell
cargo run --locked --bin recur-lang -- `
  warp demos/main.lang/main.lang.algorithm-lab.recur gcd `
  --eventness docs/demo.algorithm.gcd.todo.current.md `
  --receipt receipts/gcd.001.md `
  --confirm
```

The writer renames only the named E0 artifact to Ef and records the result
beneath `.recur/lang/`. It does not execute `gcd.f`, Cargo, Julia, Git, or any
other worker command. The external receipt is evidence supplied by a worker;
its shape alone is not proof that the worker is trustworthy.

## Run the Julia tests

```powershell
julia --startup-file=no julia-tests/main.lang.test.jl
```

## Inspect it with Recur

```powershell
recur tree "main.lang" -d . --sep . --sep _ --show-sep
recur files "main.lang.**" -d . --sep . --sep _
recur trace-id "demo.algorithm.gcd.**" --scope "**" --ext ".recur" -d demos/main.lang
```

## Prototype boundary

The Julia runtime parses and validates the symbolic source. Each `by` name
currently resolves to a trusted Julia intrinsic. Bodies under `expand` are
readable language proposals rather than compiled code.

`main.lang.skippy-watch-coordination.recur` is a 0.2 design fixture rather than
accepted 0.1 parser input. It demonstrates the Improvement 30 direction:
coordinator and worker contracts, watch/work state machines, external tool
receipts, bounded feedback, and generated orchestration reports. Its formal
companion is
`docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`.
