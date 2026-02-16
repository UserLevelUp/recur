# Command: trace Force Guardrails Complete

Status: `complete`
Date: 2026-02-16

## Completed

- Added `--force` to `recur trace` command surface.
- Preserved default depth guard (`depth > 5`) unless `--force` is explicitly set.
- Kept traversal guardrails active in force mode (max-width, cycle handling, stop-reason reporting).
- Replaced placeholder force tests with active test coverage.

## Evidence

- `cargo test`
- `julia --project=julia-tests julia-tests/runtests.trace.jl`
- `recur trace --help` (includes `--force`)
- `recur trace ... --depth 10` fails without `--force`
- `recur trace ... --depth 6 --force` succeeds

## Files

- `src/main.rs`
- `src/main_command_trace_impl.rs`
- `julia-tests/runtests.trace.jl`
