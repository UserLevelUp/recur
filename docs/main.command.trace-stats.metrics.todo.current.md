# Command: trace-stats Metrics Pipeline

Status: `todo.current` (active)
Date: 2026-03-06

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
  - risk ordering behavior (`--sort-by risk`)
  - circular-only filter behavior
  - low-risk filter behavior
  - top-N behavior
  - depth guardrail behavior (hard-fail, clamp, force bypass)

Remaining:

- Upgrade circular metric from cycle-node count to distinct cycle-pattern count.
- Add stdin-focused trace-stats integration assertions.
- Add larger-scope performance regression test.
- Add medium/high risk fixture assertions.

## Historical Validation Snapshot (2026-02-16)

Command:

```bash
julia julia-tests/main.command.trace-stats.test.jl
```

Observed status:

- `69 pass`
- `7 broken` (intentional placeholders still marked with `@test_skip`)

Broken/placeholder areas map directly to remaining work:

- stdin integration lane
- distinct cycle-pattern counting and false-positive coverage
- medium/high risk fixture checks
- large codebase performance fixture

Recently activated:

- `sort-by depth` ordering assertion is now active and passing in `julia-tests/runtests.trace-stats.jl`.
- `sort-by risk` ordering assertion is now active and passing in `julia-tests/runtests.trace-stats.jl`.

## Validation Snapshot (2026-03-06)

Command:

```bash
julia julia-tests/main.command.trace-stats.test.jl
```

Last known status (2026-03-01): `74 pass`, `6 broken`

### Precise @test_skip Map (julia-tests/runtests.trace-stats.jl)

| Line | Testset | What's needed |
|------|---------|---------------|
| 369  | stdin — trace-stats on changed files | Activate `run_recur_with_stdin`; add stdin fixture files + assert |
| 384  | circular — count distinct patterns | Fixture: A→B→A + A→C→A = 2 patterns; upgrade circular metric in src |
| 393  | circular — no false positives | Fixture: linear chain; assert circular == 0 |
| 424  | medium risk (10-30 transitive) | Add deep fixture function with 10-30 reachable callees |
| 432  | high risk (>30 transitive) | Add very deep fixture function with >30 reachable callees |
| 484  | large codebase performance | Seed 100+ functions; assert completion within time budget |

### Implementation Order (recommended)

1. **stdin** — no Rust changes needed; just activate the commented test body
2. **medium/high risk fixtures** — add fixture functions to test seed; no Rust changes
3. **circular no-false-positive** — add fixture + assert; likely no Rust changes
4. **circular distinct patterns** — requires Rust change to `src/main_command_trace_stats_impl.rs`
5. **performance** — last; seed large fixture; assert timing

### Phase 3 Close-out Criteria

1. All 6 `@test_skip` replaced with passing assertions.
2. `julia julia-tests/main.command.trace-stats.test.jl` shows 0 broken.
3. Create `docs/main.improvement.7.phase3.complete.md`.
4. Delete this `.current.md` and `docs/main.improvement.7.phase3.todo.current.md`.

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
