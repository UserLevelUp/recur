# merge File Mode Windows BOM Bug

Status: `complete` (historical todo record; see `docs/main.command.merge.file.mode.windows.bom.complete.md`)

## Problem

`recur merge` succeeded with `--stdin` but failed for file-mode JSON inputs on Windows when the JSON file had a UTF-8 BOM prefix (common from PowerShell output encodings).

## Scope

- Make file-mode JSON loading BOM-tolerant.
- Keep stdin mode behavior unchanged.
- Add regression coverage for BOM-prefixed JSON input files.

## Files

- `src/main_command_merge_impl.rs`
