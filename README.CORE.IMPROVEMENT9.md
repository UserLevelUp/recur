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

## Why This Matters

### Human Value
- Faster impact analysis: not just which files changed, but which in-file IDs/contracts changed.
- Better reviews: reviewers can inspect ref chains and unresolved identifiers quickly.
- Less drift: docs/tests/notes can share the same ID taxonomy as code references.

### LLM Value
- Deterministic context narrowing: LLM can query exact files first, then exact in-file symbols.
- Better planning loops: detect missing IDs/references and propose concrete next files.
- Lower hallucination risk: the LLM can query real graph edges instead of inferring architecture from prose.

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

Do not merge them into one monolithic command.
Composable stages are easier to reason about, test, and automate.

---

## Implementation Plan (Suggested)

### Phase 1: Minimal Viable In-File
- Add `recur in id` with plain-text extraction.
- Support `--stdin`, `--ext`, `--sep`, `--json`.
- Reuse existing search option plumbing.

### Phase 2: Reference Graph
- Add `recur in refs`.
- Emit `(from_id -> to_id, file, line)` edges.

### Phase 3: Trace + Gaps
- Add `recur in trace`.
- Add `recur in gaps` with required suffix policy.

### Phase 4: Language Extractors
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
- incremental indexing as future optimization (optional)

---

## Success Criteria

- Can chain file selection + in-file graph queries in one pipeline.
- Humans can answer "what changed semantically?" in minutes, not hours.
- LLM workflows become deterministic:
  - select files
  - extract IDs
  - trace references
  - report gaps

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
