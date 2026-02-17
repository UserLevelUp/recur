# Command: trace-stats Metrics Pipeline

Status: `todo.current` (active)
Date: 2026-02-16

## Goal

Implement the Phase 3 metrics engine behind `recur trace-stats`:

- direct callees
- transitive callees
- circular pattern count
- max depth
- risk classification

## Scope

1. Discover analyzable functions within scope/ext/stdin filters.
2. Reuse trace traversal to compute per-function stats.
3. Add sorting/filtering/top-N on computed metrics.
4. Replace bootstrap placeholder outputs with real table/json/csv data.
5. Expand Julia tests from contract-only to metric assertions.

## Progress

Completed:

- Function discovery from scoped files and extension filters.
- Per-function stats via trace traversal (direct, transitive, circular, depth, risk).
- Sorting (`transitive|direct|circular|depth|risk`) and filtering (`circular-only|high-risk|medium-risk|low-risk`).
- Top-N limiting.
- Real table/json/csv output with summary data.
- Traversal guardrail policy integration:
  - shared trait resolver (`TraversalBudgetCapable` + policy resolution)
  - config fallback from `.recur/config.toml`:
    - preferred `[traits.traversal_budget]`
    - compatibility fallback `[traversal]`
  - `trace-stats` CLI overrides: `--depth`, `--depth-guard`, `--force`
- Activated Julia metric assertions for:
  - non-empty stats on hierarchical dot-scoped fixtures
  - sort behavior
  - circular-only filter behavior
  - low-risk filter behavior
  - top-N behavior
  - depth guardrail behavior (hard-fail, clamp, force bypass)

Remaining:

- Upgrade circular metric from cycle-node count to distinct cycle-pattern count.
- Activate depth/risk ordering test assertions currently still skipped in `julia-tests/runtests.trace-stats.jl`.
- Add stdin-focused trace-stats integration assertions.
- Add larger-scope performance regression test.

## Validation Snapshot (2026-02-16)

Command:

```bash
julia julia-tests/main.command.trace-stats.test.jl
```

Observed status:

- `69 pass`
- `7 broken` (intentional placeholders still marked with `@test_skip`)

Broken/placeholder areas map directly to remaining work:

- sort by risk
- stdin integration lane
- distinct cycle-pattern counting and false-positive coverage
- medium/high risk fixture checks
- large codebase performance fixture

Recently activated:

- `sort-by depth` ordering assertion is now active and passing in `julia-tests/runtests.trace-stats.jl`.

## References

- `README.CORE.IMPROVEMENT7.md`
- `src/main_command_trace_stats_impl.rs`
- `src/search.rs`
- `src/output.rs`
- `julia-tests/runtests.trace-stats.jl`

## Discovery

```bash
recur files "main.command.trace-stats.**" -d docs/
recur tree "main.command.trace-stats" -d docs/
```
