# Improvement 14: Mirror Eventness

Status: `todo.future-plan` (currently in ideation)

## Objective

Move Improvement 14 from ideation to a concrete, implementable feature plan without
breaking recur's simple command model.

The planning direction is:

- keep filename eventness as the fast discovery/index layer
- add richer in-file or mirrored eventness for canonical state when needed
- define one source of truth explicitly so the visible file layer and hidden
  `.recur/` mirror do not drift

## Maturity Roadmap

| Stage | Name | Outcome | Status |
|------|------|---------|--------|
| A | Ideation | Problem framing + boundaries | **active** |
| B | Concrete Spec | Schema, layout, command contract | planned |
| C | Prototype | Phase 1 file mirror runnable | planned |
| D | Production Candidate | Integrated, tested, migration-ready | planned |

## Stage A (Current) Exit Checklist

- Metadata-only boundary is explicit (no full prose/code duplication)
- File eventness + in-file eventness scope is explicit
- Canonical-vs-derived responsibility is explicit:
  filename eventness stays query-friendly, while mirrored or in-file eventness
  can become the authoritative state layer
- Coder + writer + mixed profiles are defined
- Existing command-first approach is explicit (`files`, `tree`, `find`, `merge`)

## Stage B Planning Targets

1. Define canonical event schema `v1` and required fields.
2. Define path conventions under `.recur/mirror/`.
3. Define anchor naming conventions by profile (coder/writer).
4. Define status transition rules (`todo -> current -> review -> complete`, etc.).
5. Declare which layer is canonical (`filename`, `in-file`, or mirror artifact)
   and how the other layers are derived or validated.
6. Define concrete query recipes for timeline and resume flows.
7. Define `.recur/domains/*` lane contract and naming rules.
8. Define document-to-domain linking fields and query patterns.

## Stage C Prototype Targets

1. Write file-level mirror events to `.recur/mirror/files/*.events.jsonl`.
2. Add minimal query command(s) or wrappers.
3. Demonstrate sync rules between visible filename eventness and the canonical
   mirror or in-file state.
4. Demonstrate one coding flow and one writing flow.
5. Validate compatibility with `.recur/config.toml` from `recur init`.
6. Demonstrate one domain-linked writing flow (for example: `plot` + `character-development`).

## Discovery

```bash
recur files "main.improvement.14.**" -d docs/
recur tree "main.improvement.14" -d docs/
recur files "README.CORE.IMPROVEMENT14" -d ./
```

## Related

- `README.CORE.IMPROVEMENT14.md`
- `docs/main.improvement.7.todo.current.md`
- `docs/main.command.trace-id.run.todo.current.md`
