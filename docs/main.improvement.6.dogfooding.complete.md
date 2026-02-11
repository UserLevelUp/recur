# Improvement 6 - Dogfooding Complete

**Date:** 2026-02-11

## Phases Delivered
- Phase 2: Command extraction — all 10 commands in `src/main_command_*_impl.rs`
- Phase 3: Stdin support — all 10 commands support `--stdin` with passing tests

## Test Results
- 379 pass, 4 fail (auto-JSON-on-pipe), 21 broken (pre-existing)
- Remaining 4 failures are Category B (auto-JSON-on-pipe design decision, not regressions)

## Key Fix
- Removed standalone `setup_test_environment()` / `teardown_test_environment()` from `runtests.merge.jl` that was destroying the test environment mid-suite
