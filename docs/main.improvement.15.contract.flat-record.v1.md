# Improvement 15 Contract: Flat Record Schema v1

Status: `frozen.v1` (normative)

## Purpose

Define the canonical flat record exchanged by:

- `recur flatten --json` (producer)
- `recur merge --format flat` (composer)
- `recur unflatten` (consumer)

This contract is versioned to keep interoperability stable.

## Canonical Shape

```json
{
  "path": "config.database.host",
  "value": "localhost",
  "kind": "text"
}
```

## Field Rules

### `path` (required, string)

- hierarchical path identifier
- separator semantics use command separator context (global `--sep`, default `.`)
- arrays use bracket segments: `items[0]`
- attributes use suffix form: `server@host`

### `value` (optional, string or null)

- terminal payload value
- omitted or `null` indicates non-terminal/structural entry

### `kind` (required, string)

Allowed values (v1):

- `element`
- `attribute`
- `text`

Compatibility rule:

- if `kind` is missing in inbound legacy records, consumers MAY treat it as `text`

## Optional Metadata Fields

Allowed as pass-through in v1:

- `sep` (source separator marker)
- `source_index` (merge source precedence index)
- `frame` (frame grouping hint)
- `layer` (composition layering hint)

Consumers MUST ignore unknown metadata fields unless strict mode is enabled.

## Path Grammar (v1)

Informal grammar:

```text
path        := segment (sep segment)*
segment     := name index*
index       := "[" digits "]"
attribute   := path "@" name
sep         := command separator (default '.')
```

Notes:

- empty `path` is reserved for document-root primitives and MAY be rejected by strict consumers
- separators inside raw segment names are out of scope for v1 escaping

## Ordering

When order matters, canonical sort is:

1. `path` lexicographic
2. `kind` lexicographic
3. stable source order for ties

## Versioning Rules

- Breaking schema changes require `v2` doc and explicit migration notes.
- v1 producers/consumers SHOULD remain backward compatible for missing `kind`.
- New metadata fields are additive and non-breaking when ignored by default.

## Related

- `docs/main.command.flatten.readme.md`
- `docs/main.command.merge.flat-format.contract.v1.md`
- `docs/main.command.unflatten.contract.v1.md`
