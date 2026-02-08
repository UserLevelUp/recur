# Core Improvement 9: In-File Hierarchy Intelligence

## Current Status

**PLANNED / FORWARD-LOOKING** - Not implemented yet.

This proposal extends `recur` from:
- file-level hierarchy (`recur files`, `recur tree`, `recur stats`)

to:
- in-file hierarchy understanding (structured IDs, refs, contracts, tasks inside file content)

---

## Overview

`recur` is already strong at selecting the right files.
IMPROVEMENT9 proposes a second stage:

1. Select files with existing `recur` commands.
2. Run an in-file hierarchy command on that exact set.

This gives precise, composable analysis for both humans and LLMs.

---

## Core Idea

Introduce an in-file command family (working name: `recur in`), designed to consume file sets from stdin or scope selection.

Examples (proposed):

```bash
# Stage 1: select files by file hierarchy
recur files "main.command.**.readme" -d docs/ \
  | recur in id "main.command.files.**" --stdin

# Stage 1 with Rust underscore naming
recur files "main_command_*_impl" -d src/ --sep _ \
  | recur in id "main.command.files.**" --stdin

# Analyze TODO chains only in selected command docs
recur files "main.command.**.todo*" -d docs/ \
  | recur in refs "todo.**" --stdin
```

This model keeps Unix composability: file filtering and in-file semantics stay separate but chain cleanly.

---

## Design Thesis Update: Dual-Layer Hierarchy

IMPROVEMENT9 should formalize two layers that can be switched and chained at will:

1. File layer (`recur files/tree/stats`)
- Answers: "Which files matter?"
- Uses folder-appropriate separators (`--sep _` for Rust modules, `.` for docs/tests).

2. In-file layer (`recur in *`)
- Answers: "Which semantic IDs, refs, tasks, or events matter inside those files?"
- Reads selected files from stdin and/or a simple semantic-name list file.

The power is the composition:

```bash
# Select implementation modules with source separator
recur files "main_command_*_impl" -d src/ --sep _ \
  | recur in id "main.command.**" --stdin
```

This keeps separator policy local to the file selection phase while in-file semantics stay canonical.

---

## Legacy Codebase Adoption: Simple Semantic Name List First

For existing repositories, do not require immediate file renaming or leaf-file proliferation.
Start with one plain text semantic list:

- `docs/main.semantic.names.txt`

Format rules:
- one semantic ID per line
- canonical dot IDs only (`prefix.base.suffix[.qualifier]`)
- dot notation is the default for hierarchical files and semantic IDs
- optional blank lines and `#` comments
- no embedded metadata schema required
- use `.todo.tracking` when an item is centrally tracked in the list

Example file:

```text
main.command.tree.todo.current
main.command.tree.todo.trigger.event
main.command.checkpoint.todo.current
main.command.checkpoint.todo.trigger.event
```

Resolution model:

1. Analyze/select candidate IDs quickly from the text list (human first, then LLM).
2. Resolve selected IDs against the file layer to retrieve concrete context.
3. Run in-file extraction only on resolved files.

In this model, interest lives in semantic IDs while actual working context is retrieved from matched files at the file layer.

Context retrieval examples:

```bash
# docs/tests lane (dot separator)
recur files "main.command.checkpoint.todo.current" -d docs/

# src lane (underscore separator)
recur files "main_command_checkpoint_todo_current" -d src/ --sep _
```

Recommended command additions:

1. `recur in id|refs|trace|gaps --names-file docs/main.semantic.names.txt`
2. `recur in sync --names-file docs/main.semantic.names.txt` (refresh list from repo)
3. `recur in drift --names-file docs/main.semantic.names.txt` (list IDs with no matching files)
4. `recur in lane current|set --names-file ...` (single active cursor management)

This gives immediate structure to legacy repos with minimal disruption.

Tracking example:

```text
main.improvement.9.todo.tracking
```

`*.todo.tracking` is intended for fast, centralized queueing in one file instead of scattered per-item metadata files.

---

## Why This Matters

### Human Value
- Faster impact analysis: not just which files changed, but which in-file IDs/contracts changed.
- Better reviews: reviewers can inspect ref chains and unresolved identifiers quickly.
- Less drift: docs/tests/notes can share the same ID taxonomy as code references.

### LLM Value
- Deterministic context narrowing: LLM can query exact files first, then exact in-file symbols.
- Better planning loops: detect missing IDs/references and propose concrete next files.
- Lower hallucination risk: the LLM can query real semantic IDs and resolved files instead of inferring from prose.

### Human + LLM Combined Value
- Shared operational state: both humans and LLMs consume the same IDs, refs, statuses, and event logs.
- Faster handoffs: "current lane" is queryable, not hidden in chat memory.
- Better prioritization: event and dependency data can rank what to do next.
- Lower cognitive load: operators ask the system for "next valid action" instead of manually stitching context.

---

## Scope Expansion: Track More Than Code

The same hierarchy model can represent all work categories:

- engineering: `main.command.tree.impl`
- testing: `main.command.tree.test.case.stdin`
- docs: `main.command.tree.readme`
- incident response: `ops.incident.auth.outage.2026_02_08`
- release: `release.v2_3.rc1.checklist`
- experiments: `research.llm.context.windowing.sep_policy`
- product tasks: `product.search.ux.todo.priority`

This allows one query language for engineering + operations + planning.

---

## Event Triggers as First-Class Data

Add explicit event IDs and trigger rules in the in-file layer:

- `*.event.start`
- `*.event.blocked`
- `*.event.handoff`
- `*.event.complete`

Each event can carry:
- required checks (commands/tests)
- produced artifacts (checkpoint IDs, reports)
- next-lane transition rules

Example (conceptual):

```json
{"id":"main.command.tree.event.complete","requires":["cargo.test.quiet","julia.runtests"],"emits":["checkpoint.ck-20260208-tree-complete"],"next":"main.command.checkpoint.todo.current"}
```

This makes trigger behavior auditable and automatable while staying human-readable.

---

## Productivity and Interest Model

"Interest" here means what deserves attention now.
Use an explicit scoring model over the semantic ID list plus discovered refs:

- urgency (blocked, failing, near deadline)
- impact (number of downstream refs)
- freshness (stale TODOs / old checkpoints)
- confidence (parser certainty for extracted IDs)

Then expose:

- `recur in focus --names-file ... --top 20` (proposed)
- ranked work queue for humans
- deterministic context pack for LLM sessions

Result:
- humans get a prioritized worklist
- LLMs get high-signal context windows
- both operate on the same evidence base

---

## Proposed Command Surface

### 1) `recur in id`

Find in-file hierarchical identifiers matching a pattern.

```bash
recur in id <PATTERN> [--stdin] [-d DIR] [--ext LIST] [--sep CHAR] [--json]
```

Example:
```bash
recur in id "main.command.files.**" -d docs/
```

### 2) `recur in refs`

Find references between in-file IDs (edge view).

```bash
recur in refs <PATTERN> [--stdin] [-d DIR] [--json] [--count]
```

Example:
```bash
recur in refs "main.command.files.todo.**" -d docs/
```

### 3) `recur in trace`

Trace in-file ID references (similar to function trace, but for ID graph).

```bash
recur in trace <ID> [--stdin] [--depth N] [--direction callers|callees|both] [--json]
```

Example:
```bash
recur in trace "main.command.files.todo.priority" -d docs/ --depth 2
```

### 4) `recur in gaps`

Gap detection for required suffix chains inside selected files.

```bash
recur in gaps <BASE> --require readme,test,todo [--stdin] [--json]
```

Example:
```bash
recur files "main.command.**" -d docs/ \
  | recur in gaps "main.command.files" --require readme,todo,todo.priority --stdin
```

---

## Data Model (Proposed)

In-file IDs should follow the same contract as filenames:

`main.<area>.<unit>.<artifact>[.<qualifier>]`

Examples inside file content:
- `main.command.files.contract.v1`
- `main.command.files.todo.priority`
- `main.command.files.test.case.stdin.empty`

Reference formats (examples):
- Markdown link-style tags
- comment tags (`// id: main.command.files.contract.v1`)
- YAML/JSON key-value markers

Parser strategy:
- start with regex-based extractors per file type
- allow language-specific extractors later

---

## Immediate Workflows Enabled

### A) Changed-file semantic impact

```bash
git diff --name-only \
  | recur in id "main.command.**" --stdin --json
```

### B) Docs-to-tests consistency check

```bash
recur files "main.command.**.readme" -d docs/ \
  | recur in refs "main.command.**.test" --stdin --count
```

### C) Priority audit

```bash
recur files "main.command.**.todo*" -d docs/ \
  | recur in gaps "main.command" --require todo,todo.priority --stdin
```

---

## Separation of Concerns (Important)

- `recur files/tree/stats`: filesystem hierarchy truth.
- `recur in *`: content hierarchy truth.
- optional `--names-file`: coordination entry point (simple semantic ID seed list).

Do not merge them into one monolithic command.
Composable stages are easier to reason about, test, and automate.

---

## Implementation Plan (Suggested)

### Phase 1: Minimal Viable In-File
- Add `recur in id` with plain-text extraction.
- Support `--stdin`, `--ext`, `--sep`, `--json`.
- Reuse existing search option plumbing.

### Phase 2: Legacy-Friendly Semantic Name Overlay
- Add `--names-file` read path for `recur in id`.
- Add `recur in sync` and `recur in drift`.
- Add "single current lane" helpers (`recur in lane`).

### Phase 3: Reference Graph
- Add `recur in refs`.
- Emit `(from_id -> to_id, file, line)` edges.

### Phase 4: Trace + Gaps + Focus
- Add `recur in trace`.
- Add `recur in gaps` with required suffix policy.
- Add `recur in focus` ranking from event/dependency signals.

### Phase 5: Language Extractors
- Markdown extractor.
- Rust comment/doc extractor.
- JSON/YAML structured key extractor.

---

## Testing Strategy (Julia + Rust)

### Julia Integration
- Add `julia-tests/main.command.in.id.test.jl`
- Add `julia-tests/main.command.in.refs.test.jl`
- Add `julia-tests/main.command.in.trace.test.jl`
- Add `julia-tests/main.command.in.gaps.test.jl`

Test goals:
- respects stdin-selected file sets
- honors separator choice and precedence
- consistent JSON contracts
- stable exit codes for no-match scenarios

### Rust Unit Tests
- parser/extractor tests by file type
- ID normalization tests
- edge extraction tests
- gap policy tests

---

## Risks and Controls

### Risk: False positives from naive regex
Control:
- explicit marker prefixes for high-confidence mode
- language extractor adapters

### Risk: ID taxonomy drift
Control:
- central naming guide (`docs/main.dogfooding.readme.md`)
- CI checks using `recur in gaps`

### Risk: Performance on large repos
Control:
- always support stdin-scoped execution
- optional caching as future optimization (not required for the text-list model)

---

## Success Criteria

- Can chain file selection + in-file graph queries in one pipeline.
- Humans can answer "what changed semantically?" in minutes, not hours.
- LLM workflows become deterministic:
  - select files
  - extract IDs
  - trace references
  - report gaps
- Legacy repos can adopt with a single semantic-name text file before any large rename campaign.
- Event-trigger lanes are queryable and executable from data, not tribal memory.

---

## Example End-to-End Session (Target UX)

```bash
# 1) Select all command docs and tests for files command
recur files "main.command.files.**" -d docs/ \
  | recur in id "main.command.files.**" --stdin

# 2) Trace todo priority dependencies
recur files "main.command.files.todo*" -d docs/ \
  | recur in trace "main.command.files.todo.priority" --stdin --depth 2

# 3) Check missing required branches
recur files "main.command.files.**" -d docs/ \
  | recur in gaps "main.command.files" --require readme,todo,todo.priority --stdin
```

If this works reliably, `recur` becomes not only a file hierarchy tool, but a semantic coordination layer for humans + LLMs.
