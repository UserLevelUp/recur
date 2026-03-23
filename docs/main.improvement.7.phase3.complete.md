# Improvement 7 Phase 3: trace-stats

Status: `complete`
Date: 2026-03-08

## What Landed

Full `recur trace-stats` metrics pipeline — all 6 `@test_skip` placeholders activated and passing.

### Metrics Implemented

- `direct` — direct callees count per function
- `transitive` — reachable callee count (full traversal)
- `circular` — distinct back-edge pattern count (not just presence)
- `depth` — max traversal depth
- `risk` — classification: low / medium / high

### Features

- Sorting: `--sort-by transitive|direct|circular|depth|risk`
- Filtering: `--circular-only`, `--high-risk`, `--medium-risk`, `--low-risk`
- Top-N limiting: `--top N`
- Output: table / JSON / CSV
- Stdin integration: pipe file list via `--stdin`
- Traversal budget: `[traits.traversal_budget]` in `.recur/config.toml`
- CLI overrides: `--depth`, `--depth-guard`, `--force`

### Test Baseline at Close

```
recur trace-stats command | 94 passed, 0 broken, 0 failed
```

All 6 previously `@test_skip` lanes now active and passing:
1. stdin — trace-stats on changed files
2. circular — count distinct patterns (DistinctCycleService fixture, circular=2)
3. circular — no false positives
4. medium risk fixtures (MediumService: 10-30 transitive)
5. high risk fixtures (HighService: >30 transitive)
6. large codebase performance (PerformanceService, 100+ functions, <1s)

### Fix: release-safe Stack Overflow

`opt-level = 0` in `[profile.release-safe]` caused stack overflow on recursive traversal.
Fixed to `opt-level = 1` — restores unwind panic semantics without LTO.

## References

- `src/main_command_trace_stats_impl.rs` — implementation
- `julia-tests/runtests.trace-stats.jl` — full test suite
- `julia-tests/runtests.setup.jl` — fixtures (DistinctCycleService, MediumService, HighService, PerformanceService)
- `docs/main.version.a.0.2.8.complete.md` — version record
- `README.CORE.IMPROVEMENT7.md` — original spec
