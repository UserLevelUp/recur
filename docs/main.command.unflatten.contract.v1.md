# Command Contract: unflatten v1 (MVP)

Status: `frozen.v1` (normative, implementation pending)

## Purpose

Define the first stable behavior for converting flat records back into materialized output.

`unflatten` is the counterpart to `flatten` in the pipeline:

```text
flatten -> merge(flat) -> unflatten
```

## CLI Surface (v1)

```text
recur unflatten [INPUT] [OPTIONS]

OPTIONS:
  --stdin
  --format <text|json>
  --profile <FILE>            repeatable; later files override earlier files
  --root <PATH>
  --strict
  --on-conflict <error|last-wins|first-wins|array>
  --sort <path|input>
  --output <FILE>
```

Notes:

- v1 command scope is `text|json` only.
- `xml|yaml|toml|csv` are explicitly deferred to later phases.

## Input Contract

- accepts flat record arrays per `docs/main.improvement.15.contract.flat-record.v1.md`
- supports file input and stdin input
- if `--root` is provided, only entries under that prefix are materialized

## Conflict Policy

Default:

- `--on-conflict last-wins`

Other modes:

- `error`: fail fast on first conflicting `(path, kind)`
- `first-wins`: keep first observed
- `array`: collect conflicting values as arrays (json mode only in v1)

Strict mode:

- `--strict` makes unknown `kind`, malformed paths, and unresolved conflicts fatal

## Output Semantics

### `--format json`

- reconstruct hierarchical JSON object from flat paths
- array indices (`[n]`) become JSON arrays
- attribute entries (`@name`) are represented in v1 as object keys prefixed by `@`
  - example: `server@host` -> `{ "server": { "@host": "localhost" } }`

### `--format text`

Two modes:

1. no profile:
   - emit deterministic `path = value` lines (debug/materialization view)
2. with profile:
   - profile-driven rendering behavior
   - intended for scene/frame outputs (for example ASCII animation)

## Profile Layering Contract

When multiple `--profile` values are provided:

- apply in argument order
- later profile overrides earlier keys

Profile files MAY include `extends`, but CLI order remains highest precedence.

## Determinism

For identical inputs and options, output MUST be identical.

Sorting:

- `--sort path` (default): canonical sort by path/kind
- `--sort input`: preserve normalized input encounter order

## Reserved Future Flags (out of v1 scope)

- `--frames`
- `--frame-key`

These are reserved for phase E and are intentionally not required in MVP.

## Related

- `README.CORE.IMPROVEMENT15.md`
- `docs/main.command.flatten.readme.md`
- `docs/main.command.merge.flat-format.contract.v1.md`
- `docs/main.improvement.15.contract.flat-record.v1.md`
