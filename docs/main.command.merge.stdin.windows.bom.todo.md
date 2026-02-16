# Command: merge stdin Windows BOM Compatibility

Status: `complete` (historical todo record; see `docs/main.command.merge.stdin.windows.bom.complete.md`)

## Problem

`recur merge --stdin` fails when stdin JSON starts with UTF-8 BOM (`EF BB BF`), which is common on Windows/PowerShell generated streams.

## Symptom

```bash
type bom.json | recur merge --stdin --base "UserService.Game" --sep "."
# Error: Failed to parse JSON object 1 from stdin
```

## Scope

- BOM-tolerant JSON parsing for merge stdin mode.
- Preserve existing multi-object stdin JSON stream behavior.

## Code Targets

- `src/main_command_merge_impl.rs` (`execute_stdin_mode` JSON stream parse path)

