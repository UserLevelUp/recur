# Command: merge stdin Windows BOM Compatibility Complete

Status: `complete`
Date: 2026-02-16

## Completed

- Added UTF-8 BOM normalization for stdin JSON stream parsing in merge.
- Reused BOM helper in both stdin/file JSON parse paths.
- Added regression test for BOM-prefixed stdin JSON stream.

## Evidence

- `cargo test --bin recur stdin_json_stream_accepts_utf8_bom`
- `cmd /c "type <bom-json-array-file> | cargo run --quiet --bin recur -- merge --stdin --base UserService.Game --sep ."`

## Code

- `src/main_command_merge_impl.rs`
