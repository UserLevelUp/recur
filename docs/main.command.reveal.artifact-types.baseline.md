# Artifact-types baseline evidence

Observed 2026-09-07, before classifier implementation. Repository baseline:
`10d20153c79a631adceb7e6dabff9ef9d2977f70`. The existing artifact-types planning
files were untracked at entry; no pre-existing tracked source edits were present.

Built the baseline with `cargo build --locked --profile release-safe --bins`.
The existing flatten `EntryKind::Element` dead-code warning was present.

Executed against the baseline release-safe binary:

```powershell
julia --startup-file=no -C generic -O0 julia-tests/main.command.reveal.test.jl
julia --startup-file=no -C generic -O0 julia-tests/main.command.init.test.jl
julia --startup-file=no -C generic -O0 julia-tests/main.command.reveal.artifact-types.test.jl
```

- Legacy reveal: 31 passed, no failures.
- Legacy init: 33 passed, no failures.
- Initial standalone artifact-types suite: 28 passed, 37 failed, 0 errors,
  0 broken. The failures exposed missing classification/filtering/diagnostics,
  missing config validation and broadening of explicit discovery roots.

The new suite remained outside the full runner for this red observation. It was
subsequently expanded and integrated only after the implementation passed it.
These initial counts describe the initial suite, not every later acceptance case.

The [frozen contract](main.command.reveal.artifact-types.contract.md) defines
metadata/prefix precedence, JSON fields, selection tiers and directory bounds.
The [acceptance matrix](main.command.reveal.artifact-types.tests.md) retains the
required coverage. These records establish slice-0 only; implementation and final
regressions require the separate verification record.

produces: recur.reveal.artifact-types.baseline observed red contract and legacy baseline
