# Core Improvement 17: Depth-Windowed Token Separators for Recomposition Pipelines

**Status:** Future Vision (Long-Distance Backlog, Not Active)  
**Priority:** High  
**Category:** Flatten/Unflatten Semantics, Performance, Hierarchical Chunking

## Vision

Enable efficient and predictable work on very deep structured content (JSON/XML/TOML/YAML) by combining:

1. token separators (multi-character, not only single-char),
2. depth-window operations on flattened paths,
3. future `unflatten` round-trip materialization.

Target pipeline:

```bash
flatten -> filter/chunk by depth -> merge/operate -> unflatten
```

## Why This Improvement Exists

Deep files create two recurring problems:

1. Too much path fan-out at once (difficult to focus operationally).
2. Separator collisions when data keys contain separator-like characters.

Token separators and depth windows can reduce ambiguity and improve chunkability for large workflows.

## Current Reality (2026-03-01)

- `tree/files/merge` support token separators (for example `--sep "__"`).
- `flatten` still executes with single-character separator behavior.
- `unflatten` remains contract-only (`Improvement 15`), not CLI-implemented.

Improvement 17 is therefore intentionally parked until those dependencies mature.

## Core Idea

Treat path hierarchy as an explicit token stream and operate on bounded depth ranges:

- Work at one depth window first (`d..d+k`).
- Run targeted transforms/filters/merge in that window.
- Expand window only when needed.

This allows focused, staged processing of deep structures instead of whole-tree churn.

## Proposed Future Capabilities

1. **Token-stable flatten paths**
   - `flatten` must honor full separator tokens (`.`, `_`, `__`, `::`, etc.).
2. **Depth-window filtering**
   - ability to constrain operations to path depth ranges in flat records.
3. **Chunk planning**
   - deterministic chunk partitioning by depth and prefix.
4. **Round-trip safety**
   - `flatten -> merge(flat) -> unflatten` with collision diagnostics.
5. **Collision-aware policies**
   - explicit handling when keys contain separator tokens.

## Dependency Chain

1. Improvement 15 core implementation (`unflatten` MVP).
2. Flatten token-separator parity with tree/files/merge.
3. Flat-record contract stability (`path`, `value`, `kind`) across operations.

## Phased Future Plan

| Phase | Name | Outcome | Status |
|------|------|---------|--------|
| A | Contract Freeze | define token + depth-window semantics | planned |
| B | Flatten Token Parity | flatten honors full token separators | planned |
| C | Unflatten Token Parity | unflatten reconstructs from token paths | planned |
| D | Depth Window Ops | depth-bounded filter/chunk workflows | planned |
| E | Performance Validation | benchmark deep/wide workloads and guardrails | planned |

## Research Questions

1. What token escaping contract is needed for keys containing the token itself?
2. Which depth metrics best predict useful chunk boundaries?
3. Should chunking be deterministic by lexical path, structural prefix, or both?
4. How should merge provenance be retained across chunked windows?

## Success Criteria

- Token separators round-trip without silent path collapse.
- Depth-window operations reduce runtime and memory pressure on deep datasets.
- Chunked pipelines remain deterministic and composable.
- Contract tests validate `flatten -> merge(flat) -> unflatten` across deep/wide fixtures.

## Non-Goals

- Immediate implementation in current phase lanes.
- Replacing Improvement 15 scope.
- Introducing opaque automatic chunking without explicit operator controls.

## Related

- `README.CORE.IMPROVEMENT15.md`
- `README.CORE.IMPROVEMENT16.MD`
- `docs/main.command.flatten.separator-token.investigation.md`
- `docs/main.improvement.17.todo.future-plan.md`
