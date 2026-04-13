# Command Tests Progress Snapshot

Status: `current`
Date: 2026-04-09

## Current Suite Truth

- full Julia suite: `809 passed, 49 broken, 0 failed`
- `trace-stats`: `94 passed, 0 failed` in isolation and inside the full suite
- `trace-stats` is not the source of the earlier suite panic
- the earlier live failures were stale Julia expectations plus one Sudoku fixture
  drift

## What Actually Broke

- non-terminal Julia harness runs pipe stdout through `IOBuffer`, so `files` and
  `tree` auto-switched to machine JSON output
- older stdin/tree tests were still asserting terminal-text behavior:
  blank output for no matches, line-text comparison, or a count footer
- Sudoku Phase 4 still assumed `sudoku.r3.c5 = 7`, while the checked-in
  `easy-001` fixture says `sudoku.r3.c5 = 4`
- one transient Julia `EXCEPTION_ACCESS_VIOLATION` occurred around the full
  suite, but it did not reproduce after the stale assertions were corrected

## Fixes Landed On 2026-04-09

- `julia-tests/runtests.stdin.jl`
  - empty/no-match stdin cases now parse `[]` as valid machine output
  - filesystem-vs-stdin comparison now parses JSON arrays instead of raw lines
- `julia-tests/runtests.tree.jl`
  - `tree --count` in pipeline mode now validates machine JSON output instead
    of a terminal-only footer
- `julia-tests/runtests.demo.sudoku.phase4.jl`
  - assertions now match the checked-in `easy-001` truth:
    `sudoku.r3.c5 = 4`
- `julia-tests/runtests.demo.sudoku.phase3.jl`
  - saved-run coverage now proves per-cell `trace-id` run persistence and reuse
    for the Sudoku generator path
- `demos/sudoku/julia/Recur.jl` and `demos/sudoku/julia/Generator.jl`
  - Sudoku now uses stable per-cell `--save-run` + `--reuse-if-fresh` flow
    without rewriting unchanged files and breaking freshness

## Recurring Pattern

This was not a command-lane regression. It was a test-harness
expectation-drift pattern:

- isolate the failing lane first
- check whether the harness is capturing stdout in non-terminal mode
- parse machine JSON before asserting on line-oriented text
- treat checked-in demo fixtures as source truth before changing engine code
- rerun the full suite after stale-expectation cleanup before blaming the last
  command touched

Durable rediscovery note:

- `docs/main.command.tests.expectation-drift.recurring.md`

Trace it directly:

```powershell
recur trace-id "tests.pattern.expectation.drift" --scope "main.command.tests.**" --ext .md --json -d docs/
```

## Repro Commands

```powershell
julia julia-tests/main.command.trace-stats.test.jl
julia julia-tests/runtests.jl
recur trace-id "tests.pattern.expectation.drift" --scope "main.command.tests.**" --ext .md --json -d docs/
```
