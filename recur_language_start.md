# Eventness, Warp, and Goldilocks Slicing

## init

This is where we look over some of recur's basic features and think about who do we evolve it to a new programming language that handles all this and is easy for a human or ai to read.

## Overview

Eventness is a hierarchical, rediscoverable representation of project state that stores both knowledge and the means of reconstructing knowledge. Rather than acting as simple documentation, an eventness file serves as a rehydration capsule that helps both humans and AIs quickly regain context, understand the current state, discover related work, and determine the next steps.

Typically, eventness is encoded directly into the names of Markdown or text files.

Example:

```text
main.command.trace-id.run.todo.current.md
```

The file name itself conveys:

- Identity
- Hierarchical location
- Domain or lane
- Subject area
- Status
- Relationship to other work

Because the hierarchy is machine-readable, tools such as recur can navigate, discover, and reason about the work without requiring a separate database, project management system, or orchestration engine.

---

# Eventness as State + Rediscovery

Traditional documentation usually captures:

```text
What was known
```

Eventness captures:

```text
What was known
How it was discovered
How to rediscover it
```

An eventness file may contain:

- Current understanding
- Evidence
- References
- Related eventness nodes
- Discovery commands
- Validation procedures
- Next actions

The inclusion of discovery commands is particularly important.

For example:

```bash
recur tree "main.command.trace-id"
recur children "main.command"
recur related "main.command.trace-id.run.todo.current"
```

These commands provide a reproducible way to re-acquire knowledge rather than merely describing knowledge.

---

# Core Attributes of Eventness

An eventness node can be viewed as containing:

1. **Identity**
   - Unique hierarchical name.

2. **Scope**
   - Domain or subsystem represented.

3. **Status**
   - todo, current, recurring, complete, future-plan, etc.

4. **Relationships**
   - Parent, child, sibling, and related eventness nodes.

5. **Evidence**
   - Files, traces, outputs, references, and validation artifacts.

6. **Discovery Commands**
   - Queries and commands used to rediscover context.

7. **Rehydration Instructions**
   - Guidance for humans or AIs to quickly regain understanding.

8. **Workflow Hints**
   - Suggested next investigations or implementation steps.

9. **Recurrence**
   - Ongoing work that may be revisited repeatedly.

10. **Traceability**
    - Historical reasoning, decisions, and transitions.

---

# Eventness as a Time-Varying State

An eventness node is not static.

It evolves over time.

Represent the state as:

```text
E(t)
```

where:

- E = Eventness
- t = Time

At one point:

```text
main.command.trace-id.run.todo.current
```

Later:

```text
main.command.trace-id.run.complete
```

The eventness itself has evolved.

The hierarchy remains stable while knowledge, evidence, and status change.

---

# Warp

Warp is the concept of reasoning not only about the current eventness state, but also about a desired future eventness state.

Current State:

```text
E0
```

Desired Future State:

```text
Ef
```

Example:

Current:

```text
saved runs incomplete
tests missing
documentation partial
```

Desired:

```text
saved runs complete
all tests passing
documentation complete
demo validated
```

The gap between E0 and Ef represents the work still required.

Warp is the process of identifying and navigating that gap.

---

# Differential Eventness

This can be viewed similarly to differential calculus.

A small meaningful change in eventness is:

```text
dE
```

Examples:

Good dE:

```text
Implement saved run persistence
Add freshness validation
Add acceptance tests
```

Too Large:

```text
Build entire workflow engine
```

Too Small:

```text
Rename variable
Fix typo
```

The objective is to find the minimum meaningful transformation.

Conceptually:

```text
Ef - E0
```

represents the required change.

And:

```text
∫ dE = Ef - E0
```

Meaning:

The accumulation of appropriately sized changes transforms the current state into the desired future state.

---

# Goldilocks Slices

A Goldilocks Slice is a unit of progress that is:

- Small enough to validate
- Small enough for an AI or human to execute reliably
- Large enough to meaningfully advance the project

Too Coarse:

```text
Create complete orchestration platform
```

Too Fine:

```text
Change one variable name
```

Goldilocks:

```text
Add saved-run freshness validation

Acceptance Criteria:
- Detect stale inputs
- Add test coverage
- Update documentation
```

Goldilocks slices represent the ideal eventness differential.

---

# Warp as Navigation

Warp can be represented as:

```text
Warp(E0, Ef)
```

which produces:

```text
[dE1, dE2, dE3 ... dEn]
```

such that:

```text
E0 + Σ(dE) = Ef
```

The future state acts as a destination.

The current state acts as a starting point.

The slices become the route.

This turns project evolution into a navigation problem.

---

# Implications for Human and AI Collaboration

Traditional AI workflows often look like:

```text
Prompt
→ Task
→ Response
```

An eventness-driven workflow looks more like:

```text
Current Eventness
        ↓
Desired Eventness
        ↓
Differential Analysis
        ↓
Goldilocks Slice Selection
        ↓
Execution
        ↓
Updated Eventness
```

Humans and AIs are no longer simply completing isolated tasks.

Instead, they continuously reduce the distance between the current eventness state and the desired eventness state.

```text
distance(Ecurrent, Edesired)
```

When that distance approaches zero, the eventness naturally transitions from:

```text
todo.current
```

to:

```text
complete
```

---

# Summary

Eventness is a hierarchical, rediscoverable representation of project state that stores both knowledge and the means of reconstructing knowledge.

Warp is the process of imagining a desired future eventness state and determining the sequence of Goldilocks-sized transformations required to evolve the current state into that future state.

Together, Eventness and Warp create a lightweight orchestration model where humans and AIs can:

- Understand current state
- Rediscover context rapidly
- Envision desired outcomes
- Calculate meaningful next steps
- Continuously evolve projects through validated, appropriately sized slices of progress

In this model, project execution becomes the disciplined reduction of the distance between current eventness and desired eventness until the envisioned future becomes the present reality.

# Recur Language: a first executable sketch

Status: `brainstorming prototype`

This document explores a small language that works with Recur's hierarchical
names and Eventness model. It is intentionally a language sketch, not a claim
that the syntax is final.

The central idea is **progressive disclosure**:

- read a whole operation as a tiny symbolic flow;
- expand a symbol only when its details matter;
- give every short symbol a stable, fully qualified name for lookup;
- keep inputs at the far left and final outputs at the far right;
- make synchronous and asynchronous lanes visible in the source;
- emit hierarchical event identifiers that existing Recur tools can find.

The canonical prototype hierarchy is `main.lang`. Its implementation lives in
`demos/main.lang/`, its tests in `julia-tests/main.lang.test.jl`, and its
artifact index in `docs/main.lang.readme.md`.

## The bridge from Warp to code

The preceding model describes a transition in project state:

```text
E0 -> dE -> Ef
```

The language describes a transition in program data:

```recur
i(a) -> f(a) -> o(b)
```

These should align, but they should not be confused. `a` and `b` are data
contracts. `E0` and `Ef` are rediscoverable Eventness states. A function `f`
can be the Goldilocks-sized change `dE`, or a larger function can be decomposed
into several Goldilocks lanes.

The prototype connects the two explicitly:

```recur
warp gcd : E0(demo.algorithm.gcd.todo.current)
        -> dE(gcd.f)
        -> Ef(demo.algorithm.gcd.complete)
```

This gives one operation two useful projections:

```text
data projection    i(a) -> f(a) -> o(b)
state projection   E0  -> dE(f) -> Ef
```

The data projection answers "what enters and leaves?" The state projection
answers "what known state changes when this slice succeeds?" The footer keeps
the state projection because it is observable lifecycle metadata rather than
the function's internal calculation.

## The smallest useful view

```recur
gcd sync : i(a) -> f(a) -> o(b)
```

Read this from left to right:

1. `i(a)` receives the input bundle named `a`.
2. `f(a)` applies the locally named function `f`.
3. The declared signature of `f` says that it produces bundle `b`.
4. `o(b)` exposes `b` as the final output.

The line remains readable whether `f` expands to five lines or five million
lines.

`a` and `b` are not ordinary scalar variables. They are **bundle symbols**:
local aliases for complete, typed input or output contracts.

```recur
scope gcd {
  i(a) := (left: Int, right: Int)
  o(b) := (value: Int)
  f : i(a) -> o(b) ~ "Euclid greatest common divisor" by gcd.euclid
}
```

A bundle can hold ten fields just as easily as one:

```recur
i(a) := (
  customer: Customer,
  cart: Cart,
  currency: Currency,
  region: Region,
  coupon: Coupon?,
  inventory: Inventory,
  tax: TaxPolicy,
  shipping: ShippingPolicy,
  clock: Clock,
  request_id: Text
)
```

The flow still says `i(a) -> f(a) -> o(b)`.

## Symbols are local; lookup names are not

Short symbols resolve in their nearest scope. Unconnected scopes may reuse the
same letter without implying a relationship. Once a boundary is connected,
however, its producer and consumer must resolve to one canonical declaration.

```text
gcd.i(a)    full GCD input bundle
gcd.o(b)    full GCD output bundle
gcd.f       Euclid implementation
bubble.o(b) canonical sorted-list boundary
merge.i(b)  alias of bubble.o(b), not a second similar declaration
merge.f     merge-sort implementation
```

Inside the scope, use the small symbol. Outside it, use the qualified name.
This gives the reader a compact alphabet without sacrificing searchability.

The shared boundary is explicit:

```recur
scope bubble {
  o(b) := (values: List<Int>)
}

scope merge {
  i(b) := bubble.o(b)
}

bubble sync : i(a) -> f(a) -> o(b)
merge  sync : i(b) -> f(b) -> o(c)

share bubble.o(b) -> merge.i(b)
```

Here `o(b)` and `i(b)` are the two ends of the same port. `merge` does not
redeclare a merely compatible bundle; `i(b) := bubble.o(b)` imports the actual
boundary declaration while preserving its input role.

At a call boundary, the output contract of one block must match the input
contract of the next block:

```recur
pipeline sync : i(a) -> f(a) -> g(b) -> h(c) -> o(d)
```

The declarations `f : i(a) -> o(b)`, `g : i(b) -> o(c)`, and
`h : i(c) -> o(d)` are enough for a type checker to verify all three edges.
In the first version, an exact match
means the same canonical bundle declaration, field names, field types, field
order, cardinality, and defaults. A structurally similar anonymous bundle is
not enough. A deliberate signature change requires an explicit adapter block.

## Familiar names and expansion

The one-letter symbol is the reading surface. The familiar name and
implementation are discoverable metadata:

```recur
f : i(a) -> o(b) ~ "Euclid greatest common divisor" by gcd.euclid
```

- `f` is what appears in the collapsed flow.
- `a -> b` is the contract.
- `~ "..."` is the familiar human name shown on hover or lookup.
- `by gcd.euclid` selects an implementation.

The source may also carry a readable expansion:

```recur
expand gcd.f {
  while a.right != 0 {
    a := (left: a.right, right: a.left % a.right)
  }
  emit b(value: abs(a.left))
}
```

An editor or CLI should expose three levels:

```text
collapsed   i(a) -> f(a) -> o(b)
inspect     f : i(a) -> o(b) ~ "Euclid greatest common divisor"
expanded    full body, source location, events, tests, and callers
```

This is the language's most important user-interface rule. Details remain
available, but they do not continually occupy the reading surface.

## A class/file shape

A first file can represent one class-like namespace:

```recur
recur 0.1 class AlgorithmLab

header {
  # Stable scopes, bundle contracts, function signatures, and familiar names.
}

body {
  # Collapsed flows first; optional expansions after them.
}

footer {
  # Public exports and observable event identifiers.
}
```

The sections have different rates of change:

- `header` is the stable symbol dictionary.
- `body` is the executable dataflow and its drill-down bodies.
- `footer` is the outside-facing contract: exports, events, completion states,
  and eventually assertions.

`class` should initially mean namespace plus shared lifecycle, not inheritance.
Inheritance, visibility rules, and mutable instances would add complexity
before the symbolic-flow model has been proven.

## Multiple lanes

Synchronous work uses an ordinary flow:

```recur
gcd sync : i(a) -> f(a) -> o(b)
```

Independent work can be made explicit:

```recur
all async : [gcd, bubble, merge, primes, pyramid] -> await -> o(results)
```

The proposed rules are:

- `sync` evaluates each arrow in order.
- `async` starts the listed lanes concurrently.
- `await` is a visible join point.
- the joined output remains on the far right;
- a failed lane makes the join fail unless a future policy says otherwise.

This is intentionally smaller than implicit "async all the way down." A reader
can see the concurrency boundary and the join in one line.

## Eventness connection

Each flow can declare identifiers in the footer:

```recur
event gcd {
  consume demo.algorithm.gcd.input
  trigger demo.algorithm.gcd.run
  produce demo.algorithm.gcd.output
  state demo.algorithm.gcd.complete
}

warp gcd : E0(demo.algorithm.gcd.todo.current)
        -> dE(gcd.f)
        -> Ef(demo.algorithm.gcd.complete)
```

This connects runtime flow to Recur's existing ideas:

- prefix/base routes the subject: `demo.algorithm.gcd`;
- the right-hand suffix states the interesting edge or state;
- `consume`, `trigger`, and `produce` align with `recur trace-id`;
- `.complete`, `.current`, `.blocked`, and related suffixes remain Eventness
  signals rather than language control-flow keywords.

The language runtime should report events. Recur should discover, trace, rank,
and relate them. Those are complementary jobs.

## Requested algorithm examples

The runnable sketch contains:

```recur
gcd     sync : i(a) -> f(a) -> o(b)
bubble  sync : i(a) -> f(a) -> o(b)
merge   sync : i(b) -> f(b) -> o(c)
primes  sync : i(a) -> f(a) -> o(b)
pyramid sync : i(a) -> f(a) -> o(b)

share bubble.o(b) -> merge.i(b)

all async : [gcd, bubble, merge, primes, pyramid] -> await -> o(results)
```

Most algorithms reuse `a`, `b`, and `f`; their meaning comes from the nearest
named scope. Bubble and merge deliberately share `b`, so both ends resolve to
the canonical `bubble.o(b)` signature. This is the intended "throw around `a` to
our delight" behavior without allowing a connected boundary to drift.

## Prototype commands

From the repository root:

```powershell
julia --startup-file=no demos/main.lang/main.lang.cli.jl list
julia --startup-file=no demos/main.lang/main.lang.cli.jl show gcd
julia --startup-file=no demos/main.lang/main.lang.cli.jl show gcd.f --expand
julia --startup-file=no demos/main.lang/main.lang.cli.jl show "pyramid.i(a)"
julia --startup-file=no demos/main.lang/main.lang.cli.jl run gcd left=1071 right=462
julia --startup-file=no demos/main.lang/main.lang.cli.jl run merge values=9,3,7,1,4
julia --startup-file=no demos/main.lang/main.lang.cli.jl run primes limit=30
julia --startup-file=no demos/main.lang/main.lang.cli.jl run pyramid rows=5 glyph=+
julia --startup-file=no demos/main.lang/main.lang.cli.jl run all
```

The app parses the header/body/footer structure, validates each collapsed
contract and Warp projection, looks up qualified symbols, executes the five
algorithms, and uses an asynchronous join for `all`.

## What the prototype intentionally does not solve

The Julia runtime currently maps `by gcd.euclid` and similar names to trusted
intrinsics. The `expand` body is inspected but is not yet compiled. That
separates two questions:

1. Is symbolic reading plus expansion pleasant and useful?
2. What should the full implementation language and compiler be?

The first question is cheaper and more important to test now. If it works, a
next parser can turn expansion bodies into an AST and replace the intrinsics.

Other deliberately deferred features include:

- mutation and object identity;
- inheritance;
- generic types;
- overload resolution;
- cancellation and retry policy for async lanes;
- persistence of emitted Eventness;
- imports and cross-file symbol resolution;
- editor folding, hover, and click-to-expand support.

## Recommended evolution

1. Keep this spike as the executable language fixture.
2. Try reading and editing several larger flows using only their collapsed
   form.
3. Add a real lexer, parser, AST, and source spans.
4. Make edge compatibility a type-checking pass.
5. Compile a small expression/loop subset used by GCD, bubble sort, sieve, and
   pyramid.
6. Add recursion needed by merge sort.
7. Add editor folding/hover so expansion becomes a first-class interaction.
8. Expose emitted IDs to `recur trace-id`, `recur reveal`, and Eventness
   scoring.

The design succeeds if a reader can understand the system from the small flow,
then obtain every required detail without losing their place.

## Recur command split

The language follows Recur's pure-query/companion-actor convention:

```text
recur lang   = inspect, validate, expand, contract, trace, and explain
recur-lang   = interpret, compile, execute lanes/slices, and emit Eventness
```

`recur lang expand` and `recur lang contract` are lossless read-only views over
one canonical program model. The current Julia CLI is a prototype of the
`recur-lang` companion. The detailed boundary is recorded in
`docs/main.command.lang.readme.md`.
