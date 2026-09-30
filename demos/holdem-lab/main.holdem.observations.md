# Observations: 2026-09-29

Environment: Windows, Julia 1.12.7, installed Recur 0.2.8 built locally on the
`recur-lang` branch. `main.holdem.observations.json` fingerprints the experiment
inputs and executable. This is an experiment record, not a checked Warp receipt,
proof of input-dependency closure, or a claim of full release readiness.

## Authoring order observed in this session

Each numbered specification and corresponding tests preceded its implementation.
Initial Julia runs failed the explicit missing-implementation assertion. Then:

| Stage | Static result before implementation | Subsequent runtime assertions |
| --- | --- | --- |
| 01 | sound within WIR1 coverage | 3 pass |
| 02 | sound within WIR1 coverage | 6 pass |
| 03 | sound within WIR1 coverage | 14 pass |
| 04 | sound within WIR1 coverage | 36 pass |
| 05 | RLIR011 output alias rejected; corrected spec passed before implementation | 404 pass |
| 06 | RCIR006 multiline flow rejected; corrected spec passed before implementation | 10 pass |

The graph/lineage suite initially had two test-harness errors: it expected
expanded `messages` in the compact packet, and invoked a newly loaded Julia
module from an older world age. Correcting the packet expectation and using
`invokelatest` produced 54 passing assertions. No graph-engine change was needed.

Combined focused result: **527 pass, zero failures**, exit 0, 3.8 seconds.
All four intentional graph mutations were detected. The hidden runtime cycle
was caught by the bounded Julia probe despite a green Lang check. The scripted
app also ran successfully: player 2 won, final stacks were `[94, 106]`, and the
independent coordinator assigned the 12-chip pot to player 2.

## Reproducibility problem retained

The wider checks could not establish a clean full regression:

| Attempt | Observation |
| --- | --- |
| Full Julia suite, normal compilation | Access violation in compiler while loading existing `main.command.warp.bubble.test.jl` |
| Full Julia suite, `--compile=min` | Reached existing Sudoku teaching tests, then interpreter access violation |
| Exhaustive five-card histogram | Normal compilation and `-O0 --inline=no` attempts both terminated in Julia internals |
| Focused normal rerun | 527 pass and exit 0, but emitted an internal compiler BoundsError while formatting the test summary |
| Focused `--compile=min` rerun | Interpreter access violation in stage-04 test evaluation |

No later application assertion failure was reported in these faulting attempts;
process faults are still failed validation attempts. The cause has not been
diagnosed. The clean earlier focused pass is historical evidence, not a claim
that reruns are currently reliable. Julia was not upgraded and no machine-wide
settings were changed. Generated crash-run `test_environment` directories were
preserved outside the repository instead of deleting evidence.

Local diagnostic logs (outside the source tree):

```text
C:/Users/marcn/Documents/Codex/2026-09-29/how/work/recur-validation/
  holdem-full-julia.log
  holdem-full-julia-min.log
  holdem-exhaustive.log
  holdem-focused.log
  holdem-focused-min.log
```

## Commands used

```powershell
$env:RECUR_BIN = 'C:/Users/marcn/.cargo/bin/recur.exe'
$env:RECUR_PROFILE = 'release-validation-1.85/release-safe'
julia --startup-file=no -C generic --project=demos/web-evidence-lab demos/holdem-lab/main.holdem.test.jl
julia --startup-file=no -C generic demos/holdem-lab/main.holdem.app.jl
julia --startup-file=no -C generic julia-tests/runtests.jl
julia --startup-file=no -C generic --compile=min julia-tests/runtests.jl
julia --startup-file=no -C generic demos/holdem-lab/main.holdem.exhaustive.jl
julia --startup-file=no -C generic -O0 --inline=no demos/holdem-lab/main.holdem.exhaustive.jl
```

The complete regression runner includes the new experiment. Cargo was not rerun:
this change adds Julia/demo/docs files without changing Rust source or dependencies.
No commits, pushes, installation or release publication were performed for this
experiment. The six valid Lang sources remain statically sound within coverage;
no runtime evidence was added to the existing browser inspector.

consumes: demo.holdem.graph graph diagnostics and explicit coverage limit
consumes: demo.holdem.play qualified runtime observations
