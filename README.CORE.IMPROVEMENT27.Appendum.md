# Warp Methodology v0.3.0

## Eventness, Recur, Warp, and Intelligence Routing

Status: Improvement 27 methodology addendum and design proposal; not a shipped
runtime contract.

Implementation target: Recur v0.2.8. The first compositional runtime slice now
ships `recur warp map`, `recur warp merge`, status integration, and the
confirmation-gated `recur-warp complete` writer. Warp evolution remains a
later write-side slice.

Ownership: this document clarifies the human and multi-intelligence mental
model behind `README.CORE.IMPROVEMENT27.md` and its proposed read-first
`recur warp` project-control surface. It does not override Recur's purity
boundary.

Improvement 30 has a separate responsibility. Recur Lang condenses exact
inputs, functions or methods, outputs, dependencies, branches, joins, waits,
and evidence relationships into a checkable semantic map such as:

```text
i(a) -> f(a) -> o(b)
```

That map helps detect circular dependencies, missing producers, incompatible
contracts, unsatisfied joins, unreachable work, and related development
problems. Recur Lang may consume a selected Warp, Slice, or receipt as an exact
contract, but Improvement 30 does not own or redefine Warp methodology.

## Purpose

Warp is an intelligence-coordination framework for moving toward a desired
future through bounded Slices.

A Warp is the current explicit, versioned coordination model used by
participating intelligences. Eventness contextualizes that model so the
participants can see what is active, relevant, uncertain, contested, stale,
blocked, or changing.

A Warp is not reality, ground truth, a private mind, a project plan, or
authority to act. It is an inspectable working interpretation that must remain
connected to observations and evidence.

---

# Core Boundaries

| Concept | Responsibility | It is not |
|---|---|---|
| Observed state | Evidence-backed description of relevant conditions | complete reality or unquestionable truth |
| Local model | One intelligence's stated interpretation, confidence, and unknowns | automatically shared understanding |
| Warp | Versioned coordination projection across participating intelligences | reality, consensus by force, or an execution engine |
| Eventness | Context and attention around subjects, models, transitions, and evidence | the whole world state |
| Slice | One bounded attempted state transition under the current Warp | automatically a new Warp |
| Receipt | Durable evidence of what was attempted and observed | proof merely because its shape is valid |
| Companion actor | Explicitly authorized mutation or coordination mechanism | core `recur` or implicit permission |

The boundaries matter because coordination fails when a model is mistaken for
truth, disagreement is mistaken for consensus, a plan is mistaken for action,
or an action is mistaken for accepted evidence.

---

# Eventness

Eventness contextualizes what deserves attention now within a hierarchy. It
may describe lifecycle, interest, uncertainty, conflict, freshness, readiness,
verification, or another project-defined coordination fact.

Examples:

- `todo.current`
- `risk.high`
- `evidence.stale`
- `hypothesis.contested`
- `verification.needed`
- `integration.ready`
- `integration.accepted`

Eventness can point to an observed-state record or a Warp assumption, but it
does not turn either one into reality. Prefix and base identify the stable
subject; suffixes and related facts explain why that subject is interesting.

## E0

`E0` identifies the current Eventness state for the bounded subject represented
by a Warp. It is not a claim to encode all present reality.

## Ef

`Ef` identifies the desired Eventness state under the current Warp. It is a
versioned target interpretation, not an inevitable or permanently final world.

The destination may evolve when observations change the coordination model.

---

# Warp

A Warp is a snapshot of coordinated understanding with durable identity and
evidence references.

A useful Warp may contain:

- Mission and scope
- Observed-state references
- Shared understanding
- Assumptions
- Constraints and invariants
- Unknowns
- Contested hypotheses
- Participating-intelligence viewpoints
- Confidence and evidence freshness
- Current interpretation of the desired outcome
- Candidate Slices and intelligence-routing policy
- Explicit conditions under which the Warp should evolve

The Warp artifact represents what intelligences have made explicit for
coordination. It must not claim access to their unexpressed private reasoning.

## Shared understanding without forced consensus

Multiple intelligences do not need to agree about everything before useful
work begins. The Warp should distinguish:

```text
observed fact
shared interpretation
assumption
local viewpoint
contested hypothesis
unknown
```

Conflicting outputs remain visible until evidence or an authorized decision
resolves them. A coordinator must not silently replace one viewpoint with
another or report consensus merely because only one result was retained.

## Warp is not a project plan

A plan schedules intended work. A Warp explains the current coordination model
that makes particular work appear useful. Plans may be derived from a Warp,
but changing a task list alone does not necessarily create a new Warp.

## Warp is not authority

A valid Warp may recommend a Slice. It does not authorize filesystem writes,
deployments, purchases, messages, merges, or other external effects. Authority
comes from the operator and the applicable execution policy.

---

# Slices

Slices are manageable attempted state transitions selected under a Warp.

Each Slice should identify:

- Stable Slice and Warp identities
- Exact starting subject or input contract
- Intended delta
- Assumptions and relevant unknowns
- Allowed actions and write scope
- Owner or routed intelligence
- Required evidence and acceptance criteria
- Recovery or rollback behavior
- Resulting Eventness on ACK or NAK

Example:

```text
Current:
User cannot upload a selected file reliably.

Slice:
Transfer one selected file through a bounded upload contract.

Desired evidence:
Accepted types succeed, rejected types explain why, interrupted transfer is
recoverable, and the observed result is recorded in a receipt.
```

A route choice, function call, or ordinary task is normally part of a Slice.
It becomes a reason to evolve the Warp only when it materially changes the
shared assumptions, constraints, hypotheses, or desired-outcome interpretation.

---

# Compositional Slice Layers and Warp Bubbles

A Warp may declare a final bubble map containing the exact qualified Slices,
public contract identities, and evidence gates required for convergence. The
map is a target interpretation under the current Warp, not a claim about
reality and not a schedule that forces one completion order.

Each accepted Slice contributes a completion layer:

```text
warp.id
slice.id
slice.contract.hash
attempt.id
result.state = accepted
coverage = [qualified target entries]
evidence.refs = [receipts, tests, artifacts, observations]
```

The pure composed view is:

```text
compose(target map, accepted completion layers)
  -> covered
  -> pending
  -> blocked
  -> stale-contract
  -> conflicting
  -> complete | incomplete | exploded
```

The layers and their accepted receipts are canonical coordination evidence.
The merged bubble is a deterministic projection and may be materialized as a
cache, but it must remain reproducible from the target map and accepted layers.
Deleting or rebuilding that cache must not change Warp meaning.

## Ring topology and recursive Recur domains

A Warp bubble is not only a flat set of Slices. It may be rendered as recursive
rings:

```text
outer coordination ring
  mission, shared constraints, domain contracts, subscriptions, integration gates

  inner domain ring: docs-monkey/
    local .recur/config.toml, reveal, Eventness, watch state, Warp and Slices

  inner domain ring: test-bird/
    local .recur/config.toml, reveal, Eventness, watch state, Warp and Slices

  inner domain ring: implementation-agent/
    local .recur/config.toml, reveal, Eventness, watch state, Warp and Slices
```

The names “monkey,” “bird brain,” agent, worker, specialist, coordinator, and
orchestrator are local persona or routing vocabulary. Warp cares about the
declared domain identity, boundary, contract, subscription, and evidence—not
which intelligence implementation lives there.

The outer ring belongs to the coordinator or orchestrator. It maintains the
mission-wide target, shared invariants, required subdomains, public contract
hashes, cross-domain dependencies, integration acceptance, conflict visibility,
and convergence state. It does not absorb every private file or every thought
from the inner rings.

Each inner ring is a physical directory domain. It can run `recur init` and
therefore own an independent `.recur/config.toml`. Commands inside that domain
resolve the nearest config, so the domain can choose its own lanes, separators,
reveal capsule, traits, Eventness vocabulary, watcher state, and local Warp.
Inner rings can recursively contain still smaller initialized domains.

### Public boundary and private habitat

An inner domain's `.recur/` directory is its private coordination habitat. The
parent should not scrape that vault as if it were the child's public contract.
The child publishes an intentional boundary in its ordinary project tree:

```text
domain.id
domain.root
domain.warp.id
domain.public-contract.hash
domain.required-state
domain.parent-acceptance-slice
domain.evidence.refs
```

The outer ring imports that exact boundary and content hash. Internal child
changes may remain local when the boundary is preserved. A changed public hash
invalidates prior parent acceptance and becomes visible as stale or exploded
outer-ring evidence.

### Subscription edges

The rings coordinate through explicit subscription edges rather than hidden
shared memory:

```text
parent -> child: mission slice, constraints, contract changes, cancellation
child -> parent: accepted layer, readiness, blocker, NAK, public-contract change
peer   -> peer: declared dependency output or evidence handoff
```

`recur-watch` can implement filesystem subscriptions today, while core
`recur watch` remains the pure state query. A future ring schema should record
the subscription identity, direction, source domain, target domain, filter,
public event contract, freshness policy, and ACK/NAK state. A subscription is
coordination evidence, not authority for the subscriber to mutate the
publisher's domain.

### Recursive completion rule

Child completion never leaks upward automatically. The outer Warp is complete
only when:

1. its own required coordinator-ring Slices and evidence gates are covered;
2. every required inner domain reaches its declared public state;
3. each imported child contract hash matches the outer declaration;
4. the corresponding outer parent-acceptance Slice is covered;
5. required subscription edges are current and non-conflicting; and
6. no inner or outer required ring is blocked, stale, conflicting, or exploded.

This preserves local autonomy while making global convergence deterministic.
The bubble can be rendered dynamically: discover domain roots, load each exact
map, recursively compose its accepted layers, then project each child result
into the outer ring through the declared public boundary.

### Ring guardrails

- A domain root must resolve inside its declared parent workspace unless an
  explicit external-domain policy says otherwise.
- Recursive domain references must be cycle-checked before traversal.
- A parent imports public boundary evidence, not the child's private `.recur/`
  vault.
- A child cannot declare its own parent acceptance.
- A coordinator cannot rewrite child-local state merely because it subscribes
  to child events.
- Missing, stale, ambiguous, or conflicting subscriptions remain visible holes
  in the outer ring.
- Ring depth and traversal use explicit budgets so recursive coordination does
  not become unbounded context ingestion.

The shipped `warp-bubble-map-v1` remains a flat first slice. Recursive domains,
nested projections, and subscription edges require a separately frozen schema
revision and red-first fixtures; they must not be inferred from arbitrary
directory nesting.

## Merge laws

The composition contract must define and test:

1. **Qualified identity.** Coverage belongs to one exact Warp, Slice, target
   entry, and public contract version or content hash.
2. **Associativity where declared.** Grouping independent layers differently
   yields the same normalized projection.
3. **Commutativity where declared.** Independent Slice readiness order does
   not change normalized coverage, while authored order remains available for
   explanation.
4. **Idempotence.** Replaying one identical qualified receipt does not change
   coverage.
5. **Conflict rejection.** Incompatible values for the same qualified target
   entry produce a stable conflict/NAK; no layer silently wins.
6. **Determinism.** Equal target map, accepted layers, policy, and normalized
   inputs produce byte-equivalent normalized output.
7. **Exact cardinality.** Every required target entry is accounted for exactly
   once before the Warp reports complete.
8. **Monotonic independent growth.** A valid non-conflicting layer may add
   coverage without erasing unrelated accepted facts.

Dependency order remains distinct from completion order. A Slice cannot be
accepted before its declared producers, waits, or evidence gates are
satisfied. Slices without such relationships may complete in any order.

## Self-reporting composition

The automatic merge is the Warp's status report. When an accepted Slice layer
appears, the composed view changes without a human, project manager, or
particular LLM rewriting summary prose. The next participant can query the
remaining holes instead of reconstructing prior work from conversation.

This is self-reporting, not self-proving. Composition can report which
receipts were accepted and how their declared coverage fits. Independent
verification and acceptance policy establish whether a receipt deserves to
participate.

## Pure query and write-side actor boundary

```text
recur warp map <warp>       inspect the declared final bubble map
recur warp merge <warp>     purely compose and report accepted layers
recur warp status <warp>    report coverage and convergence state
recur warp explain <warp>   explain gaps conflicts hashes and evidence

recur-warp complete ...     confirmed persistence of a Slice receipt/layer
recur-warp evolve ...       future confirmed Warp supersession record
```

`recur warp merge` does not modify project state. The merge happens naturally
as a derived consequence of accepted layers becoming discoverable. A
write-side companion must validate its declared authority before persisting a
receipt, accepting a layer, materializing a cache, or recording an evolution.

## Observable bubble explosion

A bubble is exploded when accepted evidence prevents the current target model
from converging: for example, a contract hash changed, two qualified layers
conflict, a material assumption was falsified, or the Slice decomposition
became invalid.

An explosion report should identify:

```text
warp.id
triggering.receipt
conflict.or.falsified-assumption
layers.preserved
layers.invalidated
slices.unresolved
evidence.refs
candidate.evolution
```

Explosion is an observable Eventness transition, not deletion and not hidden
model confusion. Valid evidence survives. Warp evolution forms a revised
target bubble, carries forward layers whose exact contracts remain valid, and
explicitly invalidates, replaces, adds, or retires the rest.

---

# Evolution of Warp

Warp and Slice timelines are separate:

```text
Mission M

Warp W0
  Slice S1  accepted
  Slice S2  accepted
  Slice S3  NAK; evidence challenges assumption A

Warp W1 supersedes W0
  preserves mission M and accepted evidence
  revises assumption A
  carries or retires remaining Slices explicitly
  Slice S4  accepted
```

The mission can survive while the theory changes. Completed history is not
renamed to fit the new theory.

## When a Warp should evolve

Evidence should trigger a candidate evolution when it:

- Falsifies a material assumption
- Reveals an important unknown or disagreement
- Changes a constraint or invariant
- Shows that the desired outcome was misunderstood
- Makes the current Slice decomposition misleading
- Changes the appropriate intelligence or verification route
- Produces repeated surprises that the current model does not explain

A new feature, elapsed sprint, or routine route selection is not sufficient by
itself.

## Warp Evolution Record

An evolution should preserve an inspectable difference:

```text
warp.id
supersedes
mission.id
evolution.trigger
observations
assumptions.preserved
assumptions.revised-or-falsified
constraints.changed
unknowns-and-disagreements
desired-outcome.changed
slices.carried-forward
slices.retired
confidence.before
confidence.after
evidence.refs
trace.ids
```

This record explains why the new Warp exists and prevents arbitrary Warp
numbering from replacing model evolution.

---

# Recur and Companion Actors

Core `recur` is a pure hierarchy, memory, query, and explanation surface.

It helps retrieve:

- Warps and Warp-evolution records
- Slices and decisions
- Tests and receipts
- Observations and failures
- Research and evidence
- Current, blocked, stale, contested, and recurring Eventness

Purpose: retrieve knowledge before rediscovering it and expose the smallest
useful context for the current question.

Opinionated companion actors may perform explicitly confirmed operations and
leave durable ACK/NAK evidence. Examples include `recur-lang`, `recur-git`, and
`recur-watch` within their declared boundaries.

The split is:

```text
recur <topic>   = query, inspect, explain
recur-<topic>   = bounded runner, writer, watcher, or coordination actor
```

No command should declare its own work accepted merely because it ran. An
authorized actor produces a receipt; required evidence and the appropriate
operator or verification policy determine acceptance.

---

# Intelligence Routing

Not all Slices deserve the same intelligence, coordination, or verification
cost. Routing should not depend on novelty alone or hard-code today's model
pricing into doctrine.

Assess at least:

- Uncertainty and novelty
- Impact and downstream fan-out
- Reversibility and recovery cost
- Strength of available tests or evidence
- Coordination and context burden
- Data, credential, financial, safety, or privacy sensitivity
- Need for specialist knowledge
- Time and resource budget

Example routing bands:

## Routine and reversible

Route: deterministic automation or one bounded worker, with ordinary checks.

## Known variation

Route: capable local or general intelligence with focused validation.

## Uncertain but recoverable

Route: human-plus-intelligence exploration, explicit hypotheses, and cheap
experiments before commitment.

## Novel or highly coupled

Route: stronger specialist intelligence, independent review, and explicit
integration evidence.

## High-impact, sensitive, or difficult to reverse

Route: strongest appropriate capability, human authorization, independent
verification, bounded execution, and tested rollback.

A known payment or deployment change can deserve more care than a novel but
disposable prototype. The routing decision and its outcome should be recorded
so later Warps can learn whether the allocation was effective.

---

# Long-Context Strategy

Traditional systems often accumulate context without preserving why each fact
still matters. Warp uses progressive disclosure and evidence-backed
distillation.

The active coordination capsule contains:

- Mission
- Current observed-state references
- Current Warp
- Current bounded Slices
- Constraints and invariants
- Unknowns and contested hypotheses
- Evidence freshness and confidence
- Exact references and triggers for retrieving deeper history

Recur stores durable history and rediscovery points. Distillation must not
discard provenance, unresolved contradictions, rejected alternatives that
still affect a decision, or evidence needed for safety and rollback.

Only the smallest sufficient context remains in focus; the rest remains
addressable and can be rehydrated when a trigger, anomaly, or question requires
it.

---

# Game Simulation Example

## Cuzco Tourism Simulation

The Godot tourism game is a useful Warp dogfooding environment because it
combines route choice, money, preparation, uncertainty, experience value, and
player feedback.

## Mission

Help the player discover worthwhile Cuzco landmarks and tours while learning
how preparation, access, transportation, comfort, time, and complete trip cost
shape the experience.

Overspending is a visible, recoverable travel story rather than an automatic
failure. Spending less is not automatically better.

## Observed-state references

The game may observe or simulate:

- Location and visited landmarks
- Planned, actual, remaining, and contingency money
- Original and display currencies
- Time, energy, food, drink, facilities, comfort, and access
- Preparation coverage
- Learning, fun, confidence, and memory
- Photos and achievements
- Transportation access and contextual route conditions
- Evidence date, provenance, and uncertainty for travel claims

These values describe the bounded game state. Eventness identifies which
subjects currently need attention—for example budget variance, stale route
evidence, an unmet comfort need, or a contested value hypothesis.

## Example Warp evolution

```text
Warp W4 hypothesis:
  A generic Space-Invaders-style money mechanic will make landmark spending
  entertaining.

Observation:
  Player feedback says generic invaders feel cheesy and disconnected from
  tourism, while the money concept remains promising.

Warp W5 hypothesis:
  Landmark-specific people, services, value choices, local expectations, and
  a comic llama or alpaca finale will connect arcade play to tourism.

Candidate Slices:
  airport-specific cast
  site-specific waves
  cost-versus-value decision
  first-landmark callback
  mascot finale
```

The mission survives. Evidence changes the coordinated mental model, and the
new model changes which Slices appear useful.

## Pathfinding within a Slice

```text
Current Node
  -> Desired Node
  -> Candidate Routes
  -> Contextual Cost and Experience Evaluation
  -> Route Selection
  -> Travel Receipt
```

This is normally a decision inside a Slice, not a micro-Warp. It may trigger a
Warp evolution when repeated route outcomes falsify the current transportation
or value model.

## Transportation effects are contextual

Walking, bus, taxi, and other options may affect money, time, energy, comfort,
access, confidence, and experience differently. Their effects should use
ranges, conditions, provenance, and confidence where appropriate rather than
universal labels such as "taxi is always high cost" or "walking is always
free."

## Route memory

Recur can retrieve historical route evidence, but a previous path is not a
timeless recommendation. A useful record includes:

- Origin, destination, and route
- Traveler or gameplay strategy
- Transportation modes
- Complete cost and time
- Energy, comfort, access, learning, fun, and satisfaction outcomes
- Conditions and exceptions
- Evidence source and date
- Confidence and known gaps
- Warp and trace identities

Queries can then ask for a route fitting the current constraints rather than
returning an unqualified "cheapest" route.

---

# Star Cruiser Example

The same boundaries can apply to a speculative engineering program:

Routine, well-tested work may use bounded automation. Novel or dangerous work
may require specialist intelligences, explicit uncertainty, simulation,
independent verification, human authority, and reversible experiments.

The Warp remains a coordination model. It does not make frontier physics true,
prove engineering feasibility, or authorize construction.

---

# Clear-Boundary Invariants

- Never represent a Warp as reality or ground truth.
- Never imply access to an intelligence's unexpressed private mental state.
- Never erase disagreement to manufacture shared understanding.
- Never treat Eventness as the complete world state.
- Never treat an ordinary action or route choice as a new Warp by default.
- Never evolve a Warp without a recorded trigger and evidence references.
- Never let a Slice silently expand its allowed scope.
- Never treat a well-shaped receipt as sufficient proof.
- Never let core `recur` mutate project reality.
- Never let a companion actor infer authority from a valid model.
- Never route intelligence using novelty or price alone.
- Never distill context in a way that loses provenance, safety evidence,
  unresolved conflict, or rollback knowledge.

---

# Ultimate Objective

Warp helps systems of humans, intelligences, and deterministic tools maintain
enough explicit shared understanding to coordinate toward increasingly
difficult future states.

Software projects and simulations are training grounds for:

- Evidence-backed model evolution
- Human and multi-intelligence coordination
- Scientific discovery
- Large-scale engineering
- Other difficult missions whose solutions cannot be fully planned in advance

---

# Philosophy

Observed evidence constrains the model.

Recur remembers and explains.

Eventness routes context and attention.

Warp represents the current coordination model.

Slices attempt bounded changes.

Companion actors perform explicitly authorized operations.

Receipts record what happened.

Humans and intelligences verify, disagree, learn, and revise.

The mission can survive even when a Warp fails.

---

# Improvement 27 / Improvement 30 Integration Boundary

Improvement 27 answers coordination-model questions:

```text
What do the participating intelligences currently understand?
What is observed, assumed, unknown, contested, stale, or blocked?
What future Eventness are they trying to converge toward?
Which bounded Slice appears useful next, and why?
What evidence should cause the Warp to evolve?
```

Improvement 30 answers semantic relationship questions for a declared bounded
model:

```text
What exact input does this function or method consume?
What exact output does it produce?
Which downstream function consumes that output?
Are contracts compatible?
Do dependencies, waits, branches, joins, or retries create a cycle?
Is any required producer, consumer, handler, or evidence gate missing?
```

The integration direction is:

```text
Improvement 27 Warp
  -> selects and contextualizes a bounded Slice
  -> Improvement 30 Recur Lang maps the Slice's exact functional relationships
  -> external intelligences and toolchains implement and verify the work
  -> receipts return observations to Eventness
  -> Improvement 27 retains or evolves the Warp
```

Neither improvement silently performs target-language work. Neither may treat
a valid model as proof that the implementation is correct.

---

# Trace-Id Lines

```text
defines: main.improvement.27.appendum.warp.methodology explicit versioned multi-intelligence coordination mental model
defines: recur.warp.boundary observed state local model Warp Eventness Slice receipt and companion actor separation
defines: recur.warp.evolution evidence-triggered supersession with preserved mission assumptions disagreements confidence and trace identity
defines: recur.warp.bubble.composition final target manifest plus accepted qualified Slice completion layers
defines: recur.warp.ring.topology outer coordinator convergence ring containing recursive specialized Recur directory domains
defines: recur.warp.inner-domain independently initialized nearest-config scope with local reveal Eventness subscriptions Warp and Slices
defines: recur.warp.domain.public-boundary exact versioned or content-hashed child contract exported without exposing private vault state
defines: recur.warp.subscription.edge explicit directional parent child or peer event contract with freshness and ACK NAK state
defines: recur.warp.recursive-completion outer completion requires child public readiness exact hash and separate parent integration acceptance
defines: recur.warp.merge-laws qualified identity associativity commutativity idempotence determinism conflict rejection cardinality and monotonic growth
defines: recur.warp.self-reporting automatic derived status through composition rather than manually rewritten progress prose
defines: recur.warp.observable-explosion auditable non-convergence preserving valid evidence and triggering candidate Warp evolution
defines: recur.warp.intelligence-routing uncertainty impact reversibility verification coupling sensitivity and resource assessment
defines: recur.warp.lang.boundary Improvement 27 owns evolving coordination context while Improvement 30 owns exact input function output and dependency mapping
consumes: main.improvement.27 Eventness Warp and project-control command proposal
consumes: main.recur.purity.decision pure query surface and opinionated companion actor boundary
produces: recur.warp.context-capsule smallest sufficient active coordination context with durable rediscovery references
produces: recur.warp.coverage.projection reproducible covered pending blocked stale conflicting and complete bubble view
produces: recur.warp.ring.projection dynamically nested outer and inner domain status without flattening ownership boundaries
produces: main.improvement.30.input bounded Warp Slice and receipt contracts that Recur Lang may map without redefining Warp
triggers: main.improvement.warp-evolution future bounded schema and tourism-game dogfooding fixture
```
