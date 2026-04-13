# Command Tests: Expectation Drift Recurring

Purpose: remember the recurring failure pattern where Julia tests look broken
because the harness forces non-terminal JSON output or demo fixtures drift, not
because the underlying command regressed.

## When This Applies

- a focused command lane passes in isolation
- the full suite reports failures that smell like output-shape mismatch or
  stale fixture assumptions
- stdout is captured through `IOBuffer` or pipeline helpers
- commands with non-terminal auto-JSON behavior are involved

## Working Rule

1. Isolate the command lane first.
2. Check whether the harness is running in non-terminal pipeline mode.
3. Parse machine JSON before asserting on line-oriented text.
4. Treat checked-in fixture files as canonical until intentionally regenerated.
5. Only escalate to compiler/runtime suspicion after the stale-assertion path is
   ruled out.

## Trace-id Hook

```text
tests.pattern.expectation.drift = recurring
tests.pattern.expectation.drift publish tests.signal.pipeline.machine.json
tests.pattern.expectation.drift publish tests.signal.fixture.truth.drift
tests.pattern.expectation.drift publish tests.signal.isolate.before.blame
tests.signal.pipeline.machine.json subscribe tests.pattern.expectation.drift
tests.signal.fixture.truth.drift subscribe tests.pattern.expectation.drift
tests.signal.isolate.before.blame subscribe tests.pattern.expectation.drift
tests.pattern.expectation.drift trigger tests.action.rerun.full.suite
tests.pattern.expectation.drift trigger tests.action.treat.fixture.as.truth
```

## Why These Signals Matter

- `pipeline.machine.json`: captured stdout can flip `files` or `tree` into JSON
  mode
- `fixture.truth.drift`: demo assets can change while older assertions keep the
  previous values
- `isolate.before.blame`: the command that fails last is not automatically the
  command that caused the failure

## Recur Queries

```powershell
recur trace-id "tests.pattern.expectation.drift" --scope "main.command.tests.**" --ext .md --json -d docs/
recur find "machine JSON" --scope "main.command.tests.**" -d docs/ -i
recur files "main.command.tests.**" -d docs/
```

## Observed Example (2026-04-09)

- `julia-tests/runtests.stdin.jl` drifted because pipeline-mode `files` output
  was `[]` rather than blank text
- `julia-tests/runtests.tree.jl` drifted because `tree --count` emitted machine
  JSON in the harness
- `julia-tests/runtests.demo.sudoku.phase4.jl` drifted because `easy-001` now
  sets `sudoku.r3.c5 = 4`
- `julia-tests/main.command.trace-stats.test.jl` was clean; isolating it
  prevented a false blame trail

## Related

- `docs/main.command.tests.progress.current.md`
- `julia-tests/runtests.stdin.jl`
- `julia-tests/runtests.tree.jl`
- `julia-tests/runtests.demo.sudoku.phase4.jl`
