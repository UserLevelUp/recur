# main.command.flatten.readme

Command overview for `flatten`.

## Purpose

Convert structured data (currently XML, JSON, TOML, YAML, and CSV) into flat hierarchical records:

```text
path = value
```

This gives a common intermediate format for downstream filtering, searching, and pipeline composition.

## Supported inputs

- XML (including `.xml`, `.nuspec`, `.csproj`, and related XML-family files)
- JSON (`.json`, `.jsonl`)
- TOML (`.toml`)
- YAML (`.yaml`, `.yml`)
- CSV (`.csv`)

Format detection is extension-based. You can override with `--format xml|json|toml|yaml|csv`.

## Core usage

```bash
recur flatten config.xml
recur flatten data.json --json
recur flatten .recur/config.toml --format toml
recur flatten appsettings.yaml --format yaml
recur flatten levels.csv --format csv
recur flatten data.json --filter "config.database"
recur flatten data.json --max-depth 2
cat pom.xml | recur flatten --stdin --format xml
```

Use global `--sep` to control hierarchy delimiter in flattened paths:

```bash
recur --sep _ flatten config.json --json
```

## Output shape

Text mode prints:

```text
config.db.host = localhost
config.db.port = 5432
```

JSON mode prints an array:

```json
[
  { "path": "config.db.host", "value": "localhost", "kind": "text" },
  { "path": "config.db.port", "value": "5432", "kind": "text" }
]
```

Notes:
- XML attributes are emitted as `path@attribute`.
- Repeated siblings/arrays are indexed like `[0]`, `[1]`.

## merge compatibility (current)

`recur merge` can ingest `flatten --json` output because merge accepts JSON arrays of objects containing a `path` field.

Current limitations:
- merge builds a file-style hierarchy and does not preserve flatten `value` or `kind`.
- dot-separated flattened paths are lossy in merge output because merge interprets the last segment as a file extension.

Workaround for now:

```bash
recur --sep _ flatten source-a.xml --json > a.flat.json
recur --sep _ flatten source-b.json --json > b.flat.json
recur merge a.flat.json --sep _ b.flat.json --sep _ --base config
```

PowerShell stdin example:

```powershell
@(
  (recur --sep _ flatten source-a.xml --json | Out-String),
  (recur --sep _ flatten source-b.json --json | Out-String)
) -join "`n" | recur merge --stdin --base config --sep _ --sep _
```
