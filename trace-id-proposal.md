# Proposal: `recur trace-id` — Hierarchical Identifier Flow Tracing

> **Status:** Proposal
> **Author:** Discovered during ulu-11 `users.dot` reactive chain tracing session
> **Parent:** `docs/agents/recur-agent.static-analysis.md` → "The Missing Feature"
> **Recur version at time of writing:** 0.2.6

---

## Problem

`recur trace` follows **function calls** (synchronous call graph).
`recur id` finds **hierarchical string identifiers** (where they appear).

But modern architectures connect components through **string-mediated async routing** —
pub/sub topics, event suffixes, message bus routing keys, content type discriminators.
These connections are invisible to `trace` (no function call to follow) and flat in `id`
(no directionality — can't distinguish publisher from subscriber).

### Real-world example: tracing one reactive chain hop

The `users.dot` system routes events like this:

```
dot suffix "level.create"                              ← string in controller
  → DotPatternRegistry matches "*.level.create"        ← glob pattern in DotWatcher
    → PublishAsync(DotControlTopics.OwnershipCreate)   ← publishes to topic string
      → OwnershipCreateSubscriber binds to topic       ← subscribes via same string
        → ownershipService.CreatePrivate...()          ← function call (trace works here)
```

**Today this requires 5 manual commands:**

```bash
# Step 1: Find all topic identifiers
recur id "ulu.topic.dot.**" --ext .cs -C 0

# Step 2: Find publishers
recur find "PublishAsync(DotControlTopics.OwnershipCreate" --scope "**" --ext .cs -C 1

# Step 3: Find subscribers
recur find "routingKey: DotControlTopics.OwnershipCreate" --scope "**" --ext .cs -C 1

# Step 4: Trace downstream from subscriber (back in function-call land)
recur trace "CreatePrivateLevelOwnershipAsync" --scope "**" --ext .cs --depth 1

# Step 5: Trace upstream pattern trigger
recur find "*.level.create" --scope "**" --ext .cs -C 3
```

Each step is correct but requires knowing what to look for. The knowledge of "PublishAsync
means producer" and "routingKey means consumer" lives in the developer's head, not in the tool.

---

## Proposed Solution

A new command `recur trace-id` that traces the **flow of a hierarchical identifier**
through a codebase, distinguishing definition, production, and consumption sites using
configurable heuristics.

### Basic Usage

```bash
recur trace-id "ulu.topic.dot.ownership.create" --ext .cs
```

Output:
```
ulu.topic.dot.ownership.create
├── DEFINED: DotControlEvents.cs:24 (const string OwnershipCreate)
├── PUBLISHED BY:
│   ├── DotWatcherHostedService.cs:296 — context: pattern "*.level.create"
│   └── DotWatcherHostedService.cs:326 — context: pattern "*.game.create"
└── CONSUMED BY:
    └── OwnershipCreateSubscriber.cs:92 — QueueBind(routingKey: ...)
        ├── calls: IContentOwnershipService.CreatePrivateLevelOwnershipAsync()
        └── calls: IContentOwnershipService.CreateGameOwnershipAsync()
```

### With glob patterns (trace multiple identifiers)

```bash
recur trace-id "ulu.topic.dot.**" --ext .cs
```

Output:
```
ulu.topic.dot.reward.process
├── DEFINED: DotControlEvents.cs:12
├── PUBLISHED BY:
│   ├── DotWatcherHostedService.cs:184 — pattern "*.answer.correct"
│   └── DotWatcherHostedService.cs:200 — pattern "*.win"
├── CONSUMED BY:
│   └── DotJobReceiverService.cs:106 — QueueBind
└── STATUS: ⚠️ DotWatcher disabled, chain dormant

ulu.topic.dot.analytics.update
├── DEFINED: DotControlEvents.cs:15
├── PUBLISHED BY:
│   └── DotWatcherHostedService.cs:215 — pattern "*.point.*"
├── CONSUMED BY: ❌ NONE (no subscriber)
└── STATUS: Topic defined, nobody listens

ulu.topic.dot.ownership.distribute
├── DEFINED: DotControlEvents.cs:18
├── PUBLISHED BY:
│   └── DotWatcherHostedService.cs:230 — pattern "*.publish.complete"
├── CONSUMED BY: ❌ NONE (no subscriber)
└── STATUS: Topic defined, nobody listens

ulu.topic.dot.admin.cleanup
├── DEFINED: DotControlEvents.cs:21
├── PUBLISHED BY:
│   └── DotWatcherHostedService.cs:266 — pattern "*.admin.cleanup.*"
├── CONSUMED BY: ❌ NONE (no subscriber)
└── STATUS: Topic defined, nobody listens

ulu.topic.dot.ownership.create
├── DEFINED: DotControlEvents.cs:24
├── PUBLISHED BY:
│   ├── DotWatcherHostedService.cs:296 — pattern "*.level.create"
│   └── DotWatcherHostedService.cs:326 — pattern "*.game.create"
└── CONSUMED BY:
    └── OwnershipCreateSubscriber.cs:92 — QueueBind
```

### JSON output (for merge composition)

```bash
# --json is implicit when piping (recur auto-detects non-terminal stdout)
recur trace-id "ulu.topic.dot.ownership.create" --ext .cs | Out-File lane-ids.json
```

---

## Heuristics Engine

The core of `trace-id` is a set of **directional heuristics** — patterns that identify
whether a code site is a definition, producer, or consumer of an identifier.

### Built-in heuristics (cover 80% of cases)

```toml
# .recur/trace-id.toml (or inline in config.toml)

[[heuristic]]
role = "define"
patterns = [
    'const string {ID}',
    'static readonly string {ID}',
    'public const string {ID}',
]

[[heuristic]]
role = "produce"
patterns = [
    'PublishAsync({ID}',
    '_messageBus.PublishAsync({ID}',
    'Publish({ID}',
    'SendAsync({ID}',
    'Emit({ID}',
]

[[heuristic]]
role = "consume"
patterns = [
    'QueueBindAsync(routingKey: {ID}',
    'Subscribe({ID}',
    'Bind({ID}',
    'On({ID}',
    'AddListener({ID}',
    'routingKey: {ID}',
    'RoutingKey => {ID}',
]

[[heuristic]]
role = "trigger"
patterns = [
    'Register("{GLOB}", .* {ID}',       # pattern registry → handler publishes
    '.Register("{GLOB}", async .* {ID}',
]
```

Where `{ID}` matches the const name or string literal being traced, and `{GLOB}` captures
the pattern string that triggers the identifier.

### Custom heuristics per project

Projects using different messaging patterns (MediatR, MassTransit, Azure Service Bus,
Kafka, etc.) can add their own heuristics:

```toml
# .recur/trace-id.toml (project-specific)

[[heuristic]]
role = "produce"
patterns = [
    'await _mediator.Publish(new {ID}',         # MediatR
    'await _bus.Publish<{ID}>(',                 # MassTransit
    'await _producer.ProduceAsync("{ID}"',       # Kafka
]

[[heuristic]]
role = "consume"
patterns = [
    'INotificationHandler<{ID}>',               # MediatR
    'IConsumer<{ID}>',                           # MassTransit
    '[Topic("{ID}")]',                           # Dapr
]
```

### Heuristic resolution order

1. Project-local `.recur/trace-id.toml` (highest priority)
2. User-global `~/.recur/trace-id.toml`
3. Built-in defaults (lowest priority)

---

## Composition with Existing Commands

### trace-id + trace (bridge sync and async)

```bash
# --json is implicit when piping (recur auto-detects non-terminal stdout)
# Lane 1: sync call graph up to the publish point
recur trace "WriteDotAsync" --scope "**" --ext .cs --depth 3 --direction callers \
  | Out-File lane-calls.json

# Lane 2: async identifier flow through message bus
recur trace-id "ulu.topic.dot.ownership.create" --ext .cs \
  | Out-File lane-ids.json

# Lane 3: sync call graph from subscriber onward
recur trace "CreatePrivateLevelOwnershipAsync" --scope "**" --ext .cs --depth 2 \
  | Out-File lane-downstream.json

# Merge: unified view
recur merge lane-calls.json lane-ids.json lane-downstream.json \
  --base "level.create.flow" \
  --edge-type call --edge-type route --edge-type call
```

### trace-id + flatten (config gates)

```bash
# Which identifiers are gated by config?
recur trace-id "ulu.topic.dot.**" --ext .cs --show-config-gates
# Output includes: DotWatcher.Enabled = false, RabbitMQ.Enabled = false

# Or pipe flatten into trace-id context
recur flatten appsettings.json --filter "DotWatcher" \
  | recur trace-id "ulu.topic.dot.**" --ext .cs --config-context --stdin
```

### trace-id + id (discovery mode)

```bash
# First: what hierarchical identifiers exist?
recur id "ulu.topic.**" --ext .cs -C 0
# → 5 topics found

# Then: trace each one's flow
recur trace-id "ulu.topic.**" --ext .cs
# → Full publisher/subscriber map for all 5 topics
```

### trace-id + trace-stats (complexity of async chains)

```bash
# Which async identifiers have the most complex downstream?
recur trace-id "ulu.topic.dot.**" --ext .cs \
  | recur trace-stats --stdin --sort-by risk
```

---

## merge --edge-type Extension

For merge to stitch different graph types, it needs to understand **edge types** — the
semantic equivalent of separators for call graphs:

| Edge Type | Meaning | Source Command |
|-----------|---------|---------------|
| `call` | Function A calls function B | `recur trace --json` |
| `route` | String identifier routes A to B | `recur trace-id --json` |
| `config` | Config value enables/disables B | `recur flatten --json` |

```bash
recur merge lane-calls.json lane-ids.json lane-config.json \
  --base "level.create.flow" \
  --edge-type call --edge-type route --edge-type config
```

**Join detection:** Merge stitches lanes where node names overlap:
- Call lane ends at `PublishAsync(DotControlTopics.OwnershipCreate)` ← references `OwnershipCreate`
- Route lane has `OwnershipCreate` as a defined identifier ← same name
- Route lane ends at subscriber calling `CreatePrivateLevelOwnershipAsync`
- Downstream call lane starts from `CreatePrivateLevelOwnershipAsync` ← same name

The JSON output from each command already includes function/identifier names. Merge
matches on these names across lanes, same as it matches filenames across directories today.

---

## Implementation Notes

### What already exists in recur

- `id` command: finds hierarchical identifiers by glob pattern in code content
- `trace`: outputs structured call graph with `root`, `children`, `path`, `line_number`
  (JSON is the default when piping — no `--json` flag needed in pipelines)
- `merge`: unifies trees from different lanes/separators
- `find`: scoped content search (the fallback for custom heuristics)
- `traversal_budget` trait: shared safety rail that breaks out of long-running traversals
  by default; `--force` flag continues past the budget. Already used by `trace`, `trace-stats`,
  `callees` — `trace-id` should reuse this same trait.
- Config system: `.recur/config.toml` with lanes, separators, status suffixes

### What trace-id needs to add

1. **Heuristic engine**: Pattern matching on code lines surrounding an identifier reference
   to classify each site as `define`, `produce`, `consume`, or `trigger`
2. **Config file**: `.recur/trace-id.toml` for project-specific heuristics
3. **JSON output**: Compatible with merge's expected tree structure, plus `edge_type` field
   (auto-JSON when piping, same as `trace`)
4. **Glob expansion**: `trace-id "ulu.topic.dot.**"` must expand to all matching identifiers
   (reuse `id` command's glob engine)
5. **Context extraction**: When a heuristic matches, capture surrounding lines for the
   "context" field (e.g., the pattern string in `Register("*.level.create", ...)`)
6. **`traversal_budget` integration**: Reuse the existing `traversal_budget` trait so
   `trace-id` on large codebases bails out safely by default. `--force` continues past budget.
   Budget should count identifier references scanned, not just depth.

### Estimated complexity

- Heuristic engine: ~300 lines Rust (pattern matching + TOML config parsing)
- JSON output: ~100 lines (extend existing trace JSON schema with `edge_type`)
- merge --edge-type: ~150 lines (extend merge to read `edge_type` from JSON nodes)
- Config loading: ~50 lines (reuse existing `RecurConfig` infrastructure)
- Total: ~600 lines, no new dependencies

### Testing strategy

- Unit tests: heuristic pattern matching (define/produce/consume classification)
- Integration tests: trace-id on the ulu-11 codebase as the canonical test case
  - 5 known topics in `DotControlEvents.cs`
  - 7 registered patterns in `DotWatcherHostedService.cs`
  - 3 subscribers (`OwnershipCreateSubscriber`, `DotJobReceiverService`, `GamePublishedSubscriber`)
  - 2 topics with subscribers, 3 topics without → tests both paths
- Snapshot tests: JSON output stability for merge compatibility
- The ulu-11 `users.dot` reactive chain is small enough to verify by hand but complex
  enough to exercise all heuristic roles

---

## Why This Matters Beyond ulu-11

Any architecture that uses **hierarchical string identifiers as connective tissue** has
this problem:

| Architecture Pattern | Identifier Format | Producers | Consumers |
|---------------------|-------------------|-----------|-----------|
| RabbitMQ topic exchange | `ulu.topic.dot.reward.process` | PublishAsync | QueueBind |
| Kafka topics | `events.user.signup` | ProduceAsync | ConsumerGroup |
| MediatR notifications | `UserCreatedNotification` | Publish | INotificationHandler |
| MassTransit messages | `SubmitOrder` | Publish | IConsumer |
| Azure Service Bus | `orders/completed` | SendAsync | ProcessMessageAsync |
| Event Grid | `Microsoft.Storage.BlobCreated` | EventGridPublisher | EventGridTrigger |
| gRPC service routing | `package.Service/Method` | Client.CallAsync | ServerServiceDefinition |
| ContentType discriminators | `ulu.question.multiplechoice` | component.ContentType = | switch(contentType) |
| Feature flags | `feature.dark-mode.enabled` | config write | config read |

The heuristic engine is **language-agnostic** — it's just pattern matching on text.
The same approach works for C#, TypeScript, Python, Go, Java, Rust. The built-in
heuristics cover the most common patterns; project-specific heuristics handle the rest.

**The gap recur fills:** Traditional static analysis tools (Roslyn, tree-sitter) can
trace function calls with perfect accuracy but can't follow string-mediated routing.
grep/rg can find string references but can't distinguish direction. `recur trace-id`
sits in the middle — text-based (like recur's philosophy) but directionally aware
(like a call graph). Good enough for 80% of cases, composable for the rest.

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-03-03 | Discovered gap during ulu-11 dot tracing | 5 manual commands to trace 1 async hop |
| 2026-03-03 | Proposed trace-id as new recur command | Bridges trace (sync) and id (strings) |
| 2026-03-03 | Designed merge --edge-type extension | Composable: call + route + config in one tree |
| 2026-03-03 | Chose heuristic engine over AST parsing | Stays text-based (recur philosophy), language-agnostic |
| 2026-03-03 | Proposed .recur/trace-id.toml config | Per-project heuristics for different messaging frameworks |
| 2026-03-03 | Identified "chain as hierarchy" workaround | Works today with zero features, but doesn't scale |

---

## Cross-References

- **Gap discovered:** `docs/users.dot.faq.md` → "The async gap (what recur can't do yet)"
- **Static analysis context:** `docs/agents/recur-agent.static-analysis.md` → "The Missing Feature"
- **merge composition:** `docs/agents/recur-agent.static-analysis.md` → "How trace-id Could Compose with merge"
- **Workaround (today):** `docs/agents/recur-agent.md` → "Async Flows as Hierarchy"
- **Recur source:** `C:\src\recur\src\` (Rust, trace/id/merge implementations)
- **Canonical test case:** ulu-11 `users.dot` reactive chain (5 topics, 7 patterns, 3 subscribers)
