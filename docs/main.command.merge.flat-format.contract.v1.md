# Command Contract: merge `--format flat` v1

Status: `frozen.v1` (normative)

## Purpose

Define how `recur merge` composes sources into flat records without losing semantic fields (`value`, `kind`).

This contract is specifically for:

```bash
recur merge ... --format flat
```

## CLI Surface (v1)

```text
recur merge [existing options] --format flat [--json]
```

Rules:

- `--format flat` selects flat-record output mode.
- output MUST be JSON array in v1.
- `--json` MAY be accepted but is redundant in this mode.

## Accepted Input Shapes

`merge` flat mode accepts current merge inputs plus flat arrays:

1. array of strings (`["a.b.c", "x.y.z"]`)
2. object with `files` array
3. tree-like JSON containing `path` leaves
4. array of objects containing `path` (including flatten output)

## Input Normalization to Flat Records

If source item is path-only:

```json
{ "path": "a.b.c", "value": null, "kind": "element" }
```

If source item includes `value`/`kind`, those fields are preserved.

If source item omits `kind`, default to `text` only when `value` exists; otherwise `element`.

## Merge Precedence and Conflicts

Source precedence is deterministic:

- source order = CLI/input order (left to right, top to bottom for stdin objects)
- higher precedence = later source

Conflict key (v1):

- `(path, kind)`

Conflict behavior (v1 default):

- last-wins by source precedence
- winner keeps `value` and metadata

Rationale:

- aligns with overlay configuration workflows
- keeps behavior scriptable and predictable

## Separator Provenance

When `--show-sep` is active in flat mode:

- DO NOT mutate `path` text with marker suffixes like `"[.]"`.
- attach provenance as metadata field:

```json
{ "path": "a.b.c", "value": "x", "kind": "text", "sep": "." }
```

This avoids lossy path rewriting before `unflatten`.

## Output Guarantees

`--format flat` output MUST:

- conform to `docs/main.improvement.15.contract.flat-record.v1.md`
- be deterministic for same inputs/options
- preserve semantic fields when provided

## Non-goals (v1)

- custom conflict resolvers in merge itself
- schema-aware type coercion
- profile-aware rendering

## Related

- `docs/main.command.merge.readme.md`
- `docs/main.improvement.15.contract.flat-record.v1.md`
- `docs/main.command.unflatten.contract.v1.md`
