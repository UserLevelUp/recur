# Recur: Static Analysis & Distributed File Systems

> Analysis of recur's current capabilities and future potential.
> See also: `recur tree "recur-agent" -d docs/agents/`
> Updated: 2026-03-03 (recur 0.2.6)

## What Recur Can Do Today (Static Analysis)

Recur already has **six commands** that perform static analysis:

### 1. `recur callers` — Who calls this function?

```bash
recur callers "GetLinkedOwnerAccountAsync" --scope "**" --ext .cs
# ? 14 call sites across 6 files (services, controllers, tests)

recur callers "GetLinkedOwnerAccountAsync" --scope "**" --ext .cs --count
# ? 14
```

**Useful for:** Finding all consumers of an API before changing it.
Impact analysis for Phase 3 renames.

### 2. `recur callees` — What does this function call?

```bash
recur callees "CreateGameOwnershipAsync" --scope "**" --ext .cs
```

**Useful for:** Understanding dependencies. If you're refactoring a function,
see what it depends on before you touch it.

### 3. `recur trace` — Multi-level call graph

```bash
recur trace "ApplyAiContent" --depth 2 --scope "LevelController.**" --ext .cs
recur trace "GetDeletedComponents" --direction callers --depth 2 --ext .cs
recur trace "ValidateInput" --direction both --depth 1 --ext .cs
```

**Disambiguation with `--pick`:** When a function name matches multiple definitions,
trace fails with a numbered list. Use `--pick N` to select:

```bash
recur trace "EmitWizard3LevelDotAsync" --scope "**" --ext .cs --depth 2 --direction callers
# Error: Multiple matches found:
#   1) EmitWizard3LevelDotAsync in Creation.cs:151
#   2) EmitWizard3LevelDotAsync in Creation.cs:350
#   3) EmitWizard3LevelDotAsync in Creation.cs:438
#   4) EmitWizard3LevelDotAsync in Main.cs:224
# Use --pick <N> to select a match.

recur trace "EmitWizard3LevelDotAsync" --scope "**" --ext .cs --depth 2 --direction callers --pick 1
# ? Shows caller tree for the first overload
```

**Additional flags:**
- `--format tree|flat|graph` — output format (default: tree; auto-switches to JSON when piping)
- `--max-width N` — limit branches per level (default: 10)
- `--verbose` — show full paths instead of abbreviated
- `--depth-guard clamp|hard-fail` — safety guardrail for deep traces
- `--force` — bypass `traversal_budget` (see below)
- `--scope-alias name=pattern` — shorthand for long scope patterns

**`traversal_budget` trait:** trace, trace-stats, callees, and other potentially
long-running commands use a `traversal_budget` that breaks out by default when
processing exceeds the budget. Use `--force` to continue past the budget limit.
This is a safety rail — not an error. If trace stops early, add `--force` to get
the full result.

**Useful for:** Visualizing execution paths. "Who calls X, and who calls those callers?"

### 4. `recur id` — Find hierarchical identifiers in code

```bash
recur id "ulu.role.**" --ext .cs -C 0
# ? Every place "ulu.role.creator", "ulu.role.owner", etc. appears in code

recur id "ulu.system.**" --ext .cs
# ? All system ContentType references
```

**This is recur's killer static analysis feature.** Hierarchical identifiers
(dot-separated strings like `ulu.role.owner`) are scattered across code as
string literals, config values, ContentType markers, etc. `recur id` finds
them all by pattern. Traditional grep can't do `ulu.role.**` (recursive glob
inside file content).

### 5. `recur flatten` — Structured file analysis

```bash
recur flatten appsettings.json --filter "ConnectionStrings"
# ? Flattens JSON to hierarchical dot-paths with values

recur flatten config.xml --max-depth 2
# ? XML ? path=value records

recur flatten data.yaml --format yaml              # YAML support
recur flatten levels.csv --format csv              # CSV support
cat pom.xml | recur flatten --stdin                # Pipe from stdin
```

**Useful for:** Config drift detection, comparing appsettings across environments.

### 6. `recur trace-stats` — Call graph complexity analysis (added in 0.2.x)

```bash
recur trace-stats --scope "**" --ext .cs --top 10
# ? Top 10 functions by transitive dependency count

recur trace-stats --scope "**" --ext .cs --filter circular-only
# ? Only functions involved in circular dependencies

recur trace-stats --scope "**" --ext .cs --sort-by risk
# ? Functions sorted by risk score (combines transitive deps, depth, circularity)

recur trace-stats --scope "**" --ext .cs --format csv > complexity.csv
# ? Export as CSV for spreadsheet analysis
```

**Sort options:** `transitive`, `direct`, `circular`, `depth`, `risk`
**Filter options:** `circular-only`, `high-risk`, `medium-risk`, `low-risk`

**Useful for:** Finding complexity hotspots before refactoring, detecting circular
dependencies, identifying high-risk functions that need test coverage.

## What Recur Cannot Do Today

| Capability | Status | Why |
|-----------|--------|-----|
| AST parsing (Roslyn, tree-sitter) | Not supported | `callers`/`callees` use text matching, not AST |
| Type resolution | Not supported | Can't distinguish `Foo.Bar()` from `Baz.Bar()` — use `--pick N` to disambiguate |
| Cross-project dependency graphs | Works well | `trace --scope "**"` searches all projects; tested across Controllers ? Services ? Tests |
| Dead code detection | Not supported | Would need call graph + entry point analysis |
| Complexity analysis | Supported | `trace-stats` provides transitive deps, circular deps, risk scoring |
| **Hierarchical identifier flow tracing** | **Not supported** | **`recur id` finds references; `recur trace` follows calls — nothing bridges them for async chains** |
| Remote/distributed file access | Not supported | All commands require local filesystem |
| Parallel multi-node search | Not supported | Single process, single machine |

### The Missing Feature: `recur trace-id` (Hierarchical Identifier Flow)

**Discovered gap:** During a real session tracing the `users.dot` reactive chain, the
most painful friction was tracing how a **hierarchical string identifier** flows through
an async architecture. `recur trace` follows function calls. `recur id` finds string
references. But the reactive chain is connected by **strings, not function calls:**

```
dot suffix "level.create"                         ? emitted in controller (string)
  ? DotPatternRegistry matches "*.level.create"   ? pattern in DotWatcher (string)
    ? PublishAsync(DotControlTopics.OwnershipCreate)  ? publishes to topic (const string)
      ? OwnershipCreateSubscriber binds to topic  ? subscribes via queue binding (same const)
        ? ownershipService.CreatePrivate...()     ? finally, a function call trace can pick up
```

Each hop is mediated by a hierarchical string identifier (`ulu.topic.dot.ownership.create`).
`recur trace` can't follow these because they're runtime-routed, not compile-time-linked.
`recur id` can find every reference but doesn't know which is a **producer** vs **consumer**.

**What was required instead (5 manual steps):**
```bash
# Step 1: Find all topic identifiers
recur id "ulu.topic.dot.**" --ext .cs -C 0

# Step 2: For each topic, find who publishes to it
recur find "PublishAsync(DotControlTopics.OwnershipCreate" --scope "**" --ext .cs -C 1

# Step 3: Find who subscribes (binds queue)
recur find "routingKey: DotControlTopics.OwnershipCreate" --scope "**" --ext .cs -C 1

# Step 4: Trace what the subscriber calls (now we're back in function-call land)
recur trace "CreatePrivateLevelOwnershipAsync" --scope "**" --ext .cs --depth 1 --direction callers

# Step 5: Trace upstream — what pattern triggered the publish?
recur find "*.level.create" --scope "**" --ext .cs -C 3
```

**What a hypothetical `recur trace-id` could do (1 command):**
```bash
recur trace-id "ulu.topic.dot.ownership.create" --ext .cs --depth 2
# Output:
# ulu.topic.dot.ownership.create
# ??? DEFINED: DotControlEvents.cs:24 (const string OwnershipCreate)
# ??? PUBLISHED BY:
# ?   ??? DotWatcherHostedService.cs:296 — pattern "*.level.create" ? PublishAsync(...)
# ?   ??? DotWatcherHostedService.cs:326 — pattern "*.game.create" ? PublishAsync(...)
# ??? CONSUMED BY:
#     ??? OwnershipCreateSubscriber.cs:92 — QueueBind(routingKey: ...)
#         ??? calls: IContentOwnershipService.CreatePrivateLevelOwnershipAsync()
#         ??? calls: IContentOwnershipService.CreateGameOwnershipAsync()
```

**Why this matters for hierarchical systems:**

This gap only shows up in architectures that use **hierarchical string identifiers as
the connective tissue** between components — which is exactly what `users.dot`, RabbitMQ
topic exchanges, and event-driven systems do. The hierarchy is there in the code as
string literals. Recur already understands hierarchical naming. The missing piece is
understanding **directionality** — who produces vs who consumes a given identifier.

**Heuristics that could power this:**
- `PublishAsync(X)`, `_messageBus.PublishAsync(X)` ? PRODUCER of X
- `QueueBindAsync(routingKey: X)`, `Subscribe(X)` ? CONSUMER of X  
- `const string X = "..."` ? DEFINITION of X
- `Register("pattern", handler)` where handler calls PublishAsync(X) ? PATTERN TRIGGER for X
- These patterns are language-specific but stable within a codebase

**The insight:** `recur trace` follows the **call graph** (synchronous). `recur trace-id`
would follow the **identifier graph** (asynchronous). Together they'd cover both halves
of a distributed system's architecture — the parts connected by function calls AND the
parts connected by hierarchical string routing.

### How trace-id Could Compose with merge

**The reactive chain has three distinct graph types, each with its own "separator" — not
a literal character separator, but an edge type that describes HOW nodes connect:**

```
EDGE TYPE        WHAT IT MEANS                    RECUR COMMAND TODAY
?????????        ?????????????                    ??????????????????
call ?           function A calls function B      recur trace
route ~~>        string identifier routes A to B  recur id (finds refs, no directionality)
config ==>       config value enables/disables B  recur flatten (--filter)
```

> **Note:** `--json` is implicit when piping between recur commands. Output auto-switches
> to JSON when stdout is not a terminal. No need to specify it in pipelines.

**The merge insight: these are just lanes with different separators.**

Recur merge already unifies trees from different directories with different separators
(`.` for docs, `_` for Rust source). The same pattern works for different **graph types**
— each is a lane, each has its own edge semantics, merge stitches them at shared nodes:

```bash
# Lane 1: synchronous call graph (who calls WriteDotAsync?)
# --json is implicit when piping — recur auto-detects non-terminal stdout
recur trace "WriteDotAsync" --scope "**" --ext .cs --depth 3 --direction callers \
  | Out-File lane-calls.json

# Lane 2: identifier flow (where does ulu.topic.dot.ownership.create appear?)
recur trace-id "ulu.topic.dot.ownership.create" --ext .cs \
  | Out-File lane-ids.json

# Lane 3: downstream call graph (what does the subscriber do after receiving?)
recur trace "CreatePrivateLevelOwnershipAsync" --scope "**" --ext .cs --depth 2 --direction callees \
  | Out-File lane-downstream.json

# Merge: unified view across all edge types
recur merge lane-calls.json lane-ids.json lane-downstream.json \
  --base "level.create.flow" \
  --edge-type call --edge-type route --edge-type call
```

**Hypothetical merged output:**
```
level.create.flow
??? [call] CreateWizard3Level (Creation.cs:151)
?   ??? [call] EmitWizard3LevelDotAsync (Creation.cs:438)
?       ??? [call] WriteDotAsync (SemanticDotService.cs:47)
?           ??? [call] InsertOneAsync ? MongoDB users.dot
??? [route] DotWatcher pattern "*.level.create" (DotWatcherHostedService.cs:271)
?   ??? [route] PublishAsync ? ulu.topic.dot.ownership.create
??? [route:define] DotControlTopics.OwnershipCreate (DotControlEvents.cs:24)
??? [route:subscribe] OwnershipCreateSubscriber (OwnershipCreateSubscriber.cs:92)
?   ??? [call] CreatePrivateLevelOwnershipAsync (ContentOwnershipService.cs)
?       ??? [call] InsertOneAsync ? MongoDB users.contentOwnership
??? [config] DotWatcher.Enabled = false (appsettings.json)
```

**The join points are where lanes overlap:**
- Lane 1 (calls) ends at `PublishAsync(DotControlTopics.OwnershipCreate)`
- Lane 2 (identifiers) picks up from `OwnershipCreate` constant
- Lane 2 ends at `OwnershipCreateSubscriber` calling `CreatePrivateLevelOwnershipAsync`
- Lane 3 (calls) picks up from `CreatePrivateLevelOwnershipAsync` going deeper

**Merge detects joins by matching node names across lanes** — same as it does today for
file hierarchies where `UserService.Game.Load` in the code lane matches
`UserService.Game.Load.Tests` in the test lane.

### The "something else": the chain IS a hierarchy

There's an even more recur-native approach that requires **zero new commands.**

The reactive chain can be expressed as a hierarchical naming convention — making ALL
existing recur tools work on it without modification:

```
level.create.flow.emit.controller.CreateWizard3Level
level.create.flow.emit.helper.EmitWizard3LevelDotAsync
level.create.flow.emit.service.WriteDotAsync
level.create.flow.emit.mongo.users.dot.insert
level.create.flow.watch.pattern.*.level.create
level.create.flow.route.topic.ulu.topic.dot.ownership.create.publish
level.create.flow.route.topic.ulu.topic.dot.ownership.create.subscribe
level.create.flow.execute.subscriber.OwnershipCreateSubscriber
level.create.flow.execute.service.CreatePrivateLevelOwnershipAsync
level.create.flow.execute.mongo.users.contentOwnership.insert
```

**If these names exist as files (even empty `.md` markers), then:**
```bash
recur tree "level.create.flow" -d docs/flows/
# Shows the entire reactive chain as a hierarchy

recur files "**route**" -d docs/flows/
# Every async routing hop across all flows

recur files "**.subscribe" -d docs/flows/
# Every subscriber endpoint

recur merge lane-emit.json lane-route.json lane-execute.json \
  --base "level.create.flow"
# Unified view from three phases
```

**This is the recur philosophy taken to its conclusion:** The naming convention IS the
interface. If you can express something as `prefix.base.suffix`, every recur tool
works on it — `tree`, `files`, `find`, `merge`, `stats`, `id`. No new command needed.

**The tradeoff:**
- `trace-id` (new command) ? auto-discovers the flow from code, zero manual setup
- Hierarchy-as-files (existing tools) ? manual setup but works TODAY, any tool can query it
- `trace + trace-id + merge` (composition) ? most powerful, bridges sync + async + config

**Recommendation:** Start with hierarchy-as-files for the flows you already understand
(like `level.create.flow`). When the pattern proves itself, build `trace-id` to
auto-generate those hierarchies from code. Then merge becomes the unified view across
all three graph types — call, route, and config.

## How Recur Could Work with Distributed File Systems

### Pattern 1: Federated stdin (works TODAY)

Recur's `--stdin` flag already enables distributed-style workflows. The files
don't need to be local — you just need to get their **paths** into stdin:

```bash
# Mount a network share and search it
recur files "**.current" -d /mnt/shared-docs/

# SSH + pipe (files on remote, recur runs locally on streamed content)
ssh prod-server "find /app/docs -name '*.md'" | recur files "**.todo" --stdin

# Azure Blob ? local index ? recur
az storage blob list --container docs --query "[].name" -o tsv | recur files "**Phase**" --stdin

# S3 listing ? recur
aws s3 ls s3://my-bucket/docs/ --recursive | awk '{print $4}' | recur files "**.current" --stdin
```

**Key insight:** Recur doesn't need to read file **content** for `files`, `tree`,
`related`, `children`, `stats`, or `merge`. It only needs **paths**. Paths are
small, easily piped from any source.

### Pattern 2: Content-Aware Commands Need Local Files

`find`, `callers`, `callees`, `trace`, `id`, and `flatten` need to read file
content. For distributed systems, you'd need to either:

1. **Mount the remote filesystem** (NFS, CIFS, FUSE, Azure Files)
2. **Sync a subset locally** (rsync, azcopy, git sparse-checkout)
3. **Stream content through stdin** (future: `--stdin-content`)

### Pattern 3: Multi-Node Merge (hypothetical)

Imagine running recur on N nodes, each scanning their local shard, then merging:

```bash
# Node 1 scans shard A
recur tree "ulu" -d /shard-a/ --json > /tmp/shard-a.json

# Node 2 scans shard B
recur tree "ulu" -d /shard-b/ --json > /tmp/shard-b.json

# Coordinator merges
recur merge /tmp/shard-a.json /tmp/shard-b.json --base "ulu"
```

**`recur merge` already exists** for combining results from different separator
conventions. It could be extended to merge results from different nodes/shards.

### Pattern 4: Git as Distributed FS (works TODAY)

Git is already a distributed file system. Recur + git is powerful:

```bash
# What changed on this branch, viewed hierarchically?
git diff --name-only origin/main...HEAD | recur files "**" --stdin

# Search for TODOs only in files changed in this PR
git diff --name-only origin/main...HEAD | recur find "TODO" --scope "**" --stdin

# Find callers of a function, but only in files you touched
git diff --name-only | recur callers "GetLinkedOwnerAccountAsync" --scope "**" --stdin

# Compare branch hierarchies
git diff --name-only origin/main...HEAD | recur tree "User Level Up Services" --stdin
```

### Pattern 5: Container/K8s Log Analysis (hypothetical)

If logs follow hierarchical naming (they should — `ulu.role.owner`,
`ulu.game.publish.complete`, etc.), recur could analyze them:

```bash
# Stream K8s logs, find hierarchical events
kubectl logs deployment/ulu-web --since=1h | recur id "ulu.game.publish.**"

# Flatten structured log JSON
kubectl logs deployment/ulu-web --since=1h -o json | recur flatten --stdin --filter "ulu.game"
```

## Static Analysis Recipes (work TODAY)

### Recipe: Trace an event emission graph (users.dot example)

```bash
# 1. Find all emit points by method name
recur callers "WriteDotAsync" --scope "**" --ext .cs --count       # ? 17
recur callers "WriteSystemDotAsync" --scope "**" --ext .cs --count # ? 16
recur callers "WriteCreatorDotAsync" --scope "**" --ext .cs --count # ? 3

# 2. Trace upstream: user action ? emit helper ? MongoDB write
recur trace "WriteDotAsync" --scope "**" -d . --ext .cs --depth 3 --direction both
# Shows: EmitPublishDotAsync, EmitWizard3LevelDotAsync, EmitGameLoadDotAsync

# 3. Disambiguate overloads with --pick
recur trace "EmitWizard3LevelDotAsync" --scope "**" --ext .cs --depth 2 --direction callers --pick 1
# Shows: CreateWizard3Level ? CreateWizard3Game ? EmitWizard3LevelDotAsync

# 4. Find DI injection map (who CAN emit)
recur find "ISemanticDotService" --scope "**" -d . --ext .cs
# ? AdminController, DotController, LevelController, GameController.Load, PlayerHistoryCache

# 5. Verify the gap (who CANNOT emit)
recur find "_dotService" --scope "**" -d . --ext .cs
# Missing: HierarchyOperations, DeletedItems, Templates, Tab.Ai ? confirms 11 unwired operations
```

**Why this works:** The dot suffix IS hierarchical (`level.create`, `publish.complete`,
`ulu.game.load`). The same naming convention that makes dots queryable in MongoDB
makes them traceable in the codebase via recur. See `docs/users.dot.faq.md` for the
full emission graph discovered this way.

### Recipe: Find all scattered string constants for a namespace

```bash
# Where does "ulu.role.*" appear across the entire codebase?
recur id "ulu.role.**" --ext .cs | Select-String "^\." | ForEach-Object { ($_ -split ':')[0] } | Group-Object | Sort-Object Count -Descending
```

Result: instant heatmap of which files reference roles most heavily.

### Recipe: Impact analysis before a rename

```bash
# Before renaming "contentOwnership" ? "users.contentOwnership"
recur find "contentOwnership" --scope "**" -C 0
recur find "contentOwnership" --scope "**" --ext .cs --count   # C# files
recur find "contentOwnership" --scope "**" --ext .jl --count   # Julia files
recur find "contentOwnership" --scope "**" --ext .js --count   # Mongo init scripts
```

### Recipe: Cross-lane coverage check

```bash
# Does every service have tests, docs, and julia scripts?
recur files "**Service**" -d "User Level Up Services/" --count        # Code
recur files "**Service**" -d "User_Level_Up_Tests_Data_Mongo/" --count # Tests
recur files "**Service**" -d docs/ --count                             # Docs
recur find "Service" --scope "**" -d jl/ -C 0                         # Julia refs
```

### Recipe: Triple-lookup detection (from Phase 2 analysis)

```bash
# How many files call GetLinkedOwnerAccountAsync?
recur callers "GetLinkedOwnerAccountAsync" --scope "**" --ext .cs --count
# ? 14 (including 3 in the publish flow — the triple lookup we found)
```

## Strengths vs. Traditional Static Analysis

| Aspect | Recur | Roslyn/tree-sitter | grep/rg |
|--------|-------|-------------------|---------|
| Hierarchical identifier search | Best (`recur id "ulu.**"`) | Overkill | Can't glob inside content |
| Call graph | Text-based (good enough for 80%) | AST-accurate (100%) | Manual |
| Disambiguation | `--pick N` for overloads | Automatic (AST) | Manual |
| Complexity analysis | `trace-stats` (risk, circular) | Full (cyclomatic, etc.) | None |
| Cross-language | Yes (any text file) | Per-language | Yes |
| Config file analysis | `flatten` + `id` | No | Manual |
| Piping/composability | First-class (`--stdin`, `--json`) | Limited | Good |
| Distributed/remote | Via stdin pipes | Local only | Via pipes |
| Setup cost | Zero (single binary) | Heavy (SDK + analyzers) | Zero |

**Recur's sweet spot:** Hierarchical identifier tracking, cross-lane gap analysis,
impact analysis for renames, and composable piping. It's not trying to replace
Roslyn — it's the tool you reach for when you need to answer "where does
`ulu.role.owner` appear across C#, JSON, Julia, and Markdown?"

## `recur-git` — Git/Workflow Extension

`recur-git` is a **separate binary** that adds git-aware checkpoint semantics
on top of recur's pure hierarchy model. Run `recur-git --help` for full docs.

### Checkpoint Snapshot

Captures current git + lane state in one shot:

```bash
recur-git checkpoint --snapshot
```

Output:
```
== Checkpoint Snapshot ==
git.branch: copilot/update-ownership-model-logic
git.head: c9642bcc Phase 2: Add comprehensive integration tests - all passing
git.worktree: dirty=12
lane.state.docs.current: none
lane.state.src.current: none
lane.separator.docs_tests: .
lane.separator.src: _
```

### Parallel-Lane Checkpoints

Emit or append a checkpoint entry to a log file:

```bash
# Emit to stdout
recur-git checkpoint --emit-parallel --checkpoint-id ck-phase2-eventness

# Append to a log file
recur-git checkpoint --append-parallel --checkpoint-id ck-phase2-eventness -f docs/checkpoints.md
```

Each entry captures:
- `date` — unix timestamp
- `lane.git.branch` — current branch
- `lane.git.head` — commit hash + message
- `lane.git.worktree` — dirty file count
- `lane.separator.*` — detected separators
- `evidence.*_tree_cmd` — recur commands to reproduce the lane view

### Checkpoint with Tests

```bash
recur-git checkpoint --snapshot --run-tests          # Runs cargo test
recur-git checkpoint --snapshot --run-julia-tests     # Runs julia runtests.jl
```

### Using Checkpoints for Distributed Workflows

Checkpoints create a **temporal audit trail** in `docs/checkpoints.md`:

```bash
# Before starting Phase 3
recur-git checkpoint --append-parallel --checkpoint-id ck-phase3-start -f docs/checkpoints.md

# After completing Phase 3
recur-git checkpoint --append-parallel --checkpoint-id ck-phase3-done -f docs/checkpoints.md

# View checkpoint history
cat docs/checkpoints.md
```

**Key insight:** `recur-git` checkpoints bridge the gap between git's commit
history and recur's eventness pattern. Git tells you *what changed in code*.
Recur eventness tells you *what work was active*. Checkpoints capture both
at the same moment — perfect for handoffs between sessions or agents.

### Current Limitation: Hardcoded Patterns in `recur-git`

`recur-git checkpoint` **ignores** `.recur/config.toml` and hardcodes:

```rust
// src/recur_git_main.rs line 116
find_files_by_pattern(Path::new("docs"), "main.command.**.todo.current", '.')
```

But the config already defines the right abstractions:

```toml
# .recur/config.toml (ulu-11 project)
[checkpoint]
file = ".recur/checkpoints.md"
root_pattern = "**"            # ? recur-git should use this, not "main.command.**"

[status]
current_suffix = ".current.md" # ? recur-git should use this
todo_suffix = ".todo.md"
complete_suffix = ".complete.md"
```

**The fix:** `recur_git_main.rs` should read `RecurConfig` from `.recur/config.toml`
and use `[status].current_suffix` + `[checkpoint].root_pattern` + per-lane `dir`/`sep`
instead of hardcoding `"main.command.**.todo.current"` and `Path::new("docs")`.

What it should do (pseudocode):
```rust
let config = RecurConfig::load()?;
let suffix = config.status.current_suffix.unwrap_or(".current.md");
let pattern = config.checkpoint.root_pattern.unwrap_or("**");

for lane in &config.lanes {
    let current_files = find_files_by_pattern(
        Path::new(&lane.dir),
        &format!("{pattern}.{suffix}"),  // "**.current.md" not "main.command.**.todo.current"
        lane.sep,
    )?;
    println!("lane.state.{}.current: {}", lane.name, format_paths(current_files));
}
```

**Workaround until fixed:** Pair checkpoint with explicit recur query:
```bash
recur-git checkpoint --snapshot
recur files "**.current" -d docs/   # ? this finds ALL .current files
```

## Cross-Lane
- Parent: `docs/agents/recur-agent.md`
- Recur source: `C:\src\recur\src\recur_git_main.rs` (hardcoded patterns)
- Config parser: `C:\src\recur\src\project_config.rs` (`RecurConfig`, `StatusConfig`, `CheckpointConfig`)
- Project config: `.recur/config.toml` (already has `[status]` and `[checkpoint]` sections)
- Recur help: `recur --help`, `recur <command> --help`
- Recur-git help: `recur-git --help`, `recur-git checkpoint --help`
- Checkpoint log: `.recur/checkpoints.md`
