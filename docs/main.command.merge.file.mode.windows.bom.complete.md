# merge File Mode Windows BOM Bug Complete

Status: `complete`
Date: 2026-02-16

## Completed

- Added BOM normalization before JSON parsing in merge file mode (`load_files_from_json_file`).
- Added regression test for BOM-prefixed JSON file input.

## Evidence

- `cargo test --bin recur load_files_from_json_file_accepts_utf8_bom`
- `cargo run --quiet --bin recur -- merge <temp-bom-json> --base "UserService.Game" --sep "."`

## Files

- `src/main_command_merge_impl.rs`
- `docs/main.command.merge.file.mode.windows.bom.todo.md`
