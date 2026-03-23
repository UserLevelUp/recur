# Improvement 18: recur-map

Status: `brainstorm`
Date: 2026-03-13

## Core Idea

`recur-map` is a mapping layer on top of `recur`'s existing discovery primitives. Where `recur trace-id` answers *"where does this identifier appear?"*, `recur-map` answers *"what does this identifier correspond to in another namespace?"*

## Motivation

Two observations drove this:

1. **The memory problem.** Claude Code's memory system (`MEMORY.md` + typed files) is a flat pointer index. But if memory files adopted dot-path naming conventions (e.g., `memory/feedback.testing.no-mocks.md`), `recur id memory/` would discover and traverse them natively — no manual index needed. MEMORY.md becomes a projection of the file tree, not a hand-maintained list.

2. **Cross-namespace relationships.** A dot-path identifier in `docs/` (`main.improvement.9.trace-id`) relates to code in `src/` (`trace_id`) and tests in `julia-tests/` (`runtests.trace-id`). Today you have to know these mappings. `recur-map` would make them explicit and queryable.

## Concepts

### Namespace Mapping
A mapping declares how identifiers in one tree correspond to identifiers in another:

```toml
[map.docs-to-src]
from = "docs/"
to = "src/"
transform = "replace('.', '_')"  # dot-path → snake_case
```

### Projection
Generate a flat index or structured output from a dot-path tree:

```
recur map project docs/ --format markdown
```

Outputs a MEMORY.md-style index derived from the file tree itself.

### Alias / Redirect
One identifier pointing to another across namespaces — like symlinks in eventness space. Useful for renaming, deprecation, or cross-repo references.

### Reverse Lookup
Given a `src/` symbol, find the `docs/` eventness that describes it — the inverse of `trace-id`.

## Relationship to Existing Tools

| Tool | Question answered |
|---|---|
| `recur id` | Where does this dot-path identifier appear in content? |
| `recur trace-id` | What files/functions relate to this trace identifier? |
| `recur-map` | What does this identifier correspond to in another namespace? |
| `recur-map project` | Generate an index/projection of a dot-path tree |

## Potential Third Binary

Like `recur-git` extends `recur` with checkpoint semantics, `recur-map` could be a third binary that extends `recur` with cross-namespace mapping. It would consume `.recur/map.toml` config alongside the existing `config.toml`.

## Open Questions

- Should mapping config live in `.recur/map.toml` or as a `[map.*]` section in `.recur/config.toml`?
- Is a new binary warranted, or is this a subcommand on `recur` itself (`recur map`)?
- How does this interact with `recur flatten`? Could a projection just be a flatten variant?
- What's the minimal MVP — probably just `recur map project <dir>` generating a markdown index.

## Related

- `docs/main.improvement.9.trace-id.todo.current.md` — trace-id is the prerequisite primitive
- `docs/main.dogfooding.trace-id.todo.md` — recur discovering its own structure
- Claude Code memory system as first concrete use case
