# Command: flatten JSON Windows BOM Compatibility Complete

Status: `complete`
Date: 2026-02-16

## Completed

- Added UTF-8 BOM normalization in flatten JSON parse path.
- Added regression test for BOM-prefixed JSON flatten parsing.

## Evidence

- `cargo test --bin recur json_with_utf8_bom_parses`
- `cargo run --quiet --bin recur -- flatten <bom-json-file> --format json --json`
- `cmd /c "type <bom-json-file> | cargo run --quiet --bin recur -- flatten --stdin --format json --json"`

## Code

- `src/main_command_flatten_json.rs`
- `src/main_command_flatten_impl.rs`
