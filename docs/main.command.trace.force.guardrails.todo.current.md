# Command: trace Force Guardrails

Status: `todo.current` (implemented and verified)
Date: 2026-02-16

## Problem

`recur trace` has structural safety controls, but no explicit force override:

- hard request cap: depth must be `<= 5`
- traversal stops for depth/width/cycle/unresolved cases
- no wall-clock timeout/deadline control yet

This is safe by default, but blocks intentional deep diagnostics where a user wants to bypass the depth cap explicitly.

## Current Behavior (verified)

- `src/main_command_trace_impl.rs`: `depth > 5` returns an error immediately.
- `src/search.rs`: traversal tracks depth limit, width limit, and cycles.
- `src/output.rs`: stop reasons are surfaced in tree/flat/json summaries.
- No timeout/deadline logic was found in trace or shared search paths.

## Reuse Strategy

Use the same override model already present in `recur init`:

- `src/main.rs` adds a command-level `--force` boolean
- command impl receives the flag and gates safety checks with `if !force { ... }`
- default behavior remains unchanged for users who do not pass `--force`

## Patch Scope (Code-First)

1. Add `--force` to `Commands::Trace` in `src/main.rs`.
2. Thread `force` into `main_command_trace_impl::execute`.
3. Change depth hard-cap check to allow `depth > 5` only when `--force` is set.
4. Add/activate tests:
   - depth `> 5` fails without `--force`
   - depth `> 5` succeeds with `--force`
5. Keep width/cycle protections active even in force mode.

## Progress (TDD First)

- Activated force tests in `julia-tests/runtests.trace.jl`:
  - `trace --force bypasses depth cap`
  - `trace --force keeps max-width guardrail`
- Current observed behavior before code patch:
  - `trace --depth 10` fails with `Maximum depth is 5` (expected baseline)
  - `trace --force ...` fails with `unexpected argument '--force' found` (missing CLI wiring)
- Initial host issue was related to Julia package/runtime setup; after running `julia-tests/setup.packages.jl` and rebuilding `target/release-safe/recur.exe`, trace suite execution succeeded.

## Progress (Implementation)

- Added `--force` flag to `trace` CLI in `src/main.rs`.
- Threaded `force` through trace dispatch into `src/main_command_trace_impl.rs`.
- Updated depth guard to: `if depth > 5 && !force`.
- Replaced placeholder force tests in `julia-tests/runtests.trace.jl` with active assertions.

## Verification

- `cargo test` (all Rust suites): pass.
- Manual command checks on a temporary test hierarchy:
  - `recur trace ... --depth 10` -> fails with max-depth error (default safety intact)
  - `recur trace ... --depth 6 --force` -> succeeds (depth-cap bypass works)
  - `recur trace ... --depth 6 --max-width 1 --force` -> succeeds and reports max-width truncation (guardrail still active)
- `julia-tests/setup.packages.jl` confirms/install-checks `JSON3`.
- `julia --project=julia-tests julia-tests/runtests.trace.jl`: pass for new force tests after rebuilding `target/release-safe/recur.exe` (Julia harness defaults to `RECUR_PROFILE=release-safe` binary).

## Deferred (follow-up)

- `--timeout-ms` wall-clock stop control
- optional `--max-nodes` budget stop control

## Discovery

```bash
recur files "main.command.trace.force.**" -d docs/
recur tree "main.command.trace.force" -d docs/
```
