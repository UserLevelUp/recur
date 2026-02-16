# Command: flatten JSON Windows BOM Compatibility

Status: `complete` (historical todo record; see `docs/main.command.flatten.json.windows.bom.complete.md`)

## Problem

`recur flatten` with JSON input fails when content starts with UTF-8 BOM (`EF BB BF`) in file or stdin modes.

## Symptoms

```bash
recur flatten bom.json --format json --json
# Error: expected value at line 1 column 1

type bom.json | recur flatten --stdin --format json --json
# Error: expected value at line 1 column 1
```

## Scope

- BOM-tolerant JSON parse path used by flatten JSON format implementation.
- Keep YAML/TOML/XML behavior unchanged.

## Code Targets

- `src/main_command_flatten_json.rs` (`serde_json::from_str` input normalization)

