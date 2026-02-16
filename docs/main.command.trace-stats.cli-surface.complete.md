# Command: trace-stats CLI Surface Complete

Status: `complete`
Date: 2026-02-16

## Completed

Phase 3 Step 1 (CLI surface + validation) is complete:

- Added `recur trace-stats` command and dispatch wiring.
- Added dedicated handler module: `src/main_command_trace_stats_impl.rs`.
- Added validation for:
  - `--sort-by` (`transitive|direct|circular|depth|risk`)
  - `--filter` (`circular-only|high-risk|medium-risk|low-risk`)
  - `--format` (`table|csv|json`)
  - `--top` (`> 0`)
  - non-empty `--scope`
- Added deterministic bootstrap outputs for table/csv/json while metrics pipeline is still pending.
- Activated Julia contract tests for help + validation behavior.

## Evidence

- `cargo test --bin recur`
- `recur trace-stats --help`
- `recur trace-stats --scope "**"`
- `recur trace-stats --scope "**" --sort-by latency` (validation error)
- `julia --project=julia-tests julia-tests/runtests.trace-stats.jl`

## Files

- `src/main.rs`
- `src/main_command_trace_stats_impl.rs`
- `julia-tests/runtests.trace-stats.jl`
- `julia-tests/runtests.jl`
