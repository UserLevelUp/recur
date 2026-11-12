<!-- recur-trace-v1
defines: main.command.trace_id.role_classification.verification
consumer: main.command.trace_id.role_classification.contract
consumer: main.command.trace_id.role_classification.baseline
-->
# Scoped `recur trace-id` role-classification verification observations

Windows x64, `C:/src/recur` (`--profile release-safe` CLI) verified against both
repository test suites and the origin project `C:/src/kicad_ra` (`b544c01`).

## Origin Dogfood Comparison (`C:/src/kicad_ra`)

| Query / Check | Red Baseline (`2ad068c`) | Verified Implementation |
| --- | --- | --- |
| `robo.electronics.carriers.handoff` role counts | `define=16, produce=27, consume=1940, trigger=33` | `define=7, produce=6, consume=9, trigger=0` |
| `robo.electronics.carriers.handoff` `.pyc` sites | `27` | `0` |
| `robo.electronics.carriers.handoff` prose/kwarg false `consume` hits | `1,931` (`consumes=[...]` symbol explosion) | `0` |
| `robo.electronics.carriers.board_register` role counts | `define=32, produce=8, consume=10, trigger=35` | `define=20, produce=8, consume=8, trigger=3` |
| `robo.electronics.carriers.board_register` `.pyc` sites | `4` | `0` |
| `robo.electronics.carriers.board_register` `"board_register"` false triggers | `35` | `0` (3 remaining triggers contain sibling `.register` ID) |
| Pure `consume`/`produce`/`trigger` sites duplicated into `define` | `9` (`handoff`), `32+` (`board_register`) | `0` (overlaps occur only on explicit `defines` markers) |
| Missing `--scope` stderr (`--depth 1`) | `require a legacy --scope query` | `require --scope <SCOPE> (for example: --scope "**")` |
| `--format full` site presentation | Duplicate unlabeled `(path, line_number)` lines | Deduplicated per `(path, line_number)` with `[define]`, `[consume]`, `[produce, trigger]` labels |

## Backwards-Compatibility & Test Suite Summary

- `cargo test --locked`: **347 passed, 0 failed**, with 7 pre-existing ignored doctests reported separately.
- `julia julia-tests/runtests.trace-id.jl`: **88 passed, 0 failed**, with 3 pre-existing `@test_skip` placeholders (`trace -> merge`, `callers -> merge`, `callees -> merge`) reported separately.
- The checked external result artifact covers the **4** new integration tests in `tests/trace_id_role_classification.rs` and **1,115** executed assertions across the seven zero-skip Julia suites below (**1,119** total, `0` failed, `0` skipped):

| Suite | Passed |
| --- | ---: |
| `tests/trace_id_role_classification.rs` | 4 |
| `julia-tests/main.improvement.34.trace.test.jl` | 43 |
| `julia-tests/main.improvement.34.trace-context.test.jl` | 122 |
| `julia-tests/main.improvement.34.baseline.test.jl` | 13 |
| `julia-tests/runtests.lane.coordination.jl` | 28 |
| `julia-tests/runtests.demo.skippy.jl` | 35 |
| `julia-tests/runtests.demo.sudoku.jl` | 52 |
| `julia-tests/main.lang.test.jl` | 822 |
