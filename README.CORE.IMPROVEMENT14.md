# Core Improvement 14: Mirror Universe Eventness in `.recur`

**Status:** Major Milestone Proposal (`ideation`)  
**Priority:** Very High  
**Category:** Context Retention, Eventness, Adaptive Workflows

## Vision

Create a "mirror universe" inside `.recur/` that tracks:

1. **File eventness** (what is happening to each file)
2. **In-file eventness** (what is happening inside each file at symbol/section level)

This gives persistent context across sessions without depending on memory.

## Core Boundary

Recur remains a simple hierarchical tool.

Improvement 14 does **not** store the actual work product (for example paragraphs, sentences, or full code bodies) in mirror logs.  
It stores structured **eventness metadata** that mirrors what the user is doing so context can be recovered quickly.

## Why This Is a Major Milestone

Current eventness patterns are strong at file-level workflow state (`.todo`, `.current`, `.complete`), but real work often happens inside files:

- Coders care about classes, methods, tests, bugs, builds, and feature status.
- Writers care about chapters, scenes, paragraphs, personas, style, plot, and structure.

Improvement 14 adds a second layer of eventness so both file state and in-file state move together.

## Maturity Path

Improvement 14 is currently in **ideation** and should move through explicit gates:

### Phase A - Ideation (Current)

Goal:
- validate problem framing and boundaries
- define mirror eventness concept

Exit criteria:
- boundary agreed: metadata-only, no primary prose/code duplication
- command philosophy agreed: reuse simple recur primitives
- initial domain coverage agreed: coder + writer + mixed

### Phase B - Concrete Specification

Goal:
- lock data model and file layout
- define minimum viable command behavior

Exit criteria:
- canonical event schema versioned (v1)
- anchor naming conventions documented
- lifecycle/status transitions defined
- query patterns documented (`files/tree/find/merge` over mirror artifacts)

### Phase C - Prototype (Feature Feasibility)

Goal:
- implement smallest usable slice in repo

Exit criteria:
- Phase 1 file mirror implemented
- basic timeline query works
- at least one coder and one writer workflow validated

### Phase D - Production Candidate

Goal:
- harden and integrate with checkpoints/config

Exit criteria:
- session/snapshot integration complete
- regression tests + migration notes complete

## Problem Statement

Without in-file event tracking:

- Session handoffs lose detail.
- "What changed" is visible, but "why and where inside the file" is harder to recover.
- Context rebuild is expensive for humans and LLM agents.
- Different domains (code vs writing) need different anchors and signals.

## Core Idea

Treat `.recur/` as an event-sourced mirror of active work.

- **File mirror:** track lifecycle and intent per file.
- **In-file mirror:** track anchors inside files (functions, classes, sections, scenes, characters, etc.).
- **Adaptive profiles:** shape tracking to user domain (coder, writer, mixed).
- **Metadata only:** store reminders, status, clues, and references; do not duplicate full file content.
- **Memory crutch:** use recur as external memory for hard-to-hold context.
- **Domain lanes in `.recur`:** optional domain hierarchies (plot, character development, bibliography, illustrations, etc.) linked to real files.

## Proposed `.recur` Layout

```text
.recur/
  config.toml
  checkpoints.md
  domains/
    character-development/
    plot/
    bibliography/
    illustrations/
  mirror/
    index.jsonl
    sessions/
      2026-02-13T10-00-00Z.session.json
    files/
      src.main_command_merge_impl.rs.events.jsonl
      docs.chapter1.md.events.jsonl
    anchors/
      src.main_command_merge_impl.rs/
        fn.execute.events.jsonl
        fn.display_tree.events.jsonl
      docs.chapter1.md/
        section.intro.events.jsonl
        scene.02.events.jsonl
    snapshots/
      ck-impr14-phase1.json
```

## Domain Lanes and Linking

Improvement 14 allows domain extensions without creating new top-level project folders.
Domain artifacts live under `.recur/` and are linked back to real files and anchors.

Example config-style mapping:

```toml
[domains.character-development]
dir = ".recur/domains/character-development/"
sep = "."

[domains.plot]
dir = ".recur/domains/plot/"
sep = "."

[domains.bibliography]
dir = ".recur/domains/bibliography/"
sep = "."

[domains.illustrations]
dir = ".recur/domains/illustrations/"
sep = "."
```

This makes questions like these queryable:

- How does this chapter relate to `plot` and `character-development`?
- Is this anchor marked as a clue or hint?
- Which scenes have bibliography references?
- Which sections are linked to illustration planning?

Conceptually, any domain can be added to the hierarchy.  
Practically, domain naming and linking contracts are part of the refinement work.

## Eventness Model

### File Eventness

Tracks state transitions at file scope:

- `todo`
- `current`
- `blocked`
- `review`
- `complete`
- `failed`

### In-File Eventness

Tracks state transitions for anchors inside a file:

- Code anchors: module, struct, enum, trait, function, test, benchmark.
- Writing anchors: chapter, section, paragraph-group, scene, character-note, appendix.

Also tracks high-value memory aids:

- clue/hint markers
- source/page references
- biography continuity notes
- style/voice checks
- paragraph commentary events (short notes tied to an anchor)

### What Mirror Stores vs Avoids

Stores:

- event records
- short notes and reminders
- anchor references
- source pointers
- continuity flags

Avoids:

- full paragraph bodies
- full chapter content
- full code bodies
- large duplicated excerpts from primary files

### Event Record (Canonical Shape)

```json
{
  "ts": "2026-02-13T10:04:12Z",
  "scope": "anchor",
  "lane": "src",
  "path": "src/main_command_merge_impl.rs",
  "anchor": "fn.execute_stdin_mode",
  "profile": "coder",
  "domains": ["feature", "testing"],
  "event": "edit",
  "status": "current",
  "tool": "recur",
  "detail": "Adjusted JSON stream handling in stdin mode",
  "session_id": "2026-02-13T10-00-00Z"
}
```

## Adaptive Profiles

### Coder Profile

Focus signals:

- feature progress
- build/test outcomes
- bug states
- class/function/task linkage

Typical anchor names:

- `struct.FlatEntry`
- `fn.execute`
- `test.stdin_mode`

### Writer Profile

Focus signals:

- chapter/scene progression
- character and persona consistency
- fiction vs non-fiction mode
- style and section integrity

Typical anchor names:

- `chapter.03`
- `scene.03.02`
- `character.joe-bishop`
- `style.voice.dialog-heavy`
- `source.page.142`
- `commentary.paragraph.03`

### Mixed Profile

Supports docs + code + planning in one workspace with lane-specific anchor rules.

## Proposed Command Surface (Future)

```bash
recur mirror init
recur mirror track --file src/main.rs --event edit --status current
recur mirror track --file src/main.rs --anchor fn.execute --event test --status failed
recur mirror anchor list --file src/main.rs
recur mirror timeline --file src/main.rs
recur mirror timeline --anchor fn.execute --file src/main.rs
recur mirror suggest --profile coder
recur mirror suggest --profile writer
recur mirror snapshot --checkpoint-id ck-impr14-phase1
```

## Use Existing Simple Commands

Improvement 14 should stay aligned with recur's existing command style.
The mirror data should "pop out" through normal commands you already use:

```bash
# What mirror event files exist right now?
recur files "**.events" -d .recur/mirror/

# Show eventness tree for one mirrored file
recur tree "src.main_command_merge_impl.rs" -d .recur/mirror/files/

# Find blocked events across all mirrored anchors
recur find "\"status\":\"blocked\"" --scope "**" -d .recur/mirror/ -E

# Merge coding + writing mirror views for one session snapshot
recur merge \
  --pattern "src.main_command" --sep "." \
  --pattern "docs.chapter" --sep "." \
  -d .recur/mirror/

# Show domain-specific artifacts linked to writing work
recur tree "chapter.03" -d .recur/domains/plot/
recur files "**.clue.**" -d .recur/domains/character-development/
```

`recur mirror ...` commands (future) are convenience wrappers.  
Core value remains: simple hierarchical files + simple hierarchical queries.

## Workflow Examples

### Coding Workflow

```bash
# Start feature work
recur mirror track --file src/main_command_init_impl.rs --event feature-start --status current

# Mark test failure tied to a specific function
recur mirror track --file src/main_command_init_impl.rs --anchor fn.print_analyze_report --event test --status failed

# Mark fix + verification
recur mirror track --file src/main_command_init_impl.rs --anchor fn.print_analyze_report --event edit --status complete
recur mirror track --file src/main_command_init_impl.rs --event build --status complete
```

### Writing Workflow

```bash
# Start chapter revision
recur mirror track --file docs/chapter3.md --event draft --status current

# Track scene rewrite and persona continuity check
recur mirror track --file docs/chapter3.md --anchor scene.03.02 --event rewrite --status current
recur mirror track --file docs/chapter3.md --anchor character.joe-bishop --event consistency-check --status complete

# Track clue/hint and source reference without storing prose
recur mirror track --file docs/chapter3.md --anchor clue.locket --event note --status current
recur mirror track --file docs/chapter3.md --anchor source.page.142 --event cite --status complete

# Running commentary at paragraph anchor
recur mirror track --file docs/chapter3.md --anchor commentary.paragraph.03 --event review-note --status current

# Close chapter revision
recur mirror track --file docs/chapter3.md --event review --status complete
```

## Design Principles

1. **Append-only by default** for auditability.
2. **Domain-adaptive, not domain-locked**.
3. **Human-readable first**, machine-parseable always.
4. **No hidden magic**: explicit commands and visible artifacts.
5. **Composable outputs**: query with recur + grep/jq + git.

## Implementation Phases

### Phase 1 - File Mirror

- Create `.recur/mirror/files/*.events.jsonl`
- Track file-level events only
- Add timeline query by file
- Read lane defaults from `.recur/config.toml` produced by `recur init`

### Phase 2 - Anchor Mirror

- Add `.recur/mirror/anchors/...`
- Add anchor list and anchor timeline queries
- Support manual anchor naming first
- Define how anchor events link to domain lanes under `.recur/domains/...`

### Phase 3 - Profile Adapters

- Add `coder` and `writer` profiles
- Auto-suggest anchors based on profile heuristics
- Keep explicit override path for users
- Add domain recommendation hooks (for example, suggest `plot`/`character-development` links in writer flows)

### Phase 4 - Session and Snapshot Integration

- Session files and checkpoint binding
- Mirror snapshots linked to `recur-git checkpoint`
- Fast "resume where I left off" flow

### Phase 5 - Recommendation Layer

- Suggest next actions from recent eventness
- Surface stale anchors/files
- Highlight blocked work and test failures tied to anchors

## Success Criteria

- Resume context in under 60 seconds after session restart.
- Users can answer "what changed, where, and why" from mirror logs.
- Works for both coding and writing workflows without schema rewrites.
- Anchor-level state maps cleanly back to file-level state.
- No regression to existing recur workflows.
- Mirror logs do not duplicate primary prose/code bodies.

## Risks and Mitigations

### Risk: Event spam

Mitigation:

- event levels (`info`, `key`, `critical`)
- optional aggregation views

### Risk: Anchor drift after refactors

Mitigation:

- stable anchor IDs where possible
- rebind commands (`recur mirror rebind`)

### Risk: Over-complex schema

Mitigation:

- small canonical event shape
- profile-specific optional fields only

### Risk: Domain sprawl without clear contracts

Mitigation:

- define minimal domain lane contract in spec phase
- require explicit linking fields (`path`, `anchor`, `domains`)
- keep domain metadata queryable with existing commands first

## Refinement Needed (Still Ideation)

Improvement 14 is intentionally powerful but still needs refinement before implementation:

- finalize domain lane naming conventions
- finalize domain-linking schema and validation rules
- finalize conflict rules when one anchor maps to many domains
- define compaction/retention strategy for long-running mirror and domain logs

## Relationship to Existing Improvements

- Complements current eventness workflows (`.todo`, `.current`, `.complete`).
- Pairs with `.recur/config.toml` lane detection and project awareness.
- Extends checkpoint workflows through mirror snapshots.
- Bridges structured document work (Improvement 12/13 direction) with practical session memory.

## Summary

Improvement 14 turns `.recur/` into a live project memory system:

- file eventness + in-file eventness
- coder + writer adaptive behavior
- explicit, queryable, append-only metadata context
- reminders for clues, hints, sources, biographies, style, and paragraph commentary

This is a major milestone because it shifts recur from hierarchy-aware discovery to hierarchy-aware **continuity**.
