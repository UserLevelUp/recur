# Recur Assert: Eventness-Driven Code Assertions via Hierarchical Query Results

**Status:** rough draft
**Origin:** User Level Up session — 5-phase bug fix (Phases A–E) using recur for discovery, tracking, and verification
**Branch:** `copilot/optimize-section-loading`

---

## The Problem

Developer tools fall into two camps:

1. **Predictive tools** (tests, linters) — define exact inputs and expected outputs before the code runs
2. **Reactive tools** (logs, debuggers) — observe what happened after the code ran

Neither handles the middle ground: **asserting properties of the codebase as it exists right now**, without predicting how it got there or what the exact output looks like.

When a 4-bug fix spans 5 phases across C#, JavaScript, and CSHTML files — with localStorage caching, event-driven invalidation, and server-side rendering changes — no single test predicts the fix. No single log captures the state. The developer needs to **query the codebase hierarchy and assert properties of the results**.

---

## The Insight

Recur already queries hierarchy. Today it queries **file hierarchy** (dot-separated filenames, directory lanes). The planned extension queries **text hierarchy** (indentation, braces, regions, markdown headings within files).

An assertion is not a new feature. It's a **recur command whose result you check**.

```
predictable:    the recur command (the question you ask)
unpredictable:  the exact result (eventness — whatever the codebase looks like now)
assertable:     a property of the result (contains X, not-contains Y, count > 0)
```

This mirrors the `users.dot` event store philosophy:

```
predictable:    the MongoDB query
unpredictable:  the exact dots (user took whatever path they took)
assertable:     a property of the result (has publish.complete, no error dots)
```

The assertion doesn't model the event chain. It doesn't predict what happened. It checks: **is this property true right now?**

---

## Recur Today (File-Level Hierarchy)

Recur discovers and navigates hierarchy through filenames:

```sh
# Discovery: what's active?
recur files "**.current" -d docs/

# Children: what phases exist?
recur children "CreateWizard3.Tab.PlayerView.SectionLoad" -d docs/
# → PhaseA.complete.md, PhaseB.complete.md, ..., PhaseE.complete.md, reference.md

# Tree: visual hierarchy
recur tree "CreateWizard3.Tab.PlayerView.SectionLoad" -d docs/

# Cross-file search with context
recur find "GetSectionsByGameIdAsync" --scope "**" --ext .cs -C 3

# Cross-file search scoped to directory
recur find "cw3:component.saved" --scope "**" -d "User Level Up/wwwroot/js/" --ext .js -C 1
```

**What this already enables:**
- Workflow state tracking (`.todo.md` → `.current.md` → `.complete.md`)
- Cross-lane discovery (`recur files "**.Publish.Mongo.**" -d .` finds docs, scripts, and code)
- Scoped text search with context lines

**What's missing:**
- Hierarchy within files (text-level)
- Formal assertion on command results

---

## Recur Tomorrow (Text-Level Hierarchy)

The same commands, applied inward — from filesystem into file contents:

```sh
# Children of a method (what does LoadSection call?)
recur children "LoadSection" -f LevelController.CreateWizard3.Tab.PlayerView.cs
# → GetSectionsByGameIdAsync, GetComponentsBySectionIdAsync, ...

# Children of a different method (what does NextSection call?)
recur children "NextSection" -f LevelController.CreateWizard3.Tab.PlayerView.cs
# → GetGameComponentsByGameIdAsync  ← the old expensive pattern

# Children within a JS module
recur children "loadSection" -f CreateWizard3.Tab.PlayerView.Lazy.js
# → PlayerViewCache.getSection, PlayerViewCache.setSection, ...

# Children within markdown (heading hierarchy)
recur children "## Caching (Phase C)" -f PhaseE.complete.md
# → [x] Load section A..., [x] AutoSave..., [x] Media section cached...
```

**The key principle:** recur's separator convention (dots in filenames) extends to text hierarchy (indentation, braces, headings). Same mental model. Same commands. New depth.

---

## Assertions as Recur Command Sets

An assertion is a recur command plus an expected property of its result. Not the exact result — a **property**.

### Example: Bug #1 Guard (section not in its own children)

```sh
# The command (predictable — we know what to ask)
recur children "LoadSection" -f Tab.PlayerView.cs

# The assertion (property of result)
# PASS: contains "GetSectionsByGameIdAsync"
# PASS: contains "GetComponentsBySectionIdAsync"
# PASS: not-contains "GetGameComponentsByGameIdAsync"
```

If someone reintroduces the old pattern, the assertion fails. The recur command didn't change. The result changed. The property no longer holds.

### Example: Phase completion guard

```sh
# The command
recur children "CreateWizard3.Tab.PlayerView.SectionLoad" -d docs/

# The assertion
# PASS: all results match "*.complete.md" or "*.reference.md"
# FAIL: any result matches "*.todo.md" or "*.current.md"
```

### Example: Cache invalidation chain completeness

```sh
# The command
recur find "setGameVersion" --scope "**" --ext .js

# The assertion
# PASS: count >= 3 (autosave.js, tab.ai.js, tab.playerview.js)
# PASS: contains "createwizard3.autosave.js"
```

If someone adds a new save path but forgets to call `setGameVersion`, the count drops. Assertion fails.

### Example: Event emission/listener balance

```sh
# Emitters
recur find "dispatchEvent.*cw3:component.saved" --scope "**" --ext .js
# → expect count >= 3

# Listeners
recur find "addEventListener.*cw3:component.saved" --scope "**" --ext .js
# → expect count >= 1
```

---

## What This Is Not

**Not a test framework.** Tests predict inputs → outputs for runtime behavior. Assertions check static properties of the codebase. Tests run your code. Assertions query your code.

**Not a linter.** Linters enforce style rules defined by the tool author. Assertions enforce invariants defined by the developer, in the developer's own language (recur commands).

**Not AI.** No LLM, no GPU, no network. Recur commands are deterministic. Results are deterministic. Property checks are deterministic. Runs on a Raspberry Pi.

**Not MCP.** No server, no protocol, no cloud dependency. Single binary. Reads files. Returns results.

---

## The Philosophy

The assertion is the recur command set. You predict the questions. You can't predict the answers entirely. You assert that the answers satisfy properties.

This is eventness applied to code:
- The **codebase** is the event stream
- The **recur command** is the query
- The **result** is whatever happened (refactors, renames, new methods — eventness)
- The **assertion** checks: does this property hold?

You don't care what the eventness looks like that got you there. You care that the property holds right now.

Same as `users.dot`:
- The **dot collection** is the event stream
- The **MongoDB query** is the question
- The **dots** are whatever the user did (eventness)
- The **assertion** checks: does `publish.complete` exist?

Same tool. Same philosophy. Different depth.

---

## Implementation Path

### Phase 1: Text-level hierarchy (prerequisite)

Extend recur to parse hierarchy within files using indentation, braces, or language-aware markers:

- **Markdown:** `#` / `##` / `###` headings, `- [ ]` list items
- **C#:** `namespace` / `class` / `method` / `#region` nesting
- **JavaScript:** `function` / object literal nesting, IIFE scopes
- **Generic:** indentation-based fallback

This enables `recur children "LoadSection" -f file.cs` to return method-level children.

**Approach options:**
- **Indentation-based** (simplest, works for most languages, no dependencies)
- **Marker-based** (recur-specific comments like `// recur:begin LoadSection`)
- **Tree-sitter** (AST-aware, multi-language, +2MB binary size, zero runtime deps)

**Recommendation:** Start with indentation + language markers. Add tree-sitter later if demand warrants the binary size increase.

### Phase 2: Assert runner

A thin wrapper that runs recur commands and checks result properties:

```sh
# Run all assertions in a file
recur assert -f docs/assertions/PlayerView.SectionLoad.assert.txt

# The file is just recur commands + property checks:
# recur children "LoadSection" -f Tab.PlayerView.cs | contains "GetSectionsByGameIdAsync"
# recur children "LoadSection" -f Tab.PlayerView.cs | not-contains "GetGameComponentsByGameIdAsync"
# recur children "SectionLoad" -d docs/ | all-match "*.complete.md|*.reference.md"
```

Property operators:
- `contains "X"` — result includes line matching X
- `not-contains "X"` — result has no line matching X
- `count >= N` — result has at least N lines
- `count == 0` — result is empty
- `all-match "pattern"` — every line matches glob/regex

**Exit codes:** 0 = all pass, 1 = any fail (CI-friendly).

### Phase 3: CI integration

```yaml
# GitHub Actions
- name: Recur assertions
  run: recur assert -d docs/assertions/
```

Assertions live alongside the code they guard. Bug fix → write assertion → assertion runs in CI forever. The `.complete.md` file documents the fix. The `.assert.txt` file guards against regression. Same concern, same hierarchy, different eventness suffix.

---

## Naming Convention

Following recur's hierarchical naming:

```
docs/assertions/
    CreateWizard3.Tab.PlayerView.SectionLoad.assert.txt     ← guards the Phase A–E fix
    CreateWizard3.Tab.PlayerView.Cache.invalidation.assert.txt
    CreateWizard3.Tab.Hierarchy.SectionScoped.assert.txt
```

Discoverable via:

```sh
recur files "**.assert.*" -d docs/
recur children "CreateWizard3.Tab.PlayerView" -d docs/assertions/
```

---

## Real-World Example: This Session

The 5-phase section loading fix we just completed would produce this assertion file:

```
# CreateWizard3.Tab.PlayerView.SectionLoad.assert.txt
# Guards: Phases A–E (section-scoped loading, caching, media, UI state)

# Phase A: LoadSection uses scoped queries, not full game fetch
recur find "GetSectionsByGameIdAsync" -f Tab.PlayerView.cs | count >= 1
recur find "GetComponentsBySectionIdAsync" -f Tab.PlayerView.cs | count >= 1
recur find "GetGameComponentsByGameIdAsync" -f Tab.PlayerView.cs -C 0 | not-contains "LoadSection"

# Phase B: Media map populated in skeleton build
recur find "PlayerViewConfig.media" -f LoadHierarchy.cs | count >= 1

# Phase C: Cache wired in Lazy.js
recur find "PlayerViewCache.getSection" -f Tab.PlayerView.Lazy.js | count >= 1
recur find "PlayerViewCache.setSection" -f Tab.PlayerView.Lazy.js | count >= 1
recur find "PlayerViewCache.invalidateSection" -f Tab.PlayerView.Lazy.js | count >= 1

# Phase D: Console breadcrumbs exist
recur find "Mode: editMode" -f Tab.PlayerView.Lazy.js | count >= 1
recur find "Mode: showMediaSections" -f Tab.PlayerView.Lazy.js | count >= 1
recur find "showCacheIndicator" -f Tab.PlayerView.Lazy.js | count >= 2

# Phase E: All phases complete (no lingering todos)
recur children "CreateWizard3.Tab.PlayerView.SectionLoad" -d docs/ | not-contains ".todo.md"
recur children "CreateWizard3.Tab.PlayerView.SectionLoad" -d docs/ | not-contains ".current.md"

# Cross-cutting: Cache version chain complete
recur find "setGameVersion" --scope "**" --ext .js | count >= 3
recur find "cw3:component.saved" --scope "**" --ext .js | count >= 4
```

**20 minutes to fix. 5 minutes to write assertions. Guards the fix forever. Zero GPU.**

---

## Recur + Git: Hierarchy Already Exists

Git is hierarchy. Recur speaks hierarchy. The integration isn't adding git support to recur — it's recognizing that git's native structures ARE the hierarchies recur already knows how to walk.

**`recur-git` already exists.** It works with git commands via stdin/stdout, following recur's composability principle. The pieces are in place — what follows is how they connect to assertions, context scoping, and the four-depth model.

### Three Depths of Hierarchy

```
DEPTH 1: Filesystem         recur children "CreateWizard3.Tab.PlayerView" -d docs/
         (today)            → PhaseA.complete.md, PhaseB.complete.md, ...

DEPTH 2: Within files       recur children "LoadSection" -f Tab.PlayerView.cs
         (planned)          → GetSectionsByGameIdAsync, GetComponentsBySectionIdAsync, ...

DEPTH 3: Across time        recur-git | recur children "LoadSection" --diff HEAD~5..HEAD
         (recur-git)        → what changed in LoadSection across last 5 commits
```

Depth 3 is where git enters. Not as a separate tool — as a composable pipe. `recur-git` produces git hierarchy on stdout. `recur` consumes it through stdin. Same separation, same unix philosophy.

### Git Structures That Are Already Hierarchy (via recur-git)

`recur-git` bridges git's output into recur's hierarchical model via stdin/stdout pipes. This follows recur's lane design principle — git is just another lane with its own separator conventions.

**Branches are hierarchy.** `copilot/optimize-section-loading` has a separator (`/`). Recur already handles mixed separators via merge. `recur-git` can emit branch names as hierarchical input:

```sh
# recur-git emits branch hierarchy, recur queries it
recur-git branches | recur children "copilot"
# → optimize-section-loading, fix-media-cache, add-assert-runner

# Which branches touched PlayerView files?
recur-git changed-files copilot/optimize-section-loading | recur find "Tab.PlayerView"
# → LevelController.CreateWizard3.Tab.PlayerView.cs
# → LevelController.CreateWizard3.Tab.PlayerView.LazyLoad.cs
```

**Commits are hierarchy.** Each commit touches files. `recur-git` can emit commit → file mappings that recur walks as hierarchy:

```sh
# What concerns were touched in the last 10 commits?
recur-git log -n 10 | recur tree
# → CreateWizard3
#     Tab.PlayerView (3 commits)
#       SectionLoad (2 commits: PhaseA, PhaseB)
#       LazyLoad (1 commit: cache wiring)
#     Tab.Hierarchy (1 commit)
#   docs (5 commits: phase tracking)
```

**Diffs are hierarchy.** A diff has files → hunks → lines. `recur-git` emits diff structure, recur queries it:

```sh
# What methods changed between this branch and main?
recur-git diff main..HEAD | recur children --depth method
# → LoadSection (Tab.PlayerView.cs)
# → BuildSectionContentViewModel (LoadHierarchy.cs)
# → loadSection (Tab.PlayerView.Lazy.js)

# What changed inside LoadSection specifically?
recur-git diff main..HEAD -f Tab.PlayerView.cs | recur children "LoadSection"
# → +GetSectionsByGameIdAsync (added)
# → +GetComponentsBySectionIdAsync (added)
# → -GetGameComponentsByGameIdAsync (removed from LoadSection scope)
```

**The composability matters.** `recur-git` doesn't duplicate recur's hierarchy logic. It translates git output into a format recur already understands. If git changes its output format, only `recur-git` changes. If recur adds new hierarchy features, `recur-git` gets them for free through the pipe.

### Git-Aware Assertions (recur-git | recur assert)

This is where `recur-git` + `recur assert` converge through pipes. The assertion isn't just "does this property hold in the codebase now?" — it's "does this property hold **in this diff?**"

```sh
# Assert: this PR doesn't reintroduce the full-game fetch in LoadSection
recur-git diff main..HEAD -f Tab.PlayerView.cs | recur children "LoadSection" | not-contains "+GetGameComponentsByGameIdAsync"

# Assert: every file changed in this PR has a corresponding .complete.md
recur-git changed-files main..HEAD | recur each-has-sibling "*.complete.md" -d docs/

# Assert: no .current.md files exist on this branch (all work is done)
recur files "**.current.md" -d docs/ | count == 0
```

These run in CI on every PR. They compose `recur-git` (git awareness) with `recur` (hierarchy querying) with `recur assert` (property checking). Three tools, one pipe, zero tokens.

### Context Scoping for LLM Sessions (Token Efficiency)

This is the practical win for day-to-day Copilot usage. `recur-git` knows what changed. `recur` knows the hierarchy. Together through pipes they scope what the LLM needs to see:

```sh
# "What files matter for this conversation?"
recur-git changed-files main..HEAD
# → 7 files (not 600 lines of unchanged Tab.PlayerView.cs)

# "What concerns are active?"
recur files "**.current.md" -d docs/
# → 2 active concerns. Only those need full context.

# "What did I just change?"
recur-git diff HEAD~1..HEAD | recur children --depth method
# → 3 methods across 2 files. Send THOSE to the LLM, not everything.
```

**The token math:**

| Context strategy | Tokens sent | Quality |
|---|---|---|
| Send all open files (current) | ~25,000 | LLM has everything, wastes most |
| Send git diff files only | ~5,000 | LLM sees what changed |
| Send recur-scoped diff methods | ~1,500 | LLM sees exactly what matters |

Same result. 94% fewer tokens. 5x more sessions for the same cost.

### Eventness Files as Git-Native Artifacts

The `.todo.md` → `.current.md` → `.complete.md` lifecycle is already git-native. Each rename is a commit. The git log IS the eventness history. `recur-git` can surface this:

```sh
# How did this concern progress?
recur-git log --follow "docs/**PhaseA**"
# → abc1234 rename PhaseA.todo.md → PhaseA.current.md
# → def5678 rename PhaseA.current.md → PhaseA.complete.md

# Pipe to recur for hierarchical view:
recur-git log --follow "docs/**PhaseA**" | recur tree
# → PhaseA
#     .todo.md (created: abc1234)
#     .current.md (renamed: def5678)
#     .complete.md (renamed: ghi9012)
```

The eventness suffix IS the git history. Recur doesn't need to build a separate state machine. Git already tracks state transitions. Recur just queries them hierarchically.

### Branch Naming as Recur Lanes (via recur-git)

Branches already use `/` separators. Recur's lane design principle (one separator per lane) maps directly. `recur-git` emits branches as hierarchical output:

```sh
# Git branches are a lane with "/" separator
copilot/optimize-section-loading
copilot/fix-media-cache
feature/executor-sqlite
bugfix/trim-contenttype

# recur-git emits, recur queries
recur-git branches | recur children "copilot"
# → optimize-section-loading, fix-media-cache

recur-git branches | recur children "feature"
# → executor-sqlite
```

Branch names become discoverable the same way filenames are. The convention is the same. The hierarchy is the same. The tool is the same.

### Merge as Hierarchical Convergence (recur-git merge-base)

When recur merges lanes with different separators, it already handles the complexity of combining `docs/CreateWizard3.Tab.PlayerView.SectionLoad.PhaseA.complete.md` (dots + slashes) into a unified tree.

Git merge is the same concept at the time dimension — two branches with different commit histories converging. `recur-git` can emit the divergence, recur can visualize it:

```sh
# What concerns diverged between branches?
recur-git merge-base main copilot/optimize-section-loading | recur tree
# → CreateWizard3
#     Tab.PlayerView
#       SectionLoad ← diverged (5 commits on copilot, 0 on main)
#       MediaRef ← diverged (2 commits on copilot, 1 on main) ← conflict risk
#     Tab.Hierarchy ← copilot only
```

This tells the developer: "PlayerView.MediaRef has changes on both branches — look there for merge conflicts." Recur doesn't resolve the conflict. It shows you where the hierarchies diverged.

---

## The Full Stack: Four Depths

| Depth | What | Recur command | Status |
|-------|------|--------------|--------|
| **1. Filesystem** | Files, directories, lanes | `recur children -d`, `recur tree`, `recur files` | ✅ Today |
| **2. Within files** | Methods, classes, headings | `recur children -f`, `recur trace` | 🔜 Planned |
| **3. Across time** | Commits, diffs, branches | `recur-git diff \| recur children`, `recur-git log \| recur tree` | 🔧 recur-git exists |
| **4. Across sessions** | LLM context scoping | `recur-git changed-files \| recur scope` | 🔮 Future |

**The composability model:** `recur-git` handles git ↔ stdin/stdout translation. `recur` handles hierarchy queries. `recur assert` handles property checks. Each is a single-purpose tool. Pipes connect them. No monolith.

Depth 4 is where recur + MCP converges. `recur-git changed-files` + `recur files "**.current.md"` provides the scoped context. The LLM receives only what matters. Token cost drops. Session quality stays at 10.

**The floor is always 9** — recur alone handles depths 1–3, deterministic, offline, free.
**The ceiling is 10** — recur + MCP at depth 4, when the task warrants it.

---

## Recur Is Not User Level Up

Everything above uses User Level Up as the worked example. But recur doesn't know what User Level Up is. It doesn't know what C# is. It doesn't know what a GameComponent is. It knows one thing: **hierarchy**.

Hierarchy is everywhere. Not just in code.

### Any Codebase

The conventions change. The tool doesn't.

```sh
# React project — components are hierarchy
recur tree -d src/components/
# → Dashboard
#     Sidebar
#       NavItem
#     Header
#       UserMenu

# Rust project — modules are hierarchy
recur children "auth" -d src/
# → login.rs, oauth.rs, session.rs

# Monorepo — packages are hierarchy
recur tree -d packages/
# → @myorg
#     api (14 files changed)
#     web (3 files changed)
#     shared (0 files changed — safe to skip in review)
```

The developer names things. Recur reads the names. The hierarchy emerges. No framework. No config file. No `.recurrc`.

### Any Documentation

A book is hierarchy. Chapters contain sections. Sections contain paragraphs. Characters appear and disappear across chapters. Plot threads start and resolve.

```sh
# Novel project — chapters are hierarchy
recur tree -d chapters/
# → Act1
#     Ch01.TheDeparture
#     Ch02.TheRefusal
#     Ch03.TheMentor
#   Act2
#     Ch04.TheThreshold
#     Ch05.TheOrdeal

# Character tracking — who appears where?
recur find "Elara" --scope "**" -d chapters/ --ext .md
# → Ch01 (3 mentions), Ch03 (7 mentions), Ch05 (12 mentions)
# → Ch02 (0 mentions) ← character absent from this chapter

# Plot thread tracking
recur files "**.todo.md" -d plot-threads/
# → Act2.TheBetrayal.todo.md ← unresolved plot thread
# → Act3.TheReturn.todo.md ← unresolved plot thread
recur files "**.complete.md" -d plot-threads/
# → Act1.TheCall.complete.md ← resolved
```

Same tool. Same commands. The writer names their files with intent. Recur shows the shape.

### Any Project Management

Sprints, epics, tasks — hierarchy. Recur doesn't need Jira.

```sh
# Sprint tracking
recur files "**.current.md" -d sprints/
# → Sprint12.Auth.OAuth.current.md ← in progress
# → Sprint12.API.RateLimit.current.md ← in progress

recur files "**.todo.md" -d sprints/
# → Sprint12.Auth.MFA.todo.md ← not started
# → Sprint13.Billing.Stripe.todo.md ← next sprint

# Cross-sprint discovery
recur files "**.Auth.**" -d sprints/
# → Sprint12.Auth.OAuth.current.md
# → Sprint12.Auth.MFA.todo.md
# → Sprint11.Auth.BasicLogin.complete.md
# → Sprint10.Auth.Registration.complete.md
```

The entire auth history, across sprints, one query. No dashboard. No API. No subscription.

### Any Research

Papers cite papers. Topics branch into subtopics. Literature reviews are hierarchy.

```sh
# Literature review
recur tree -d papers/
# → MachineLearning
#     Transformers
#       Attention.Vaswani2017
#       BERT.Devlin2019
#       GPT.Radford2018
#     Diffusion
#       DDPM.Ho2020
#       StableDiffusion.Rombach2022

# What's read vs unread?
recur files "**.read.md" -d papers/ | count
# → 12 papers read
recur files "**.unread.md" -d papers/ | count
# → 34 papers unread
recur files "**.Transformers.**.unread.md" -d papers/
# → 2 transformer papers still unread
```

### The +10 Level Up

When recur is incorporated into existing software, it doesn't replace that software. It **enhances** it:

| Existing tool | Without recur | With recur piped in |
|---|---|---|
| **git** | Flat file lists, linear log | Hierarchical concern tree across commits |
| **VS Code** | File explorer, text search | `recur find` with concern-scoped context |
| **CI/CD** | Pass/fail on tests | `recur assert` on structural invariants |
| **Jira/Linear** | Manual ticket tracking | `recur files "**.todo"` auto-discovers state |
| **Documentation** | Static pages | `recur tree` shows living structure |
| **LLM/Copilot** | Sends everything | `recur scope` sends only what matters |

Every tool that works with files can pipe through recur. Every tool that has hierarchy in its naming can be queried by recur. The integration is stdin/stdout. The cost is zero. The binary is one file.

That's the +10. Not replacing software. Making every piece of software aware of the hierarchy it already contains but can't query.

---

## The Floor and the Ceiling

Most developer tools are evaluated by their **ceiling** — what's the best possible outcome? But the ceiling is a lie. It assumes perfect conditions: the service is up, the model is good today, the API hasn't changed, the rate limit hasn't been hit, the context window is large enough, the training data included your framework.

The honest question is: **what's the floor?** What happens on a bad day? What happens offline? What happens when the budget runs out? What happens when the AI hallucinates? What happens at 3am when the service is down and production is broken?

### The Floor Is What You Can Count On

```
recur (today):
  Floor: 8     → always works, always deterministic, always free, always offline
  Ceiling: 8   → can't read inside files, can't see git history, can't predict next steps

recur + recur-git + recur assert (tomorrow):
  Floor: 9     → same guarantees + git-aware hierarchy + automated property checks
  Ceiling: 9   → surfaces every pattern, catches every contradiction, but can't act on them

recur + MCP/LLM (when warranted):
  Floor: 4     → service outage, hallucination, wrong model, rate limited, expensive
  Ceiling: 10  → semantic understanding, code generation, multi-phase orchestration

recur + recur-git + recur assert + MCP/LLM:
  Floor: 9     → recur handles depths 1-3 regardless of LLM availability
  Ceiling: 10  → LLM adds semantic layer on top of recur's structural queries
```

The last row is the product. **The floor never drops below 9 because recur doesn't depend on the LLM.** The LLM is an optional accelerator that raises the ceiling when conditions are good. When conditions are bad, you still have recur. You still have assertions. You still have git-aware hierarchy. You still have deterministic, free, offline querying.

### Why the Floor Matters More Than the Ceiling

A tool with a ceiling of 10 and a floor of 4 is **unreliable**. You can't put it in CI. You can't depend on it at 3am. You can't budget for it predictably. You can't reproduce its results. You get a different answer on Tuesday than you got on Monday because the model was updated over the weekend.

A tool with a ceiling of 9 and a floor of 9 is **infrastructure**. It works the same way every time. It costs the same every time (zero). It runs the same offline as online. It gives the same answer today as tomorrow. You can put it in CI and it never flakes.

The combination — floor of 9 from recur, ceiling of 10 from LLM — gives you both. Infrastructure reliability with AI capability. The LLM doesn't lower the floor because the floor is recur's job. The LLM only raises the ceiling.

### The Floor in Practice: This Session

This session itself demonstrates both the floor and the ceiling:

**What the LLM provided (ceiling):**
- Semantic understanding of the bug ("section can't be in its own children")
- Fix pattern recognition ("fetch sections separately, pass as parameter")
- Multi-phase orchestration across C#, JavaScript, and CSHTML
- Code generation for each phase
- Design conversation about recur's future

**What recur would have provided alone (floor):**
- Discovery: `recur files "**.current" -d docs/` → what's active
- Tracking: `recur children "SectionLoad" -d docs/` → phase progress
- Search: `recur find "GetGameComponentsByGameIdAsync" --scope "**" --ext .cs` → where's the old pattern
- Verification: `recur files "**.todo.md" -d docs/` → what's not done
- Contradiction detection: `recur files "**.todo.md" -d docs/ | recur each-has-sibling "*.complete.md"` → stale state

The floor doesn't write the code. But it shows you **where to look, what's active, what's stale, and what contradicts what**. A competent developer with that information fixes the bug without an LLM. It takes longer, but it works. Every time. For free.

**What recur would have caught that the LLM missed:**
- 5 stale `.todo.md` files still open after work completed
- 2 `.current.md` files that should be `.complete.md`
- These contradictions would derail the next LLM session with polluted context

The floor catches what the ceiling misses. That's the point.

### The Floor for Other Domains

The floor/ceiling model applies everywhere recur goes:

| Domain | Floor (recur alone) | Ceiling (recur + intelligence) |
|---|---|---|
| **Codebase** | Find patterns, track phases, detect stale state | Understand semantics, generate fixes, orchestrate changes |
| **Book writing** | Track characters across chapters, find unresolved plots | Suggest plot improvements, detect narrative inconsistencies |
| **Project management** | Show what's active/done/blocked across sprints | Predict bottlenecks, suggest task reordering |
| **Research** | Track read/unread, show citation hierarchy | Identify gaps in literature, suggest next papers |
| **CI/CD** | Assert structural invariants on every commit | Suggest new assertions based on change patterns |

In every case: the floor is deterministic, free, and offline. The ceiling requires intelligence (human or AI). Recur provides the floor. The intelligence — whatever form it takes — provides the ceiling. The floor never depends on the ceiling.

### The Floor Is the Product

This is the pitch:

> **Recur guarantees a floor of 9 for any project that names things with intent.** The ceiling depends on what you plug into the pipe — a human, a local model, a cloud LLM, or nothing at all. The floor never moves. The ceiling is your choice.

No other tool makes this guarantee. Test frameworks have a floor of 0 when tests aren't written. Linters have a floor of 0 when rules aren't configured. AI assistants have a floor of 0 when the service is down. Recur's floor is 9 because it reads files. Files are always there. Names are always there. Hierarchy is always there.

**The floor is the product. The ceiling is the bonus.**

---

## Summary

| Concept | Today | Tomorrow |
|---------|-------|----------|
| **Hierarchy scope** | Files (dot-separated names) | Files + text + git (via recur-git) + LLM context |
| **Query** | `recur children`, `recur find`, `recur files` | Same commands, new depth: `-f` for intra-file |
| **Git integration** | `recur-git` exists (stdin/stdout) | Pipe: `recur-git diff \| recur children \| recur assert` |
| **Assertion** | Human reads output | `recur assert` checks properties, git-aware via pipes |
| **CI** | Manual verification | `recur assert -d docs/assertions/` in pipeline |
| **LLM efficiency** | Send everything | `recur-git changed-files \| recur scope` sends only what matters |
| **Composability** | Single binary | Three tools piped: `recur-git` → `recur` → `recur assert` |
| **Dependencies** | Single binary, zero deps | Same (git is already installed) |
| **AI required** | No (depths 1–3) | Optional (depth 4, for ceiling) |

The recur command is the question. The result is eventness. The assertion checks the property. `recur-git` bridges the time dimension. Pipes connect them all. Everything in between is not your concern.
